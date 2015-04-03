use std::fmt;
use std::fmt::Formatter;
use std::ops::{Add, Sub};
use std::time::duration::Duration;

use gameboy;
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
