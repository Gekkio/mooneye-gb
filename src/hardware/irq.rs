use std::num::FromPrimitive;

use util::int::IntExt;

pub struct Irq {
  int_flag: InterruptType,
  int_enable: InterruptType
}

impl Irq {
  pub fn new() -> Irq {
    Irq {
      int_flag: InterruptType::empty(),
      int_enable: InterruptType::empty()
    }
  }
  pub fn get_interrupt_flag(&self) -> u8 {
    self.int_flag.bits | INT_UNUSED_MASK
  }
  pub fn get_interrupt_enable(&self) -> u8 {
    self.int_enable.bits | INT_UNUSED_MASK
  }
  pub fn set_interrupt_flag(&mut self, value: u8) {
    self.int_flag = InterruptType::from_bits_truncate(value);
  }
  pub fn set_interrupt_enable(&mut self, value: u8) {
    self.int_enable = InterruptType::from_bits_truncate(value);
  }
  pub fn request_interrupt(&mut self, interrupt: Interrupt) {
    self.int_flag = self.int_flag | InterruptType::from_bits_truncate(interrupt as u8);
  }
  pub fn ack_interrupt(&mut self) -> Option<Interrupt> {
    let highest_priority = (self.int_enable & self.int_flag).isolate_highest_priority();
    self.int_flag = self.int_flag - highest_priority;
    FromPrimitive::from_u8(highest_priority.bits)
  }
  pub fn has_interrupt(&self) -> bool { (self.int_enable & self.int_flag) != InterruptType::empty() }
}

#[derive(FromPrimitive, Debug)]
pub enum Interrupt {
  VBlank = 1 << 0,
  LcdStat = 1 << 1,
  TimerOverflow = 1 << 2,
  SerialIoDone = 1 << 3,
  Joypad = 1 << 4
}

impl Interrupt {
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

const INT_UNUSED_MASK: u8 = (1 << 5) | (1 << 6) | (1 << 7);

bitflags!(
  flags InterruptType: u8 {
    const INT_VBLANK = Interrupt::VBlank as u8,
    const INT_LCDSTAT = Interrupt::LcdStat as u8,
    const INT_TIMER_OVERFLOW = Interrupt::TimerOverflow as u8,
    const INT_SERIAL_IO_DONE = Interrupt::SerialIoDone as u8,
    const INT_JOYPAD = Interrupt::Joypad as u8,
  }
);

impl InterruptType {
  fn isolate_highest_priority(&self) -> InterruptType {
    InterruptType { bits: self.bits.isolate_rightmost_one() }
  }
}
