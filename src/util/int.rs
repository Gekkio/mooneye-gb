use num::traits::{Num, One, Zero};
use std::num::Wrapping;
use std::ops::{BitAnd, Shl, Shr};

#[cfg(test)]
use quickcheck::quickcheck;

pub trait IntExt where Self: Num + BitAnd<Output=Self> + Shl<usize, Output=Self> + Shr<usize, Output=Self> + Sized {
  fn isolate_rightmost_one(self) -> Self;
  fn bit(self, bit: usize) -> Self {
    (self >> bit) & One::one()
  }
  fn bit_bool(self, bit: usize) -> bool {
    !self.bit(bit).is_zero()
  }
}

impl IntExt for u8 {
  fn isolate_rightmost_one(self) -> u8 {
    let x = Wrapping(self);
    let one = Wrapping(1);
    // Hacker's Delight 2nd ed, 2-1 Manipulating Rightmost Bits
    (x & (!x + one)).0
  }
}

impl IntExt for u16 {
  fn isolate_rightmost_one(self) -> u16 {
    let x = Wrapping(self);
    let one = Wrapping(1);
    // Hacker's Delight 2nd ed, 2-1 Manipulating Rightmost Bits
    (x & (!x + one)).0
  }
}

#[cfg(test)]
fn test_isolate_rightmost_one<T: IntExt + Copy>(x: T) -> bool {
  let y = x.isolate_rightmost_one();
  if (x.is_zero()) { y.is_zero() }
  else {
    let mut value = x;
    let mut expected = One::one();
    while (!value.bit_bool(0)) {
      value = value >> 1;
      expected = expected << 1;
    }
    y == expected
  }
}

#[cfg(test)]
#[quickcheck]
fn test_u8_isolate_rightmost_one(x: u8) -> bool { test_isolate_rightmost_one(x) }

#[cfg(test)]
#[quickcheck]
fn test_u16_isolate_rightmost_one(x: u16) -> bool { test_isolate_rightmost_one(x) }
