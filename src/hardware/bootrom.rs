use std::ops::Index;

pub struct Bootrom {
  data: Vec<u8>,
  installed: bool,
  active: bool
}

impl Bootrom {
  pub fn new(config: Option<Vec<u8>>) -> Bootrom {
    let (installed, data) = match config {
      Some(config_data) => (true, config_data),
      None => (false, vec![])
    };

    Bootrom {
      data: data,
      installed: installed,
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
  fn index(&self, index: u16) -> &u8 {
    &self.data[index as usize]
  }
}
