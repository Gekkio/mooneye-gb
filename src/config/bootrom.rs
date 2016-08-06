// This file is part of Mooneye GB.
// Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use crc::crc32;
use podio::ReadPodExt;
use std::convert::From;
use std::env;
use std::fmt;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

use gameboy::BOOTROM_SIZE;

static DEFAULT_BOOT_ROM_PRIORITY: [BootromType; 5] = [
  BootromType::Dmg,
  BootromType::Mgb,
  BootromType::Sgb2,
  BootromType::Sgb,
  BootromType::Dmg0,
];

#[derive(Debug)]
pub struct Bootrom {
  pub data: Vec<u8>,
  pub kind: BootromType
}

impl Bootrom {
  pub fn from_path(path: &Path) -> Result<Bootrom, BootromError> {
    let mut file = try!(File::open(path));
    let data = try!(file.read_exact(BOOTROM_SIZE));
    Bootrom::from_data(data)
  }
  pub fn from_data(data: Vec<u8>) -> Result<Bootrom, BootromError> {
    let checksum = crc32::checksum_ieee(&data);
    let kind = try!(match checksum {
      0xc2f5cc97 => Ok(BootromType::Dmg0),
      0x59c8598e => Ok(BootromType::Dmg),
      0xe6920754 => Ok(BootromType::Mgb),
      0xec8a83b9 => Ok(BootromType::Sgb),
      0x53d0dd63 => Ok(BootromType::Sgb2),
      checksum => Err(BootromError::Checksum(checksum))
    });
    Ok(Bootrom {
      data: data,
      kind: kind
    })
  }
  pub fn from_default_bootrom(priority: &[BootromType]) -> Option<Bootrom> {
    let mut candidates = vec![];
    let priority = if priority.is_empty() { &DEFAULT_BOOT_ROM_PRIORITY } else { priority };

    if let Ok(cwd) = env::current_dir() {
      for kind in priority {
        candidates.push(cwd.join(kind.file_name()));
      }
    }

    if let Some(home) = env::home_dir().map(|home| home.join(".mooneye-gb")) {
      for kind in priority {
        candidates.push(home.join(kind.file_name()));
      }
    }

    for path in candidates {
      match Bootrom::from_path(&path) {
        Err(BootromError::Io(ref e)) if e.kind() == io::ErrorKind::NotFound => (),
        Err(BootromError::Io(ref e)) =>
          println!("Warning: Boot rom \"{}\" ({})", path.to_string_lossy(), e),
        Err(BootromError::Checksum(ref e)) =>
          println!("Warning: Boot rom \"{}\" ({})", path.to_string_lossy(), e),
        Ok(bootrom) => return Some(bootrom)
      }
    }
    None
  }
  pub fn save_to_home(&self) -> Result<(), io::Error> {
    if let Some(home) = env::home_dir().map(|home| home.join(".mooneye-gb")) {
      match fs::create_dir(&home) {
        Err(e) => {
          if e.kind() != io::ErrorKind::AlreadyExists {
            return Err(e);
          }
        },
        _ => ()
      }
      let path = home.join(self.kind.file_name());
      let mut file = try!(File::create(&path));
      return file.write_all(&self.data)
    }
    Ok(())
  }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum BootromType {
  Dmg0, Dmg, Mgb, Sgb, Sgb2
}

impl BootromType {
  pub fn file_name(&self) -> &'static str {
    match *self {
      BootromType::Dmg0 => "dmg0_boot.bin",
      BootromType::Dmg => "dmg_boot.bin",
      BootromType::Mgb => "mgb_boot.bin",
      BootromType::Sgb => "sgb_boot.bin",
      BootromType::Sgb2 => "sgb2_boot.bin",
    }
  }
}

impl fmt::Display for BootromType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::BootromType::*;
    f.write_str(match *self {
      Dmg0 => "DMG (Game Boy), original version",
      Dmg => "DMG (Game Boy)",
      Mgb => "MGB (Game Boy Pocket)",
      Sgb => "SGB (Super Game Boy)",
      Sgb2 => "SGB2 (Super Game Boy 2)"
    })
  }
}

#[derive(Debug)]
pub enum BootromError {
  Io(io::Error),
  Checksum(u32)
}

impl fmt::Display for BootromError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::BootromError::*;
    match *self {
      Io(ref e) => write!(f, "{}", e),
      Checksum(crc32) => write!(f, "Unrecognized CRC32 checksum {}", crc32)
    }
  }
}

impl From<io::Error> for BootromError {
  fn from(e: io::Error) -> Self { BootromError::Io(e) }
}
