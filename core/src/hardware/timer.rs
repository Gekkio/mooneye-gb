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
use bitflags::bitflags;

use crate::hardware::irq::{Interrupt, InterruptRequest};
use crate::hardware::MappedHardware;

pub struct Timer {
  internal_counter: u16,
  tac: TacReg,
  counter: u8,
  modulo: u8,
  overflow: bool,
  enabled: bool,
}

bitflags!(
  struct TacReg: u8 {
    const ENABLE = 0b100;
    const MASK_1 = 0b010;
    const MASK_0 = 0b001;
  }
);

impl TacReg {
  fn counter_mask(&self) -> u16 {
    match self.bits() & 0b11 {
      0b11 => (1 << 5),
      0b10 => (1 << 3),
      0b01 => (1 << 1),
      _ => (1 << 7),
    }
  }
}

pub struct Div;
pub struct Tima;
pub struct Tma;
pub struct Tac;

impl MappedHardware<Div> for Timer {
  fn read_cycle<I: InterruptRequest>(&mut self, _: Div, intr_req: &mut I) -> u8 {
    self.tick_cycle(intr_req);
    (self.internal_counter >> 6) as u8
  }
  fn write_cycle<I: InterruptRequest>(&mut self, _: Div, _: u8, intr_req: &mut I) {
    self.tick_cycle(intr_req);
    if self.counter_bit() {
      self.increment();
    }
    self.internal_counter = 0;
  }
}

impl MappedHardware<Tima> for Timer {
  fn read_cycle<I: InterruptRequest>(&mut self, _: Tima, intr_req: &mut I) -> u8 {
    self.tick_cycle(intr_req);
    self.counter
  }
  fn write_cycle<I: InterruptRequest>(&mut self, _: Tima, value: u8, intr_req: &mut I) {
    let overflow = self.overflow;
    self.tick_cycle(intr_req);
    if !overflow {
      self.overflow = false;
      self.counter = value
    }
  }
}

impl MappedHardware<Tma> for Timer {
  fn read_cycle<I: InterruptRequest>(&mut self, _: Tma, intr_req: &mut I) -> u8 {
    self.tick_cycle(intr_req);
    self.modulo
  }
  fn write_cycle<I: InterruptRequest>(&mut self, _: Tma, value: u8, intr_req: &mut I) {
    let overflow = self.overflow;
    self.tick_cycle(intr_req);
    self.modulo = value;
    if overflow {
      self.counter = value;
    }
  }
}

impl MappedHardware<Tac> for Timer {
  fn read_cycle<I: InterruptRequest>(&mut self, _: Tac, intr_req: &mut I) -> u8 {
    self.tick_cycle(intr_req);
    const TAC_UNUSED: u8 = 0b11111_000;
    TAC_UNUSED | self.tac.bits()
  }
  fn write_cycle<I: InterruptRequest>(&mut self, _: Tac, value: u8, intr_req: &mut I) {
    self.tick_cycle(intr_req);
    let old_bit = self.enabled && self.counter_bit();
    self.tac = TacReg::from_bits_truncate(value);
    self.enabled = self.tac.contains(TacReg::ENABLE);
    let new_bit = self.enabled && self.counter_bit();
    if old_bit && !new_bit {
      self.increment();
    }
  }
}

impl Timer {
  pub fn new() -> Timer {
    Timer {
      internal_counter: 0,
      tac: TacReg::empty(),
      counter: 0,
      modulo: 0,
      overflow: false,
      enabled: false,
    }
  }
  fn counter_bit(&self) -> bool {
    (self.internal_counter & self.tac.counter_mask()) != 0
  }
  fn increment(&mut self) {
    let (counter, overflow) = self.counter.overflowing_add(1);
    self.counter = counter;
    self.overflow = overflow;
  }
  pub fn tick_cycle<I: InterruptRequest>(&mut self, intr_req: &mut I) {
    if self.overflow {
      self.internal_counter = self.internal_counter.wrapping_add(1);
      self.counter = self.modulo;
      intr_req.request_t12_interrupt(Interrupt::TimerOverflow);
      self.overflow = false;
    } else if self.enabled && self.counter_bit() {
      self.internal_counter = self.internal_counter.wrapping_add(1);
      let new_bit = self.counter_bit();
      if !new_bit {
        self.increment();
      }
    } else {
      self.internal_counter = self.internal_counter.wrapping_add(1);
    }
  }
}
