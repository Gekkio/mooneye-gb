use std::num::FromPrimitive;

pub type BootromData = [u8; BOOTROM_SIZE];
pub type HiramData = [u8; HIRAM_SIZE];
pub type ScreenBuffer = [Color; SCREEN_PIXELS];
pub type WramBank = [u8; WRAM_BANK_SIZE];

#[derive(PartialEq, FromPrimitive, Copy)]
pub enum Color {
  Off = 0,
  Light = 1,
  Dark = 2,
  On = 3
}

impl Color {
  pub fn from_u8(value: u8) -> Color {
    FromPrimitive::from_u8(value).unwrap_or(Color::Off)
  }
}

pub const BOOTROM_SIZE: uint = 0x100;
pub const BOOTROM_EMPTY: BootromData = [0; BOOTROM_SIZE];
pub const CPU_SPEED_HZ: uint = 4_194_304;
pub const HIRAM_SIZE: uint = 0x80;
pub const HIRAM_EMPTY: HiramData = [0; HIRAM_SIZE];
pub const ROM_BANK_SIZE: uint = 0x4000;
pub const RAM_BANK_SIZE: uint = 0x2000;
pub const SCREEN_WIDTH: uint = 160;
pub const SCREEN_HEIGHT: uint = 144;
pub const SCREEN_PIXELS: uint = SCREEN_WIDTH * SCREEN_HEIGHT;
pub const SCREEN_EMPTY: ScreenBuffer = [Color::Off; SCREEN_PIXELS];
pub const WRAM_BANK_SIZE: uint = 0x1000;
pub const WRAM_BANK_EMPTY: WramBank = [0; WRAM_BANK_SIZE];
