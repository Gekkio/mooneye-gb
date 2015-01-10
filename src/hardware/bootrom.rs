use std::ops::Index;

use gameboy::{BootromData, BOOTROM_EMPTY};

pub struct Bootrom {
  data: BootromData,
  installed: bool,
  active: bool
}

impl Bootrom {
  pub fn new(config: Option<BootromData>) -> Bootrom {
    let data = match config {
      Some(config_data) => config_data,
      None => BOOTROM_EMPTY
    };

    Bootrom {
      data: data,
      installed: config.is_some(),
      active: false
    }
  }

  pub fn is_installed(&self) -> bool { self.installed }
  pub fn is_active(&self) -> bool { self.active }

  pub fn reset(&mut self) { self.active = self.installed; }
  pub fn deactivate(&mut self) { self.active = false; }
}

impl Index<u16> for Bootrom {
  type Output = u8;
  fn index(&self, index: &u16) -> &u8 {
    &self.data[*index as usize]
  }
}
