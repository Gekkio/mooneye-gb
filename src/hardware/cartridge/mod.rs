use std::iter;

use config::{
  CartridgeConfig,
  CartridgeRamSize,
  CartridgeType
};
use gameboy::{
  RAM_BANK_SIZE, ROM_BANK_SIZE
};

enum Mbc {
  None,
  Mbc1
}

impl Mbc {
  fn from_cartridge_type(t: CartridgeType) -> Mbc {
    use config::CartridgeType::*;
    match t {
      RomOnly => Mbc::None,
      Mbc1 | Mbc1Ram | Mbc1RamBattery => Mbc::Mbc1
    }
  }
}

pub struct Cartridge {
  mbc: Mbc,
  rom: Vec<u8>,
  rom_offset: usize,
  rom_bank: u8,
  rom_banks: usize,
  ram: Vec<u8>,
  ram_offset: usize,
  ram_bank: u8,
  mbc1_ram_banking: bool,
  writable: bool
}

impl Cartridge {
  pub fn new(config: CartridgeConfig) -> Cartridge {
    let mbc = Mbc::from_cartridge_type(config.cartridge_type);
    let ram_size = config.ram_size.as_usize();
    let rom_banks = config.rom_size.banks();
    Cartridge {
      mbc: mbc,
      rom: config.data,
      rom_bank: 0,
      rom_offset: 0x4000,
      rom_banks: rom_banks,
      ram: iter::repeat(0).take(ram_size).collect(),
      ram_offset: 0x0000,
      ram_bank: 0,
      mbc1_ram_banking: false,
      writable: false
    }
  }

  pub fn read_rom_bank0(&self, reladdr: u16) -> u8 {
    self.rom[reladdr as usize]
  }
  pub fn read_rom_bankx(&self, reladdr: u16) -> u8 {
    self.rom[self.rom_offset + reladdr as usize]
  }
  pub fn write_control(&mut self, reladdr: u16, value: u8) {
    match self.mbc {
      Mbc::None => (),
      Mbc::Mbc1 => {
        match reladdr >> 8 {
          0x00 ... 0x1f => {
            self.writable = (value & 0x0f) == 0x0a;
          },
          0x20 ... 0x3f => {
            self.rom_bank = (self.rom_bank & 0x60) | (value & 0x1f);
            self.update_rom_offset();
          },
          0x40 ... 0x5f => {
            if self.mbc1_ram_banking {
              self.ram_bank = value & 0x03;
              self.update_ram_offset();
            } else {
              self.rom_bank = ((value & 0x03) << 5) | (self.rom_bank & 0x1f);
              self.update_rom_offset();
            }
          },
          0x60 ... 0x7f => {
            self.mbc1_ram_banking = (value & 0x01) == 0x01;
          },
          _ => panic!("Unsupported MBC control {:04x} = {:02x}", reladdr, value)
        }
      }
    }
  }
  pub fn read_ram(&self, reladdr: u16) -> u8 {
    if self.writable {
      if let Some(addr) = self.ram.as_slice().get(self.ram_offset + reladdr as usize) {
        return *addr
      }
    }
    0xff
  }
  pub fn write_ram(&mut self, reladdr: u16, value: u8) {
    if self.writable {
      if let Some(addr) = self.ram.as_mut_slice().get_mut(self.ram_offset + reladdr as usize) {
        *addr = value;
      }
    }
  }
  fn update_rom_offset(&mut self) {
    match self.mbc {
      Mbc::Mbc1 => {
        let bank =
          match self.rom_bank & (self.rom_banks as u8 - 1) { // ROM bank numbers wrap around
            0x00 => 0x01,
            bank => bank
          };
        self.rom_offset = ROM_BANK_SIZE * bank as usize;
      },
      _ => ()
    }
  }
  fn update_ram_offset(&mut self) {
    match self.mbc {
      Mbc::Mbc1 => {
        self.ram_offset = RAM_BANK_SIZE * self.ram_bank as usize;
      },
      _ => ()
    }
  }
}
