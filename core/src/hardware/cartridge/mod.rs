// This file is part of Mooneye GB.
// Copyright (C) 2014-2018 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use config;
use config::CartridgeType;
use crc::crc32;
use gameboy::{RAM_BANK_SIZE, ROM_BANK_SIZE};
use util::int::IntExt;

#[derive(Debug, Clone)]
struct Mbc1State {
  bank1: u8,
  bank2: u8,
  mode: bool,
}

impl Mbc1State {
  fn rom_offsets(&self, multicart: bool) -> (usize, usize) {
    let upper_bits = if multicart {
      self.bank2 << 4
    } else {
      self.bank2 << 5
    };
    let lower_bits = if multicart {
      self.bank1 & 0b1111
    } else {
      self.bank1
    };

    let lower_bank = if self.mode { upper_bits as usize } else { 0b00 };
    let upper_bank = (upper_bits | lower_bits) as usize;
    (ROM_BANK_SIZE * lower_bank, ROM_BANK_SIZE * upper_bank)
  }
  fn ram_offset(&self) -> usize {
    let bank = if self.mode { self.bank2 as usize } else { 0b00 };
    (RAM_BANK_SIZE * bank)
  }
}

fn is_mbc1_multicart(rom: &[u8]) -> bool {
  // Only 8 Mbit MBC1 multicarts exist. Since it's not clear how other ROM sizes would be wired,
  // it's pointless to try to support them
  if rom.len() != 1_048_576 {
    return false;
  }

  let nintendo_logo_count = (0..4)
    .map(|page| {
      let start = page * 0x40000 + 0x0104;
      let end = start + 0x30;

      crc32::checksum_ieee(&rom[start..end])
    })
    .filter(|&checksum| checksum == 0x46195417)
    .count();

  // A multicart should have at least two games + a menu with valid logo data
  nintendo_logo_count >= 3
}

#[derive(Debug, Clone)]
enum Mbc {
  None,
  Mbc1 { multicart: bool, state: Mbc1State },
  Mbc2,
  Mbc3,
}

impl Mbc {
  fn from_cartridge_type(t: CartridgeType, rom: &[u8]) -> Mbc {
    use config::CartridgeType::*;
    match t {
      Rom | RomRam | RomRamBattery => Mbc::None,
      Mbc1 | Mbc1Ram | Mbc1RamBattery => Mbc::Mbc1 {
        multicart: is_mbc1_multicart(rom),
        state: Mbc1State {
          bank1: 0b00001,
          bank2: 0b00,
          mode: false,
        },
      },
      Mbc2 | Mbc2RamBattery => Mbc::Mbc2,
      Mbc3 | Mbc3Ram | Mbc3RamBattery => Mbc::Mbc3,
    }
  }
}

pub struct Cartridge {
  mbc: Mbc,
  rom: Box<[u8]>,
  rom_offsets: (usize, usize),
  rom_bank: u8,
  ram: Box<[u8]>,
  ram_offset: usize,
  ram_bank: u8,
  ram_accessible: bool,
}

impl Cartridge {
  pub fn new(config: config::Cartridge) -> Cartridge {
    let mbc = Mbc::from_cartridge_type(config.cartridge_type, &config.data);
    let ram_size = match mbc {
      Mbc::Mbc2 => 512,
      _ => config.ram_size.as_usize(),
    };
    Cartridge {
      mbc,
      rom: config.data.into_boxed_slice(),
      rom_bank: 0,
      rom_offsets: (0x0000, 0x4000),
      ram: vec![0; ram_size].into_boxed_slice(),
      ram_offset: 0x0000,
      ram_bank: 0,
      ram_accessible: false,
    }
  }

  pub fn read_rom_bank0(&self, addr: u16) -> u8 {
    let (rom_lower, _) = self.rom_offsets;
    self.rom[(rom_lower | (addr as usize & 0x3fff)) & (self.rom.len() - 1)]
  }
  pub fn read_rom_bankx(&self, addr: u16) -> u8 {
    let (_, rom_upper) = self.rom_offsets;
    self.rom[(rom_upper | (addr as usize & 0x3fff)) & (self.rom.len() - 1)]
  }
  pub fn write_control(&mut self, reladdr: u16, value: u8) {
    match self.mbc {
      Mbc::None => (),
      Mbc::Mbc1 {
        multicart,
        ref mut state,
      } => match reladdr >> 8 {
        0x00...0x1f => {
          self.ram_accessible = (value & 0b1111) == 0b1010;
        }
        0x20...0x3f => {
          state.bank1 = if value & 0b11111 == 0b00000 {
            0b00001
          } else {
            value & 0b11111
          };
          self.rom_offsets = state.rom_offsets(multicart);
        }
        0x40...0x5f => {
          state.bank2 = value & 0b11;
          self.rom_offsets = state.rom_offsets(multicart);
          self.ram_offset = state.ram_offset();
        }
        0x60...0x7f => {
          state.mode = (value & 0b1) == 0b1;
          self.rom_offsets = state.rom_offsets(multicart);
          self.ram_offset = state.ram_offset();
        }
        _ => (),
      },
      Mbc::Mbc2 => match reladdr >> 8 {
        0x00...0x1f => {
          if !reladdr.bit_bool(8) {
            self.ram_accessible = (value & 0x0f) == 0x0a;
          }
        }
        0x20...0x3f => {
          if reladdr.bit_bool(8) {
            self.rom_bank = value & 0x0f;
            self.update_rom_offsets();
          }
        }
        _ => (),
      },
      Mbc::Mbc3 => match reladdr >> 8 {
        0x00...0x1f => {
          self.ram_accessible = (value & 0x0f) == 0x0a;
        }
        0x20...0x3f => {
          self.rom_bank = value & 0x7f;
          self.update_rom_offsets();
        }
        0x40...0x5f => {
          self.ram_bank = value & 0x07;
          self.update_ram_offset();
        }
        _ => (),
      },
    }
  }
  pub fn read_ram(&self, addr: u16) -> u8 {
    if self.ram_accessible && !self.ram.is_empty() {
      let addr = self.ram_addr(addr);
      self.ram[addr]
    } else {
      0xff
    }
  }
  pub fn write_ram(&mut self, addr: u16, value: u8) {
    if self.ram_accessible && !self.ram.is_empty() {
      let addr = self.ram_addr(addr);
      match self.mbc {
        Mbc::Mbc2 => {
          self.ram[addr] = value & 0x0f;
        }
        _ => {
          self.ram[addr] = value;
        }
      }
    }
  }
  fn ram_addr(&self, addr: u16) -> usize {
    (self.ram_offset | (addr as usize & 0x1fff)) & (self.ram.len() - 1)
  }
  fn update_rom_offsets(&mut self) {
    match self.mbc {
      Mbc::Mbc2 | Mbc::Mbc3 => {
        self.rom_offsets = (0x0000, ROM_BANK_SIZE * self.rom_bank as usize);
      }
      _ => (),
    }
  }
  fn update_ram_offset(&mut self) {
    match self.mbc {
      Mbc::Mbc3 => {
        self.ram_offset = RAM_BANK_SIZE * self.ram_bank as usize;
      }
      _ => (),
    }
  }
}
