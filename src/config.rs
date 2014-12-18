use gameboy::{
  BootromData,
  BOOTROM_EMPTY, BOOTROM_SIZE,
  ROM_BANK_SIZE
};
use std::fmt;
use std::fmt::{Formatter, Show};
use std::io::fs::File;

use util::program_result::ProgramResult;

pub struct HardwareConfig {
  pub bootrom: Option<BootromData>,
  pub cartridge: CartridgeConfig
}

impl Show for HardwareConfig {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    try!(writeln!(f, "Bootrom: {}", match self.bootrom {
      Some(_) => "yes",
      None => "no"
    }));
    write!(f, "{}", self.cartridge)
  }
}

pub struct CartridgeConfig {
  pub title: String,
  pub cartridge_type: CartridgeType,
  pub rom_size: CartridgeRomSize,
  pub ram_size: CartridgeRamSize,
  pub data: Vec<u8>
}

impl Show for CartridgeConfig {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    try!(writeln!(f, "Title: {}", self.title));
    try!(writeln!(f, "Type: {}", self.cartridge_type));
    try!(writeln!(f, "ROM: {}", self.rom_size));
    writeln!(f, "RAM: {}", self.ram_size)
  }
}

#[deriving(PartialEq, Eq, FromPrimitive)]
pub enum CartridgeType {
  RomOnly = 0x00,
  Mbc1 = 0x01,
  Mbc1Ram = 0x02,
  Mbc1RamBattery = 0x03
}

impl Show for CartridgeType {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      CartridgeType::RomOnly => "ROM ONLY",
      CartridgeType::Mbc1 => "MBC1",
      CartridgeType::Mbc1Ram => "MBC1+RAM",
      CartridgeType::Mbc1RamBattery => "MBC1+RAM+BATTERY"
    })
  }
}

impl CartridgeType {
  fn from_u8(value: u8) -> Result<CartridgeType, ProgramResult> {
    match FromPrimitive::from_u8(value) {
      Some(result) => Ok(result),
      _ => Err(ProgramResult::Error(format!("Unsupported cartridge type {:02x}", value)))
    }
  }
  fn should_have_ram(&self) -> bool {
    match *self {
      CartridgeType::RomOnly => false,
      CartridgeType::Mbc1 => false,
      CartridgeType::Mbc1Ram => true,
      CartridgeType::Mbc1RamBattery => true
    }
  }

}

#[deriving(PartialEq, Eq, FromPrimitive)]
pub enum CartridgeRomSize {
  NoRomBanks = 0x00,
  RomBanks4 = 0x01,
  RomBanks8 = 0x02,
  RomBanks16 = 0x03,
  RomBanks32 = 0x04
}

impl Show for CartridgeRomSize {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use self::CartridgeRomSize::*;
    write!(f, "{}", match *self {
      NoRomBanks => "256 Kbit",
      RomBanks4 => "512 Kbit",
      RomBanks8 => "1 Mbit",
      RomBanks16 => "2 Mbit",
      RomBanks32 => "4 Mbit"
    })
  }
}

impl CartridgeRomSize {
  fn from_u8(value: u8) -> Result<CartridgeRomSize, ProgramResult> {
    match FromPrimitive::from_u8(value) {
      Some(result) => Ok(result),
      _ => Err(ProgramResult::Error(format!("Unsupported rom size {:02x}", value)))
    }
  }
  pub fn as_uint(&self) -> uint {
    use self::CartridgeRomSize::*;
    match *self {
      NoRomBanks => ROM_BANK_SIZE * 2,
      RomBanks4  => ROM_BANK_SIZE * 4,
      RomBanks8  => ROM_BANK_SIZE * 8,
      RomBanks16 => ROM_BANK_SIZE * 16,
      RomBanks32 => ROM_BANK_SIZE * 32,
    }
  }
}

#[deriving(PartialEq, Eq, FromPrimitive)]
pub enum CartridgeRamSize {
  NoRam = 0x00,
  Ram2K = 0x01,
  Ram8K = 0x02
}

impl Show for CartridgeRamSize {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{}", match *self {
      CartridgeRamSize::NoRam => "-",
      CartridgeRamSize::Ram2K => "16 Kbit",
      CartridgeRamSize::Ram8K => "64 Kbit"
    })
  }
}

impl CartridgeRamSize {
  fn from_u8(value: u8) -> Result<CartridgeRamSize, ProgramResult> {
    match FromPrimitive::from_u8(value) {
      Some(result) => Ok(result),
      _ => Err(ProgramResult::Error(format!("Unsupported ram size {:02x}", value)))
    }
  }
}

impl CartridgeConfig {
  fn read(data: Vec<u8>) -> Result<CartridgeConfig, ProgramResult> {
    let new_cartridge = data[0x14b] == 0x33;

    let title = {
      let slice =
        if new_cartridge { data.slice(0x134, 0x13f) } else { data.slice(0x134, 0x143) };

      match slice.to_ascii_opt() {
        Some(text) => text.as_str_ascii().trim_right_chars('\0').into_string(),
        None => return Err(ProgramResult::Error("Invalid ROM title".to_string()))
      }
    };

    let cartridge_type = try!(CartridgeType::from_u8(data[0x147]));
    let rom_size = try!(CartridgeRomSize::from_u8(data[0x148]));
    let ram_size = try!(CartridgeRamSize::from_u8(data[0x149]));

    if cartridge_type.should_have_ram() && ram_size == CartridgeRamSize::NoRam {
      return Err(ProgramResult::Error(format!("{} cartridge without ram", cartridge_type)))
    }
    if !cartridge_type.should_have_ram() && ram_size != CartridgeRamSize::NoRam {
      return Err(ProgramResult::Error(format!("{} cartridge with ram size {:02x}", cartridge_type, data[0x149])))
    }
    if data.len() != rom_size.as_uint() {
      return Err(ProgramResult::Error(format!("Expected {} bytes of cartridge ROM, got {}", rom_size.as_uint(), data.len())));
    }

    Ok(CartridgeConfig {
      title: title,
      cartridge_type: cartridge_type,
      rom_size: rom_size,
      ram_size: ram_size,
      data: data,
    })
  }
}

fn read_bootrom(path: &Path) -> Result<BootromData, ProgramResult> {
  let mut file = File::open(path);
  let mut buf = BOOTROM_EMPTY;

  let data = try!(file.read_exact(BOOTROM_SIZE));

  buf.move_from(data, 0, BOOTROM_SIZE);

  return Ok(buf);
}

pub fn create_hardware_config(bootrom_path: Option<&Path>, cartridge_path: &Path) -> Result<HardwareConfig, ProgramResult> {
  let mut bootrom = None;
  for path in bootrom_path.iter() {
    bootrom = Some(try!(read_bootrom(*path)))
  }

  let mut cartridge_file = File::open(cartridge_path);
  let cartridge_data = try!(cartridge_file.read_to_end());
  let cartridge = try!(CartridgeConfig::read(cartridge_data));

  Ok(HardwareConfig {
    bootrom: bootrom,
    cartridge: cartridge
  })
}
