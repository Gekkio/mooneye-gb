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
