// This file is part of Mooneye GB.
// Copyright (C) 2014-2018 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use hardware::irq::{Irq, Interrupt};

const CONTROL_UNUSED_MASK: u8 = (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3);

pub struct Timer {
  internal_counter: u16,
  counter: u8,
  modulo: u8,
  enabled: bool,
  input_clock: InputClock,
  timer_cycles: i32
}

#[derive(Clone, Copy)]
enum InputClock {
  Hz4096 = 0,
  Hz262144 = 1,
  Hz65536 = 2,
  Hz16384 = 3
}

impl InputClock {
  pub fn from_u8(value: u8) -> Option<InputClock> {
    use self::InputClock::*;
    match value {
      0 => Some(Hz4096),
      1 => Some(Hz262144),
      2 => Some(Hz65536),
      3 => Some(Hz16384),
      _ => None
    }
  }
  fn cycles(&self) -> i32 {
    match *self {
      InputClock::Hz4096 => 1024 / 4,
      InputClock::Hz16384 => 256 / 4,
      InputClock::Hz65536 => 64 / 4,
      InputClock::Hz262144 => 16 / 4
    }
  }
}

impl Timer {
  pub fn new() -> Timer {
    Timer {
      internal_counter: 0,
      counter: 0,
      modulo: 0,
      enabled: false,
      input_clock: InputClock::Hz4096,
      timer_cycles: InputClock::Hz4096.cycles()
    }
  }
  pub fn get_divider(&self) -> u8 {
    (self.internal_counter >> 8) as u8
  }
  pub fn get_counter(&self) -> u8 {
    self.counter
  }
  pub fn get_modulo(&self) -> u8 {
    self.modulo
  }
  pub fn get_control(&self) -> u8 {
    self.input_clock as u8 | CONTROL_UNUSED_MASK |
      if self.enabled { (1 << 2) } else { 0 }
  }
  pub fn reset_divider(&mut self) {
    self.internal_counter = 0;
  }
  pub fn set_counter(&mut self, value: u8) {
    self.counter = value;
  }
  pub fn set_modulo(&mut self, value: u8) {
    self.modulo = value;
  }
  pub fn set_control(&mut self, value: u8) {
    self.enabled = (value & (1 << 2)) != 0;
    self.input_clock = InputClock::from_u8(value & 0x3).unwrap();
    self.timer_cycles = self.input_clock.cycles();
  }
  pub fn emulate(&mut self, irq: &mut Irq) {
    self.internal_counter = self.internal_counter.wrapping_add(4);
    if self.enabled {
      self.timer_cycles -= 1;
      if self.timer_cycles <= 0 {
        self.timer_cycles += self.input_clock.cycles();
        if self.counter != 0xff {
          self.counter += 1;
        } else {
          self.counter = self.modulo;
          irq.request_interrupt(Interrupt::TimerOverflow);
        }
      }
    }
  }
}
