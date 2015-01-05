#![allow(dead_code)]
use std::fmt;

bitflags!(
  flags Flags: u8 {
    const ZERO = 0x80,
    const ADD_SUBTRACT = 0x40,
    const HALF_CARRY = 0x20,
    const CARRY = 0x10
  }
);

impl Flags {
  pub fn test(&self, test: bool) -> Flags {
    if test { *self } else { Flags::empty() }
  }
  pub fn get(&self) -> u8 { self.bits }
  pub fn set(&mut self, value: u8) { *self = Flags::from_bits_truncate(value) }
}

impl fmt::Show for Flags {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:04b}", self.bits >> 4)
  }
}

#[derive(Copy, Show)]
pub enum Reg8 {
  A, B, C, D, E, H, L
}

#[derive(Copy, Show)]
pub enum Reg16 {
  AF, BC, DE, HL, SP
}

pub struct Registers {
  pub a: u8,
  pub b: u8,
  pub c: u8,
  pub d: u8,
  pub e: u8,
  pub f: Flags,
  pub h: u8,
  pub l: u8,
  pub sp: u16,
  pub pc: u16
}

impl Registers {
  pub fn new() -> Registers {
    Registers {
      a: 0,
      b: 0,
      c: 0,
      d: 0,
      e: 0,
      f: Flags::empty(),
      h: 0,
      l: 0,
      sp: 0,
      pc: 0
    }
  }

  pub fn read16(&self, reg: Reg16) -> u16 {
    use self::Reg16::*;
    match reg {
      AF => ((self.a as u16) << 8) | (self.f.get() as u16),
      BC => ((self.b as u16) << 8) | (self.c as u16),
      DE => ((self.d as u16) << 8) | (self.e as u16),
      HL => ((self.h as u16) << 8) | (self.l as u16),
      SP => self.sp,
    }
  }
  pub fn write16(&mut self, reg: Reg16, value: u16) {
    use self::Reg16::*;
    match reg {
      AF => { self.a = (value >> 8) as u8; self.f.set(value as u8) },
      BC => { self.b = (value >> 8) as u8; self.c = value as u8 },
      DE => { self.d = (value >> 8) as u8; self.e = value as u8 },
      HL => { self.h = (value >> 8) as u8; self.l = value as u8 },
      SP => self.sp = value
    }
  }
}

impl fmt::Show for Registers {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "A:{:2x} B:{:2x} C:{:2x} D:{:2x} E:{:2x} \
               F:{} H:{:2x} L:{:2x} SP:{:4x} PC:{:4x}",
               self.a, self.b, self.c, self.d, self.e,
               self.f, self.h, self.l, self.sp, self.pc)
  }
}
