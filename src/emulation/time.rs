// This file is part of Mooneye GB.
// Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
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

use super::ClockEdges;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EmuTime {
  clock_edges: ClockEdges
}

const REWIND_THRESHOLD: ClockEdges = ClockEdges(0x80000000);

impl EmuTime {
  pub fn zero() -> EmuTime { EmuTime::new(ClockEdges(0)) }
  pub fn new(clock_edges: ClockEdges) -> EmuTime {
    EmuTime { clock_edges: clock_edges }
  }
  pub fn clock_edges(&self) -> ClockEdges { self.clock_edges }
  pub fn tick_machine_cycle(&mut self) {
    self.clock_edges = self.clock_edges + ClockEdges(8);
  }
  pub fn needs_rewind(&self) -> bool {
    self.clock_edges >= REWIND_THRESHOLD
  }
  pub fn rewind(&mut self) {
    assert!(self.needs_rewind());
    self.clock_edges = self.clock_edges - REWIND_THRESHOLD;
  }
}

impl Add<ClockEdges> for EmuTime {
  type Output = EmuTime;
  fn add(self, rhs: ClockEdges) -> EmuTime {
    EmuTime {
      clock_edges: self.clock_edges + rhs
    }
  }
}

impl Sub for EmuTime {
  type Output = ClockEdges;
  fn sub(self, rhs: EmuTime) -> ClockEdges {
    self.clock_edges - rhs.clock_edges
  }
}
