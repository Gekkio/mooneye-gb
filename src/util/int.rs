pub trait IntOps {
  fn isolate_rightmost_one(self) -> Self;
  fn bit(self, usize) -> Self;
  fn bit_bool(self, usize) -> bool;
}

impl IntOps for u8 {
  #[allow(unsigned_negation)]
  fn isolate_rightmost_one(self) -> u8 {
    // Hacker's Delight 2nd ed, 2-1 Manipulating Rightmost Bits
    self & (-self)
  }
  fn bit(self, bit: usize) -> u8 {
    (self >> bit) & 0x01
  }
  fn bit_bool(self, bit: usize) -> bool {
    self.bit(bit) != 0x00
  }
}
