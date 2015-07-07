use clock_ticks::precise_time_ns;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError, sync_channel};
use time::Duration;

use backend::{
  BackendSharedMemory, BackendMessage
};
use config::HardwareConfig;
use cpu::Cpu;
use cpu::registers::Registers;
use emulation::{EmuTime, MachineCycles};
use hardware::Hardware;
use self::perf_counter::PerfCounter;

mod perf_counter;
mod pulse;

pub struct Machine<'a> {
  cpu: Cpu<Hardware<'a>>,
  perf_counter: PerfCounter,
  mode: EmulationMode,
  time: EmuTime
}

pub struct Channels {
  to_backend: SyncSender<MachineMessage>,
  from_backend: Receiver<BackendMessage>
}

pub enum MachineMessage {
  RelativeSpeedStat(f64), Debug(Registers)
}

impl Channels {
  pub fn new(to_backend: SyncSender<MachineMessage>,
             from_backend: Receiver<BackendMessage>) -> Channels {
    Channels {
      to_backend: to_backend,
      from_backend: from_backend
    }
  }
}

/// Amount of cycles in each emulation pulse
const PULSE_CYCLES: MachineCycles = MachineCycles(10484); // 10ms worth of cycles

impl<'a> Machine<'a> {
  pub fn new(backend: &'a BackendSharedMemory, config: HardwareConfig) -> Machine<'a> {
    Machine {
      cpu: Cpu::new(Hardware::new(backend, config)),
      perf_counter: PerfCounter::new(),
      mode: EmulationMode::Normal,
      time: EmuTime::zero()
    }
  }
  pub fn set_mode(&mut self, mode: EmulationMode) {
    self.mode = mode;
  }
  fn emulate(&mut self, to_backend: &SyncSender<MachineMessage>) -> bool {
    let end_time = self.time + PULSE_CYCLES;

    self.cpu.execute_until(end_time);

    self.perf_counter.update(end_time - self.time);
    self.time = end_time;

    if self.time.needs_rewind() {
      self.time.rewind();
      self.cpu.rewind_time();
    }
    if let Some(regs) = self.cpu.ack_debug() {
      to_backend.send(MachineMessage::Debug(regs)).unwrap();
    }
    true
  }
  fn debug_step(&mut self) {
    self.cpu.execute();
    self.debug_status();
  }
  fn debug_status(&mut self) {
    let pc = self.cpu.get_pc();
    let op = self.cpu.disasm_op();
    println!("${:04x}: {:18} {}", pc, op, self.cpu);
  }
  pub fn main_benchmark(&mut self, channels: Channels, duration: Duration) {
    let from_backend = channels.from_backend;
    let to_backend = channels.to_backend;
    self.reset();

    let start_time = precise_time_ns();

    loop {
      let time = precise_time_ns();
      if Duration::nanoseconds((time - start_time) as i64) > duration {
          println!("{}", self.time.cycles().as_clock_cycles());
        break;
      }
      match from_backend.try_recv() {
        Err(TryRecvError::Disconnected) => break,
        _ => ()
      }
      self.emulate(&to_backend);
    }
  }
  pub fn main_loop(&mut self, channels: Channels) {
    let from_backend = channels.from_backend;
    let to_backend = channels.to_backend;
    self.reset();

    let pulse_duration = PULSE_CYCLES.as_duration();

    let mut last_perf_update = precise_time_ns();
    let perf_update_freq = Duration::milliseconds(100);

    loop {
      match self.mode {
        EmulationMode::Normal => {
          let pulse = pulse::start(pulse_duration);

          loop {
            let time = precise_time_ns();
            if Duration::nanoseconds((time - last_perf_update) as i64) > perf_update_freq {
              last_perf_update = time;
              let value = self.perf_counter.get_relative_speed();
              to_backend.send(MachineMessage::RelativeSpeedStat(value)).unwrap();
            }
            match from_backend.try_recv() {
              Err(TryRecvError::Empty) => (),
              Err(_) => return,
              Ok(BackendMessage::Quit) => return,
              Ok(BackendMessage::KeyDown(key)) => self.cpu.hardware().key_down(key),
              Ok(BackendMessage::KeyUp(key)) => self.cpu.hardware().key_up(key),
              Ok(BackendMessage::Turbo(true)) => { self.mode = EmulationMode::MaxSpeed; break },
              Ok(BackendMessage::Break) => { self.mode = EmulationMode::Debug; break },
              Ok(_) => ()
            }
            match pulse.recv() {
              Ok(_) => {
                if !self.emulate(&to_backend) {
                  break;
                }
              },
              _ => return
            }
          }
        },
        EmulationMode::MaxSpeed => {
          loop {
            let time = precise_time_ns();
            if Duration::nanoseconds((time - last_perf_update) as i64) > perf_update_freq {
              last_perf_update = time;
              let value = self.perf_counter.get_relative_speed();
              to_backend.send(MachineMessage::RelativeSpeedStat(value)).unwrap();
            }
            match from_backend.try_recv() {
              Err(TryRecvError::Disconnected) => return,
              Ok(BackendMessage::Quit) => return,
              Ok(BackendMessage::KeyDown(key)) => self.cpu.hardware().key_down(key),
              Ok(BackendMessage::KeyUp(key)) => self.cpu.hardware().key_up(key),
              Ok(BackendMessage::Turbo(false)) => { self.mode = EmulationMode::Normal; break },
              Ok(BackendMessage::Break) => { self.mode = EmulationMode::Debug; break },
              _ => ()
            }
            if !self.emulate(&to_backend) {
              break;
            }
          }
        },
        EmulationMode::Debug => {
          self.debug_status();
          loop {
            match from_backend.recv() {
              Err(_) => return,
              Ok(BackendMessage::Quit) => return,
              Ok(BackendMessage::KeyDown(key)) => self.cpu.hardware().key_down(key),
              Ok(BackendMessage::KeyUp(key)) => self.cpu.hardware().key_up(key),
              Ok(BackendMessage::Run) => { self.mode = EmulationMode::Normal; break },
              Ok(BackendMessage::Step) => { self.debug_step(); }
              _ => ()
            }
          }
        }
      }
    }
  }
  pub fn reset(&mut self) {
    self.cpu.hardware().bootrom.reset();
  }
}

pub enum EmulationMode {
  Debug, Normal, MaxSpeed
}

pub fn new_channel() -> (SyncSender<MachineMessage>, Receiver<MachineMessage>) {
  sync_channel(128)
}
