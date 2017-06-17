// This file is part of Mooneye GB.
// Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
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
pub struct BootromData(pub [u8; 0x100]);

impl BootromData {
  pub fn new() -> BootromData { BootromData([0; 0x100]) }
}

impl Clone for BootromData {
  fn clone(&self) -> BootromData { BootromData((*self).0) }
}

pub type HiramData = [u8; HIRAM_SIZE];
pub type ScreenBuffer = [Color; SCREEN_PIXELS];

#[derive(PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum Color {
  Off = 0,
  Light = 1,
  Dark = 2,
  On = 3
}

impl Color {
  #[inline]
  pub fn from_u8(value: u8) -> Color {
    use self::Color::*;
    match value {
      1 => Light,
      2 => Dark,
      3 => On,
      _ => Off
    }
  }
}

pub const CPU_SPEED_HZ: usize = 4_194_304;
pub const HIRAM_SIZE: usize = 0x80;
pub const HIRAM_EMPTY: HiramData = [0; HIRAM_SIZE];
pub const ROM_BANK_SIZE: usize = 0x4000;
pub const RAM_BANK_SIZE: usize = 0x2000;
pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;
pub const SCREEN_PIXELS: usize = SCREEN_WIDTH * SCREEN_HEIGHT;
pub const SCREEN_EMPTY: ScreenBuffer = [Color::Off; SCREEN_PIXELS];
