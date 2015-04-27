use std::fmt;
use std::fmt::{Debug, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;
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

#[derive(PartialEq, Eq)]
pub enum CartridgeType {
  Rom  = 0x00,  RomRam = 0x08,  RomRamBattery = 0x09,
  Mbc1 = 0x01, Mbc1Ram = 0x02, Mbc1RamBattery = 0x03,
  Mbc2 = 0x05,                 Mbc2RamBattery = 0x06,
  Mbc3 = 0x11, Mbc3Ram = 0x12, Mbc3RamBattery = 0x13
}

impl Debug for CartridgeType {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    use self::CartridgeType::*;
    write!(f, "{}", match *self {
      Rom => "ROM ONLY", RomRam => "ROM+RAM",   RomRamBattery => "ROM+RAM+BATTERY",
      Mbc1 => "MBC1",   Mbc1Ram => "MBC1+RAM", Mbc1RamBattery => "MBC1+RAM+BATTERY",
      Mbc2 => "MBC2",                          Mbc2RamBattery => "MBC2+RAM+BATTERY",
      Mbc3 => "MBC3",   Mbc3Ram => "MBC3+RAM", Mbc3RamBattery => "MBC3+RAM+BATTERY"
    })
  }
}

impl CartridgeType {
  fn from_u8(value: u8) -> Option<CartridgeType> {
    use self::CartridgeType::*;
    match value {
      0x00 => Some(Rom),
      0x08 => Some(RomRam),
      0x09 => Some(RomRamBattery),
      0x01 => Some(Mbc1),
      0x02 => Some(Mbc1Ram),
      0x03 => Some(Mbc1RamBattery),
      0x05 => Some(Mbc2),
      0x06 => Some(Mbc2RamBattery),
      0x11 => Some(Mbc3),
      0x12 => Some(Mbc3Ram),
      0x13 => Some(Mbc3RamBattery),
      _ => None
    }
  }
  fn should_have_ram(&self) -> bool {
    use self::CartridgeType::*;
    match *self {
       RomRam => true,  RomRamBattery => true,
      Mbc1Ram => true, Mbc1RamBattery => true,
      Mbc3Ram => true, Mbc3RamBattery => true,
      _ => false
    }
  }

}

#[derive(PartialEq, Eq)]
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
  fn from_u8(value: u8) -> Option<CartridgeRomSize> {
    use self::CartridgeRomSize::*;
    match value {
      0x00 => Some(NoRomBanks),
      0x01 => Some(RomBanks4),
      0x02 => Some(RomBanks8),
      0x03 => Some(RomBanks16),
      0x04 => Some(RomBanks32),
      0x05 => Some(RomBanks64),
      0x06 => Some(RomBanks128),
      0x07 => Some(RomBanks256),
      0x08 => Some(RomBanks512),
      _ => None
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

#[derive(PartialEq, Eq)]
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
  fn from_u8(value: u8) -> Option<CartridgeRamSize> {
    use self::CartridgeRamSize::*;
    match value {
      0x00 => Some(NoRam),
      0x01 => Some(Ram2K),
      0x02 => Some(Ram8K),
      0x03 => Some(Ram16K),
      0x04 => Some(Ram128K),
      0x05 => Some(Ram32K),
      _ => None
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

    let cartridge_type = try!(CartridgeType::from_u8(data[0x147]).ok_or_else(||{
      ProgramResult::Error(format!("Unsupported cartridge type {:02x}", data[0x147]))
    }));
    let rom_size = try!(CartridgeRomSize::from_u8(data[0x148]).ok_or_else(||{
      ProgramResult::Error(format!("Unsupported rom size {:02x}", data[0x148]))
    }));
    let ram_size = try!(CartridgeRamSize::from_u8(data[0x149]).ok_or_else(||{
      ProgramResult::Error(format!("Unsupported ram size {:02x}", data[0x149]))
    }));

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
  let mut file = try!(File::open(path));
  let mut bootrom_data = BOOTROM_EMPTY;

  let mut buffer = vec!();

  try!(file.read_to_end(&mut buffer));

  bootrom_data.move_from(buffer, 0, BOOTROM_SIZE);

  return Ok(bootrom_data);
}

pub fn create_hardware_config(bootrom_path: Option<&Path>, cartridge_path: &Path) -> Result<HardwareConfig, ProgramResult> {
  let mut bootrom = None;
  for path in bootrom_path.iter() {
    bootrom = Some(try!(read_bootrom(*path)))
  }

  let mut cartridge_file = try!(File::open(cartridge_path));
  let mut cartridge_data = vec!();
  try!(cartridge_file.read_to_end(&mut cartridge_data));
  let cartridge = try!(CartridgeConfig::read(&*cartridge_data));

  Ok(HardwareConfig {
    bootrom: bootrom,
    cartridge: cartridge
  })
}
