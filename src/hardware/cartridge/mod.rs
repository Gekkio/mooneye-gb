// This file is part of Mooneye GB.
// Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use std::iter;

use config::{
  CartridgeConfig,
  CartridgeType
};
use gameboy::{
  RAM_BANK_SIZE, ROM_BANK_SIZE
};
use util::int::IntExt;

#[cfg(all(test, not(feature = "acceptance_tests")))]
mod test;

enum Mbc {
  None, Mbc1, Mbc2, Mbc3
}

impl Mbc {
  fn from_cartridge_type(t: CartridgeType) -> Mbc {
    use config::CartridgeType::*;
    match t {
       Rom |  RomRam |  RomRamBattery => Mbc::None,
      Mbc1 | Mbc1Ram | Mbc1RamBattery => Mbc::Mbc1,
      Mbc2           | Mbc2RamBattery => Mbc::Mbc2,
      Mbc3 | Mbc3Ram | Mbc3RamBattery => Mbc::Mbc3
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
  ram_accessible: bool,
  mbc1_ram_banking: bool
}

impl Cartridge {
  pub fn new(config: CartridgeConfig) -> Cartridge {
    let mbc = Mbc::from_cartridge_type(config.cartridge_type);
    let ram_size = match mbc {
      Mbc::Mbc2 => 512,
      _ => config.ram_size.as_usize()
    };
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
      ram_accessible: false,
      mbc1_ram_banking: false
    }
  }

  pub fn read_rom_bank0(&self, reladdr: u16) -> u8 {
    self.rom[reladdr as usize]
  }
  pub fn read_rom_bankx(&self, reladdr: u16) -> u8 {
    self.rom[self.rom_addr(reladdr)]
  }
  pub fn write_control(&mut self, reladdr: u16, value: u8) {
    match self.mbc {
      Mbc::None => (),
      Mbc::Mbc1 => {
        match reladdr >> 8 {
          0x00 ... 0x1f => {
            self.ram_accessible = (value & 0x0f) == 0x0a;
          },
          0x20 ... 0x3f => {
            let value = if value & 0x1f == 0x00 { 0x01 } else { value & 0x1f };
            self.rom_bank = (self.rom_bank & 0x60) | value;
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
          _ => ()
        }
      },
      Mbc::Mbc2 => {
        match reladdr >> 8 {
          0x00 ... 0x1f => {
            if !reladdr.bit_bool(8) {
              self.ram_accessible = (value & 0x0f) == 0x0a;
            }
          },
          0x20 ... 0x3f => {
            if reladdr.bit_bool(8) {
              self.rom_bank = value & 0x0f;
              self.update_rom_offset();
            }
          },
          _ => ()
        }
      },
      Mbc::Mbc3 => {
        match reladdr >> 8 {
          0x00 ... 0x1f => {
            self.ram_accessible = (value & 0x0f) == 0x0a;
          },
          0x20 ... 0x3f => {
            self.rom_bank = value & 0x7f;
            self.update_rom_offset();
          },
          0x40 ... 0x5f => {
            self.ram_bank = value & 0x07;
            self.update_ram_offset();
          },
          _ => ()
        }
      }
    }
  }
  pub fn read_ram(&self, reladdr: u16) -> u8 {
    if self.ram_accessible && self.ram.len() > 0 {
      let addr = self.ram_addr(reladdr);
      self.ram[addr]
    } else { 0xff }
  }
  pub fn write_ram(&mut self, reladdr: u16, value: u8) {
    if self.ram_accessible && self.ram.len() > 0 {
      let addr = self.ram_addr(reladdr);
      match self.mbc {
        Mbc::Mbc2 => {
          self.ram[addr] = value & 0x0f;
        },
        _ => {
          self.ram[addr] = value;
        }
      }
    }
  }
  fn ram_addr(&self, reladdr: u16) -> usize {
    (self.ram_offset + reladdr as usize) & (self.ram.len() - 1)
  }
  fn rom_addr(&self, reladdr: u16) -> usize {
    (self.rom_offset + reladdr as usize) & (self.rom.len() - 1)
  }
  fn update_rom_offset(&mut self) {
    match self.mbc {
      Mbc::Mbc1 | Mbc::Mbc2 | Mbc::Mbc3 => {
        self.rom_offset = ROM_BANK_SIZE * self.rom_bank as usize;
      },
      _ => ()
    }
  }
  fn update_ram_offset(&mut self) {
    match self.mbc {
      Mbc::Mbc1 | Mbc::Mbc3 => {
        self.ram_offset = RAM_BANK_SIZE * self.ram_bank as usize;
      },
      _ => ()
    }
  }
}
