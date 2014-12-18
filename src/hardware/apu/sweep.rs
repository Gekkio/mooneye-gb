#[deriving(Copy, FromPrimitive)]
enum Time {
  None = 0,
  Div1 = 1,
  Div2 = 2,
  Div3 = 3,
  Div4 = 4,
  Div5 = 5,
  Div6 = 6,
  Div7 = 7
}

pub struct Sweep {
  time: Time,
  increasing: bool,
  shift: u8
}

impl Sweep {
  pub fn new() -> Sweep {
    Sweep {
      time: Time::None,
      increasing: false,
      shift: 0
    }
  }
  pub fn read_reg(&self) -> u8 {
    const MASK: u8 = 0x80;

    MASK |
    (self.time as u8 << 4) |
    if self.increasing { 1 << 3 } else { 0 } |
    (self.shift)
  }
  pub fn write_reg(&mut self, value: u8) {
    self.time = FromPrimitive::from_u8((value >> 4) & 0x07).unwrap();
    self.increasing = value & (1 << 3) != 0;
    self.shift = value & 0x07;
  }
}
