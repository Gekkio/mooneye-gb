pub trait IntOps {
  fn isolate_rightmost_one(self) -> Self;
  fn bit(self, uint) -> Self;
  fn bit_bool(self, uint) -> bool;
}

impl IntOps for u8 {
  #[allow(unsigned_negation)]
  fn isolate_rightmost_one(self) -> u8 {
    // Hacker's Delight 2nd ed, 2-1 Manipulating Rightmost Bits
    self & (-self)
  }
  fn bit(self, bit: uint) -> u8 {
    (self >> bit) & 0x01
  }
  fn bit_bool(self, bit: uint) -> bool {
    self.bit(bit) != 0x00
  }
}
