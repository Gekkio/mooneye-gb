use std::fmt;
use std::io::timer::Timer;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError, sync_channel};
use std::time::duration::Duration;

use backend::{
  BackendSharedMemory, BackendMessage
};
use config::HardwareConfig;
use cpu::Cpu;
use gameboy;
use hardware::Clock;
use hardware::Hardware;
use self::perf_counter::PerfCounter;

mod perf_counter;
mod pulse;

pub struct Machine<'a> {
  cpu: Cpu<Hardware<'a>>,
  perf_counter: PerfCounter,
  mode: EmulationMode,
  clock: Clock
}

pub struct Channels {
  to_backend: SyncSender<MachineMessage>,
  from_backend: Receiver<BackendMessage>
}

pub enum MachineMessage {
  RelativeSpeedStat(f64)
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

impl<'a> fmt::Show for Machine<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.cpu)
  }
}

/// Amount of cycles in each emulation pulse
const PULSE_CYCLES: u32 = 10484; // 10ms worth of cycles

impl<'a> Machine<'a> {
  pub fn new(backend: &'a BackendSharedMemory, config: HardwareConfig) -> Machine<'a> {
    Machine {
      cpu: Cpu::new(Hardware::new(backend, config)),
      perf_counter: PerfCounter::new(),
      mode: EmulationMode::Normal,
      clock: Clock::new()
    }
  }
  fn emulate(&mut self) -> bool {
    let end_clock = self.clock + PULSE_CYCLES;

    self.cpu.execute_until(end_clock);

    self.perf_counter.update(end_clock.as_clock_cycles() - self.clock.as_clock_cycles());
    self.clock = end_clock;

    if self.clock.needs_normalize() {
      self.clock.normalize();
      self.cpu.normalize_clock();
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
    println!("${:04x}: {:18} {}", pc, op, self);
  }
  pub fn main_benchmark(&mut self, channels: Channels, duration: Duration) {
    let from_backend = channels.from_backend;
    self.reset();

    let mut timer = Timer::new().unwrap();
    let limit = timer.oneshot(duration);

    loop {
      match limit.try_recv() {
        Ok(_) => {
          println!("{}", self.clock.as_clock_cycles());
          break;
        },
        _ => ()
      }
      match from_backend.try_recv() {
        Err(TryRecvError::Disconnected) => break,
        _ => ()
      }
      self.emulate();
    }
  }
  pub fn main_loop(&mut self, channels: Channels) {
    let from_backend = channels.from_backend;
    let to_backend = channels.to_backend;
    self.reset();

    let pulse_duration = Duration::seconds(4 * PULSE_CYCLES as i64) / gameboy::CPU_SPEED_HZ as i32;

    let mut perf_timer = Timer::new().unwrap();
    let perf_update = perf_timer.periodic(Duration::milliseconds(100));

    loop {
      match self.mode {
        EmulationMode::Normal => {
          let pulse = pulse::start(pulse_duration);

          loop {
            select!(
              backend_event = from_backend.recv() => {
                match backend_event {
                  Err(_) => return,
                  Ok(BackendMessage::KeyDown(key)) => self.cpu.hardware().key_down(key),
                  Ok(BackendMessage::KeyUp(key)) => self.cpu.hardware().key_up(key),
                  Ok(BackendMessage::Turbo(true)) => { self.mode = EmulationMode::MaxSpeed; break },
                  Ok(BackendMessage::Break) => { self.mode = EmulationMode::Debug; break },
                  _ => ()
                }
              },
              _ = perf_update.recv() => {
                let value = self.perf_counter.get_relative_speed();
                to_backend.send(MachineMessage::RelativeSpeedStat(value)).unwrap();
              },
              _ = pulse.recv() => {
                if !self.emulate() {
                  break;
                }
              }
            )
          }
        },
        EmulationMode::MaxSpeed => {
          loop {
            match from_backend.try_recv() {
              Err(TryRecvError::Disconnected) => return,
              Ok(BackendMessage::KeyDown(key)) => self.cpu.hardware().key_down(key),
              Ok(BackendMessage::KeyUp(key)) => self.cpu.hardware().key_up(key),
              Ok(BackendMessage::Turbo(false)) => { self.mode = EmulationMode::Normal; break },
              Ok(BackendMessage::Break) => { self.mode = EmulationMode::Debug; break },
              _ => ()
            }
            match perf_update.try_recv() {
              Err(TryRecvError::Disconnected) => return,
              Ok(_) => {
                let value = self.perf_counter.get_relative_speed();
                to_backend.send(MachineMessage::RelativeSpeedStat(value)).unwrap();
              },
              _ => ()
            }
            if !self.emulate() {
              break;
            }
          }
        },
        EmulationMode::Debug => {
          self.debug_status();
          loop {
            match from_backend.recv() {
              Err(_) => return,
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

enum EmulationMode {
  Debug, Normal, MaxSpeed
}

pub fn new_channel() -> (SyncSender<MachineMessage>, Receiver<MachineMessage>) {
  sync_channel(128)
}
