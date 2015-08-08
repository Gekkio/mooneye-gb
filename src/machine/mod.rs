// This file is part of Mooneye GB.
// Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
use clock_ticks::precise_time_ns;
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError, sync_channel};
use time::Duration;

use config::HardwareConfig;
use cpu::Cpu;
use cpu::registers::Registers;
use emulation::{EmuTime, MachineCycles};
use frontend::{
  FrontendSharedMemory, FrontendMessage
};
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
  to_frontend: SyncSender<MachineMessage>,
  from_frontend: Receiver<FrontendMessage>
}

pub enum MachineMessage {
  RelativeSpeedStat(f64), Debug(Registers)
}

impl Channels {
  pub fn new(to_frontend: SyncSender<MachineMessage>,
             from_frontend: Receiver<FrontendMessage>) -> Channels {
    Channels {
      to_frontend: to_frontend,
      from_frontend: from_frontend
    }
  }
}

/// Amount of cycles in each emulation pulse
const PULSE_CYCLES: MachineCycles = MachineCycles(10484); // 10ms worth of cycles

impl<'a> Machine<'a> {
  pub fn new(frontend: &'a FrontendSharedMemory, config: HardwareConfig) -> Machine<'a> {
    Machine {
      cpu: Cpu::new(Hardware::new(frontend, config)),
      perf_counter: PerfCounter::new(),
      mode: EmulationMode::Normal,
      time: EmuTime::zero()
    }
  }
  pub fn set_mode(&mut self, mode: EmulationMode) {
    self.mode = mode;
  }
  fn emulate(&mut self, to_frontend: &SyncSender<MachineMessage>) -> bool {
    let end_time = self.time + PULSE_CYCLES;

    self.cpu.execute_until(end_time);

    self.perf_counter.update(end_time - self.time);
    self.time = end_time;

    if self.time.needs_rewind() {
      self.time.rewind();
      self.cpu.rewind_time();
    }
    if let Some(regs) = self.cpu.ack_debug() {
      to_frontend.send(MachineMessage::Debug(regs)).unwrap();
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
    let from_frontend = channels.from_frontend;
    let to_frontend = channels.to_frontend;
    self.reset();

    let start_time = precise_time_ns();

    loop {
      let time = precise_time_ns();
      if Duration::nanoseconds((time - start_time) as i64) > duration {
          println!("{}", self.time.cycles().as_clock_cycles());
        break;
      }
      match from_frontend.try_recv() {
        Err(TryRecvError::Disconnected) => break,
        _ => ()
      }
      self.emulate(&to_frontend);
    }
  }
  pub fn main_loop(&mut self, channels: Channels) {
    let from_frontend = channels.from_frontend;
    let to_frontend = channels.to_frontend;
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
              to_frontend.send(MachineMessage::RelativeSpeedStat(value)).unwrap();
            }
            match from_frontend.try_recv() {
              Err(TryRecvError::Empty) => (),
              Err(_) => return,
              Ok(FrontendMessage::Quit) => return,
              Ok(FrontendMessage::KeyDown(key)) => self.cpu.hardware().key_down(key),
              Ok(FrontendMessage::KeyUp(key)) => self.cpu.hardware().key_up(key),
              Ok(FrontendMessage::Turbo(true)) => { self.mode = EmulationMode::MaxSpeed; break },
              Ok(FrontendMessage::Break) => { self.mode = EmulationMode::Debug; break },
              Ok(_) => ()
            }
            match pulse.recv() {
              Ok(_) => {
                if !self.emulate(&to_frontend) {
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
              to_frontend.send(MachineMessage::RelativeSpeedStat(value)).unwrap();
            }
            match from_frontend.try_recv() {
              Err(TryRecvError::Disconnected) => return,
              Ok(FrontendMessage::Quit) => return,
              Ok(FrontendMessage::KeyDown(key)) => self.cpu.hardware().key_down(key),
              Ok(FrontendMessage::KeyUp(key)) => self.cpu.hardware().key_up(key),
              Ok(FrontendMessage::Turbo(false)) => { self.mode = EmulationMode::Normal; break },
              Ok(FrontendMessage::Break) => { self.mode = EmulationMode::Debug; break },
              _ => ()
            }
            if !self.emulate(&to_frontend) {
              break;
            }
          }
        },
        EmulationMode::Debug => {
          self.debug_status();
          loop {
            match from_frontend.recv() {
              Err(_) => return,
              Ok(FrontendMessage::Quit) => return,
              Ok(FrontendMessage::KeyDown(key)) => self.cpu.hardware().key_down(key),
              Ok(FrontendMessage::KeyUp(key)) => self.cpu.hardware().key_up(key),
              Ok(FrontendMessage::Run) => { self.mode = EmulationMode::Normal; break },
              Ok(FrontendMessage::Step) => { self.debug_step(); }
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
