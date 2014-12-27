pub type BootromData = [u8, ..BOOTROM_SIZE];
pub type WramBank = [u8, ..WRAM_BANK_SIZE];

pub const BOOTROM_SIZE: uint = 0x100;
pub const BOOTROM_EMPTY: BootromData = [0, ..BOOTROM_SIZE];
pub const CPU_SPEED_HZ: uint = 4_194_304;
pub const ROM_BANK_SIZE: uint = 0x4000;
pub const RAM_BANK_SIZE: uint = 0x2000;
pub const SCREEN_WIDTH: uint = 160;
pub const SCREEN_HEIGHT: uint = 144;
pub const WRAM_BANK_SIZE: uint = 0x1000;
pub const WRAM_BANK_EMPTY: WramBank = [0, ..WRAM_BANK_SIZE];
