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
