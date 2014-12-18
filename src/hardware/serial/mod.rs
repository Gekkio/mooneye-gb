use std::char;

pub struct Serial {
  data: u8,
  control: Control
}

impl Serial {
  pub fn new() -> Serial {
    Serial {
      data: 0x00,
      control: Control::empty()
    }
  }
  pub fn get_data(&self) -> u8 {
    self.data
  }
  pub fn set_data(&mut self, value: u8) {
    self.data = value
  }
  pub fn get_control(&self) -> u8 {
    self.control.bits & CTRL_UNUSED_MASK
  }
  pub fn set_control(&mut self, value: u8) {
    self.control = Control::from_bits_truncate(value);
    if self.control.contains(CTRL_START) {
      // println!("Serial transfer {:02x} {}, control = {:08b}", self.data, char::from_u32(self.data as u32).unwrap_or('?'), self.control.bits);
    }
  }
}

const CTRL_UNUSED_MASK: u8 = (1 << 1) | (1 << 2) | (1 << 3) |
                             (1 << 4) | (1 << 5) | (1 << 6);

bitflags!(
  flags Control: u8 {
    const CTRL_CLOCK = 1 << 0,
    const CTRL_START = 1 << 7
  }
)
