use std::num::FromPrimitive;

use super::envelope::Envelope;
use super::wave_duty::WaveDuty;

pub struct Ch2 {
  wave_duty: WaveDuty,
  pub envelope: Envelope,
  freq_bits: u16,
  use_counter: bool,
  counter: uint,
  pub status: bool
}

impl Ch2 {
  pub fn new() -> Ch2 {
    Ch2 {
      wave_duty: WaveDuty::HalfQuarter,
      envelope: Envelope::new(),
      freq_bits: 0,
      use_counter: false,
      counter: 0,
      status: false
    }
  }
  pub fn reset(&mut self) {
    *self = Ch2::new()
  }
  pub fn read_reg1(&self) -> u8 {
    const REG1_MASK: u8 = 0x3F;

    REG1_MASK |
    ((self.wave_duty as u8) << 6)
  }
  pub fn write_reg1(&mut self, value: u8) {
    self.wave_duty = FromPrimitive::from_u8((value >> 6) & 0x03).unwrap();
    self.counter = 64 - (value & 0x3f) as uint;
  }
  pub fn write_reg3(&mut self, value: u8) {
    self.freq_bits = (self.freq_bits & 0x700) | value as u16;
  }
  pub fn read_reg4(&self) -> u8 {
    const REG4_MASK: u8 = 0xBF;

    REG4_MASK |
    if self.use_counter { 1 << 6 } else { 0 }
  }
  pub fn write_reg4(&mut self, value: u8) {
    self.status = value & (1 << 7) != 0;
    self.use_counter = value & (1 << 6) != 0;
    self.freq_bits = (self.freq_bits & 0xff) | ((value as u16) << 8);
    if self.status && self.counter == 0 {
      self.counter = 64;
    }
  }
  pub fn clock(&mut self) {
    if self.use_counter && self.counter > 0 {
      self.counter -= 1;

      if self.counter == 0 {
        self.status = false;
      }
    }
  }
}
