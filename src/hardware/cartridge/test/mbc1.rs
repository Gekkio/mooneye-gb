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
use config::{CartridgeConfig, CartridgeRamSize, CartridgeRomSize, CartridgeType};
use hardware::cartridge::Cartridge;

#[test]
fn test_mbc1() {
  let mut data = vec![0u8; 4 * 0x4000];
  for bank in 0u8..4 {
    let start = bank as usize * 0x4000;
    let end = start + 0x4000;
    for x in data[start..end].iter_mut() {
      *x = bank;
    }
  }

  let config = CartridgeConfig {
    title: "TEST".into(),
    cartridge_type: CartridgeType::Mbc1,
    rom_size: CartridgeRomSize::RomBanks4,
    ram_size: CartridgeRamSize::NoRam,
    data: data
  };
  let mut cart = Cartridge::new(config);

  cart.write_control(0x2000, 0x00);
  assert_eq!(cart.read_rom_bankx(0), 0x01);
  cart.write_control(0x2000, 0x01);
  assert_eq!(cart.read_rom_bankx(0), 0x01);
  cart.write_control(0x2000, 0x02);
  assert_eq!(cart.read_rom_bankx(0), 0x02);
  cart.write_control(0x2000, 0x03);
  assert_eq!(cart.read_rom_bankx(0), 0x03);
  cart.write_control(0x2000, 0x04);
  assert_eq!(cart.read_rom_bankx(0), 0x00);
  cart.write_control(0x2000, 0x05);
  assert_eq!(cart.read_rom_bankx(0), 0x01);
  cart.write_control(0x2000, 0x06);
  assert_eq!(cart.read_rom_bankx(0), 0x02);
  cart.write_control(0x2000, 0x07);
  assert_eq!(cart.read_rom_bankx(0), 0x03);
}
