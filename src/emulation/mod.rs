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
pub struct EmuDuration(u32);

impl EmuDuration {
  pub fn clock_edges(amount: u32) -> EmuDuration { EmuDuration(amount) }
  pub fn clock_cycles(amount: u32) -> EmuDuration { EmuDuration(amount * 2) }
  pub fn machine_cycles(amount: u32) -> EmuDuration { EmuDuration(amount * 8) }
  pub fn as_clock_edges(self) -> u32 { self.0 }
}

impl Add<EmuDuration> for EmuDuration {
  type Output = EmuDuration;
  fn add(self, rhs: EmuDuration) -> EmuDuration {
    EmuDuration(self.0 + rhs.0)
  }
}

impl Sub<EmuDuration> for EmuDuration {
  type Output = EmuDuration;
  fn sub(self, rhs: EmuDuration) -> EmuDuration {
    EmuDuration(self.0 - rhs.0)
  }
}

impl fmt::Debug for EmuDuration {
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
