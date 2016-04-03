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
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Sub};
use time::Duration;

use gameboy;
pub use self::emu_events::{EmuEvents, EE_DEBUG_OP, EE_VSYNC};
pub use self::time::EmuTime;

mod time;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ClockEdges(pub u32);

impl Add<ClockEdges> for ClockEdges {
  type Output = ClockEdges;
  fn add(self, rhs: ClockEdges) -> ClockEdges {
    ClockEdges(self.0 + rhs.0)
  }
}

impl Sub<ClockEdges> for ClockEdges {
  type Output = ClockEdges;
  fn sub(self, rhs: ClockEdges) -> ClockEdges {
    ClockEdges(self.0 - rhs.0)
  }
}

impl fmt::Debug for ClockEdges {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[allow(dead_code)]
mod emu_events {
  bitflags!(
    pub flags EmuEvents: u8 {
      const EE_DEBUG_OP = 0b_0000_0001,
      const EE_VSYNC    = 0b_0000_0010
    }
  );
}
