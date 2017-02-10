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
use std::fmt;
use std::ops::{Add, AddAssign, Sub};

use super::EmuDuration;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct EmuTime {
  clock_edges: u32
}

const REWIND_THRESHOLD: u32 = 0x80000000;

impl EmuTime {
  pub fn zero() -> EmuTime { EmuTime { clock_edges: 0 } }
  pub fn needs_rewind(&self) -> bool {
    self.clock_edges >= REWIND_THRESHOLD
  }
  pub fn rewind(&mut self) {
    assert!(self.needs_rewind());
    self.clock_edges -= REWIND_THRESHOLD;
  }
  pub fn as_duration(&self) -> EmuDuration { EmuDuration::clock_edges(self.clock_edges) }
}

impl Add<EmuDuration> for EmuTime {
  type Output = EmuTime;
  fn add(self, rhs: EmuDuration) -> EmuTime {
    EmuTime {
      clock_edges: self.clock_edges + rhs.as_clock_edges()
    }
  }
}

impl AddAssign<EmuDuration> for EmuTime {
  fn add_assign(&mut self, rhs: EmuDuration) {
    self.clock_edges += rhs.as_clock_edges();
  }
}

impl Sub for EmuTime {
  type Output = EmuDuration;
  fn sub(self, rhs: EmuTime) -> EmuDuration {
    EmuDuration::clock_edges(self.clock_edges - rhs.clock_edges)
  }
}

impl fmt::Display for EmuTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} clock edges", self.clock_edges)
  }
}
