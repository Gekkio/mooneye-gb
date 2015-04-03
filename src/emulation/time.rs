use std::ops::{Add, Sub};

use super::MachineCycles;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EmuTime {
  cycles: MachineCycles
}

const REWIND_THRESHOLD: MachineCycles = MachineCycles(0x80000000);

impl EmuTime {
  pub fn zero() -> EmuTime { EmuTime::new(MachineCycles(0)) }
  pub fn new(cycles: MachineCycles) -> EmuTime {
    EmuTime { cycles: cycles }
  }
  pub fn cycles(&self) -> MachineCycles { self.cycles }
  pub fn tick(&mut self) {
    self.cycles = self.cycles + MachineCycles(1);
  }
  pub fn needs_rewind(&self) -> bool {
    self.cycles >= REWIND_THRESHOLD
  }
  pub fn rewind(&mut self) {
    assert!(self.needs_rewind());
    self.cycles = self.cycles - REWIND_THRESHOLD;
  }
}

impl Add<MachineCycles> for EmuTime {
  type Output = EmuTime;
  fn add(self, rhs: MachineCycles) -> EmuTime {
    EmuTime {
      cycles: self.cycles + rhs
    }
  }
}

impl Sub for EmuTime {
  type Output = MachineCycles;
  fn sub(self, rhs: EmuTime) -> MachineCycles {
    self.cycles - rhs.cycles
  }
}
