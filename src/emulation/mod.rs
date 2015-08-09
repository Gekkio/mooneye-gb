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
use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Sub};
use time::Duration;

use gameboy;
pub use self::emu_events::{EmuEvents, EE_DEBUG_OP};
pub use self::time::EmuTime;

mod time;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct MachineCycles(pub u32);

impl MachineCycles {
  pub fn as_clock_cycles(self) -> u32 {
    self.0 * 4
  }
  pub fn as_duration(self) -> Duration {
    Duration::seconds(self.as_clock_cycles() as i64) / gameboy::CPU_SPEED_HZ as i32
  }
}

impl Add<MachineCycles> for MachineCycles {
  type Output = MachineCycles;
  fn add(self, rhs: MachineCycles) -> MachineCycles {
    MachineCycles(self.0 + rhs.0)
  }
}

impl Sub<MachineCycles> for MachineCycles {
  type Output = MachineCycles;
  fn sub(self, rhs: MachineCycles) -> MachineCycles {
    MachineCycles(self.0 - rhs.0)
  }
}

impl fmt::Debug for MachineCycles {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.0)
  }
}

#[allow(dead_code)]
mod emu_events {
  bitflags!(
    flags EmuEvents: u8 {
      const EE_DEBUG_OP = 0b_0000_0001
    }
  );
}
