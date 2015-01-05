use std::num::Int;
use std::ops::{Add, Sub};

pub type Cycles = u32;

#[derive(PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct Clock {
  m_cycles: Cycles
}

const NORMALIZE_THRESHOLD: Cycles = 0x80000000;

impl Clock {
  pub fn new() -> Clock { Clock::from_machine_cycles(0) }
  pub fn from_machine_cycles(cycles: Cycles) -> Clock {
    Clock {
      m_cycles: cycles
    }
  }
  pub fn from_clock_cycles(cycles: Cycles) -> Clock {
    assert_eq!(cycles & 0x03, 0);
    Clock {
      m_cycles: cycles / 4
    }
  }
  pub fn as_machine_cycles(&self) -> Cycles {
    self.m_cycles
  }
  pub fn as_clock_cycles(&self) -> Cycles {
    self.m_cycles * 4
  }
  pub fn tick(&mut self) {
    self.m_cycles += 1;
  }
  pub fn needs_normalize(&self) -> bool {
    self.m_cycles >= NORMALIZE_THRESHOLD
  }
  pub fn normalize(&mut self) {
    self.m_cycles =
      self.m_cycles.checked_sub(NORMALIZE_THRESHOLD).expect("Underflow in Clock::normalize()");
  }
}

impl Add<Cycles> for Clock {
  type Output = Clock;
  fn add(self, rhs: Cycles) -> Clock {
    Clock::from_machine_cycles(self.m_cycles + rhs)
  }
}

impl Sub for Clock {
  type Output = Clock;
  fn sub(self, rhs: Clock) -> Clock {
    Clock::from_machine_cycles(self.m_cycles - rhs.m_cycles)
  }
}
