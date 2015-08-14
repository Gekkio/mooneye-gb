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
use config::HardwareConfig;
use cpu::Cpu;
use cpu::registers::Registers;
use emulation::{EmuTime, EmuEvents};
use frontend::{GbKey};
use gameboy;
use hardware::Hardware;
pub use self::perf_counter::PerfCounter;

mod perf_counter;

pub struct Machine {
  cpu: Cpu<Hardware>
}

impl Machine {
  pub fn new(config: HardwareConfig) -> Machine {
    Machine {
      cpu: Cpu::new(Hardware::new(config))
    }
  }
  pub fn emulate(&mut self, target_time: EmuTime) -> (EmuEvents, EmuTime) {
    loop {
      self.cpu.execute();
      if !self.cpu.hardware().emu_events().is_empty() || self.cpu.time() >= target_time {
        break;
      }
    }
    (self.cpu.hardware().ack_emu_events(), self.cpu.time())
  }
  pub fn key_down(&mut self, key: GbKey) {
    self.cpu.hardware().key_down(key);
  }
  pub fn key_up(&mut self, key: GbKey) {
    self.cpu.hardware().key_up(key);
  }
  pub fn regs(&self) -> Registers { self.cpu.regs }
  pub fn screen_buffer(&self) -> &gameboy::ScreenBuffer { self.cpu.hardware.screen_buffer() }
}
