use std::num::FromPrimitive;

use emulation::EmuTime;
use hardware::irq::{Irq, Interrupt};

const CONTROL_UNUSED_MASK: u8 = (1 << 7) | (1 << 6) | (1 << 5) | (1 << 4) | (1 << 3);

pub struct Timer {
  counter: u8,
  modulo: u8,
  enabled: bool,
  input_clock: InputClock,
  timer_cycles: i32,
  divider_time: EmuTime,
}

#[derive(FromPrimitive, Clone, Copy)]
enum InputClock {
  Hz4096 = 0,
  Hz262144 = 1,
  Hz65536 = 2,
  Hz16384 = 3
}

impl InputClock {
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
      counter: 0,
      modulo: 0,
      enabled: false,
      input_clock: InputClock::Hz4096,
      timer_cycles: InputClock::Hz4096.cycles(),
      divider_time: EmuTime::zero()
    }
  }
  pub fn get_divider(&self, time: EmuTime) -> u8 {
    assert!(time >= self.divider_time);
    ((time - self.divider_time).0 / 64) as u8
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
  pub fn reset_divider(&mut self, time: EmuTime) {
    self.divider_time = time;
  }
  pub fn set_counter(&mut self, value: u8) {
    self.counter = value;
  }
  pub fn set_modulo(&mut self, value: u8) {
    self.modulo = value;
  }
  pub fn set_control(&mut self, value: u8) {
    println!("TIMER CONTROL {:08b}", value);
    self.enabled = (value & (1 << 2)) != 0;
    self.input_clock = FromPrimitive::from_u8(value & 0x3).unwrap();
    self.timer_cycles = self.input_clock.cycles();
  }
  pub fn rewind_time(&mut self) {
    self.divider_time.rewind();
  }
  pub fn emulate(&mut self, irq: &mut Irq) {
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
