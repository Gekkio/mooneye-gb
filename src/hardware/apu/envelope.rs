pub struct Envelope {
  volume: u8,
  increasing: bool,
  length: u8
}

impl Envelope {
  pub fn new() -> Envelope {
    Envelope {
      volume: 0,
      increasing: false,
      length: 0
    }
  }
  pub fn read_reg(&self) -> u8 {
    (self.volume << 4) |
    if self.increasing { 1 << 3 } else { 0 } |
    self.length
  }
  pub fn write_reg(&mut self, value: u8) {
    self.volume = (value >> 4) & 0x0f;
    self.increasing = value & (1 << 3) != 0;
    self.length = value & 0x07;
  }
}
