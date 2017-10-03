// This file is part of Mooneye GB.
// Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use util::int::IntExt;

pub struct Irq {
  int_flag: InterruptType,
  int_enable: u8,
}

impl Irq {
  pub fn new() -> Irq {
    Irq {
      int_flag: InterruptType::empty(),
      int_enable: 0x00,
    }
  }
  pub fn get_interrupt_flag(&self) -> u8 {
    const IF_UNUSED_MASK: u8 = (1 << 5) | (1 << 6) | (1 << 7);

    self.int_flag.bits | IF_UNUSED_MASK
  }
  pub fn get_interrupt_enable(&self) -> u8 {
    self.int_enable
  }
  pub fn set_interrupt_flag(&mut self, value: u8) {
    self.int_flag = InterruptType::from_bits_truncate(value);
  }
  pub fn set_interrupt_enable(&mut self, value: u8) {
    self.int_enable = value;
  }
  pub fn request_interrupt(&mut self, interrupt: Interrupt) {
    self.int_flag |= InterruptType::from_bits_truncate(interrupt as u8);
  }
  pub fn ack_interrupt(&mut self) -> Option<Interrupt> {
    let highest_priority = (InterruptType::from_bits_truncate(self.int_enable) & self.int_flag).isolate_highest_priority();
    self.int_flag -= highest_priority;
    Interrupt::from_u8(highest_priority.bits)
  }
  pub fn has_interrupt(&self) -> bool { (InterruptType::from_bits_truncate(self.int_enable) & self.int_flag) != InterruptType::empty() }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
  VBlank = 1 << 0,
  LcdStat = 1 << 1,
  TimerOverflow = 1 << 2,
  SerialIoDone = 1 << 3,
  Joypad = 1 << 4
}

impl Interrupt {
  pub fn from_u8(value: u8) -> Option<Interrupt> {
    use self::Interrupt::*;
    match value {
      1 => Some(VBlank),
      2 => Some(LcdStat),
      4 => Some(TimerOverflow),
      8 => Some(SerialIoDone),
      16 => Some(Joypad),
      _ => None
    }
  }
  pub fn get_addr(&self) -> u16 {
    match *self {
      Interrupt::VBlank => 0x40,
      Interrupt::LcdStat => 0x48,
      Interrupt::TimerOverflow => 0x50,
      Interrupt::SerialIoDone => 0x58,
      Interrupt::Joypad => 0x60
    }
  }
}

bitflags!(
  pub struct InterruptType: u8 {
    const VBLANK = 1 << 0;
    const LCDSTAT = 1 << 1;
    const TIMER_OVERFLOW = 1 << 2;
    const SERIAL_IO_DONE = 1 << 3;
    const JOYPAD = 1 << 4;
  }
);

impl InterruptType {
  #[inline(always)]
  fn isolate_highest_priority(&self) -> InterruptType {
    InterruptType { bits: self.bits.isolate_rightmost_one() }
  }
}
