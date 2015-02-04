use std::fmt;
use std::fmt::{Debug, Formatter};
use std::old_io::fs::File;
use std::num::FromPrimitive;
use std::str;

use gameboy::{
  BootromData,
  BOOTROM_EMPTY, BOOTROM_SIZE,
  ROM_BANK_SIZE
};
use util::program_result::ProgramResult;

pub struct HardwareConfig {
  pub bootrom: Option<BootromData>,
  pub cartridge: CartridgeConfig
}

impl Debug for HardwareConfig {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    try!(writeln!(f, "Bootrom: {}", match self.bootrom {
      Some(_) => "yes",
      None => "no"
    }));
    write!(f, "{:?}", self.cartridge)
  }
}

pub struct CartridgeConfig {
  pub title: String,
  pub cartridge_type: CartridgeType,
  pub rom_size: CartridgeRomSize,
  pub ram_size: CartridgeRamSize,
  pub data: Vec<u8>
}

impl Debug for CartridgeConfig {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    try!(writeln!(f, "Title: {:?}", self.title));
    try!(writeln!(f, "Type: {:?}", self.cartridge_type));
    try!(writeln!(f, "ROM: {:?}", self.rom_size));
    writeln!(f, "RAM: {:?}", self.ram_size)
  }
}

#[derive(PartialEq, Eq, FromPrimitive)]
pub enum CartridgeType {
  Rom  = 0x00,  RomRam = 0x08,  RomRamBattery = 0x09,
  Mbc1 = 0x01, Mbc1Ram = 0x02, Mbc1RamBattery = 0x03,
  Mbc2 = 0x05,                 Mbc2RamBattery = 0x06
}

impl Debug for CartridgeType {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use self::CartridgeType::*;
    write!(f, "{}", match *self {
      Rom => "ROM ONLY", RomRam => "ROM+RAM",   RomRamBattery => "ROM+RAM+BATTERY",
      Mbc1 => "MBC1",   Mbc1Ram => "MBC1+RAM", Mbc1RamBattery => "MBC1+RAM+BATTERY",
      Mbc2 => "MBC2",                          Mbc2RamBattery => "MBC2+RAM+BATTERY"
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
    use self::CartridgeType::*;
    match *self {
      RomRam => true,
      RomRamBattery => true,
      Mbc1Ram => true,
      Mbc1RamBattery => true,
      _ => false
    }
  }

}

#[derive(PartialEq, Eq, FromPrimitive)]
pub enum CartridgeRomSize {
  NoRomBanks = 0x00,
  RomBanks4 = 0x01,
  RomBanks8 = 0x02,
  RomBanks16 = 0x03,
  RomBanks32 = 0x04,
  RomBanks64 = 0x05,
  RomBanks128 = 0x06,
  RomBanks256 = 0x07,
  RomBanks512 = 0x08
}

impl Debug for CartridgeRomSize {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use self::CartridgeRomSize::*;
    write!(f, "{}", match *self {
      NoRomBanks => "256 Kbit",
      RomBanks4 => "512 Kbit",
      RomBanks8 => "1 Mbit",
      RomBanks16 => "2 Mbit",
      RomBanks32 => "4 Mbit",
      RomBanks64 => "8 Mbit",
      RomBanks128 => "16 Mbit",
      RomBanks256 => "32 Mbit",
      RomBanks512 => "64 Mbit"
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
  pub fn banks(&self) -> usize {
    use self::CartridgeRomSize::*;
    match *self {
      NoRomBanks => 2,
      RomBanks4  => 4,
      RomBanks8  => 8,
      RomBanks16 => 16,
      RomBanks32 => 32,
      RomBanks64 => 64,
      RomBanks128 => 128,
      RomBanks256 => 256,
      RomBanks512 => 512
    }
  }
  pub fn as_usize(&self) -> usize { self.banks() * ROM_BANK_SIZE }
}

#[derive(PartialEq, Eq, FromPrimitive)]
pub enum CartridgeRamSize {
  NoRam = 0x00,
  Ram2K = 0x01,
  Ram8K = 0x02,
  Ram16K = 0x03,
  Ram128K = 0x04,
  Ram32K = 0x05
}

impl Debug for CartridgeRamSize {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use self::CartridgeRamSize::*;
    write!(f, "{}", match *self {
      NoRam => "-",
      Ram2K => "16 Kbit",
      Ram8K => "64 Kbit",
      Ram16K => "128 Kbit",
      Ram128K => "1 Mbit",
      Ram32K => "256 Kbit"
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
  pub fn as_usize(&self) -> usize {
    use self::CartridgeRamSize::*;
    match *self {
      NoRam => 0,
      Ram2K => 2048,
      Ram8K => 8192,
      Ram16K => 16384,
      Ram128K => 131072,
      Ram32K => 32768
    }
  }
}

impl CartridgeConfig {
  pub fn read(data: &[u8]) -> Result<CartridgeConfig, ProgramResult> {
    let new_cartridge = data[0x14b] == 0x33;

    let title = {
      let slice =
        if new_cartridge { &data[0x134 .. 0x13f] } else { &data[0x134 .. 0x143] };
      let utf8 = try!(str::from_utf8(slice).map_err(|_|{
        ProgramResult::Error("Invalid ROM title".to_string())
      }));

      utf8.trim_right_matches('\0').to_string()
    };

    let cartridge_type = try!(CartridgeType::from_u8(data[0x147]));
    let rom_size = try!(CartridgeRomSize::from_u8(data[0x148]));
    let ram_size = try!(CartridgeRamSize::from_u8(data[0x149]));

    if cartridge_type.should_have_ram() && ram_size == CartridgeRamSize::NoRam {
      return Err(ProgramResult::Error(format!("{:?} cartridge without ram", cartridge_type)))
    }
    if !cartridge_type.should_have_ram() && ram_size != CartridgeRamSize::NoRam {
      return Err(ProgramResult::Error(format!("{:?} cartridge with ram size {:02x}", cartridge_type, data[0x149])))
    }
    if data.len() != rom_size.as_usize() {
      return Err(ProgramResult::Error(format!("Expected {} bytes of cartridge ROM, got {:?}", rom_size.as_usize(), data.len())));
    }

    Ok(CartridgeConfig {
      title: title,
      cartridge_type: cartridge_type,
      rom_size: rom_size,
      ram_size: ram_size,
      data: data.to_vec(),
    })
  }
}

pub fn read_bootrom(path: &Path) -> Result<BootromData, ProgramResult> {
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
  let cartridge = try!(CartridgeConfig::read(&*cartridge_data));

  Ok(HardwareConfig {
    bootrom: bootrom,
    cartridge: cartridge
  })
}
