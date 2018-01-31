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
use app_dirs::{AppDataType, AppInfo, app_dir, get_app_dir};
use crc::crc32;
use std::env;
use std::fs::File;
use std::io;
use std::io::{Read, Write};
use std::path::Path;

use gameboy::BootromData;
use errors::{MooneyeError, MooneyeErrorKind, MooneyeResult};
use config::{Model, DEFAULT_MODEL_PRIORITY};

const APP_INFO: AppInfo = AppInfo{name: "mooneye-gb", author: "Gekkio"};

pub struct Bootrom {
  pub model: Model,
  pub data: Box<BootromData>,
}

impl Bootrom {
  pub fn from_path(path: &Path) -> MooneyeResult<Bootrom> {
    let mut file = File::open(path)?;
    let mut data = Box::new(BootromData::new());
    file.read_exact(&mut data.0)?;
    Bootrom::from_data(data)
  }
  pub fn from_data(data: Box<BootromData>) -> MooneyeResult<Bootrom> {
    let checksum = crc32::checksum_ieee(&data.0);
    let model = match checksum {
      0xc2f5cc97 => Some(Model::Dmg0),
      0x59c8598e => Some(Model::Dmg),
      0xe6920754 => Some(Model::Mgb),
      0xec8a83b9 => Some(Model::Sgb),
      0x53d0dd63 => Some(Model::Sgb2),
      _ => None,
    };
    match model {
      Some(model) => Ok(Bootrom { model, data }),
      None => Err(MooneyeErrorKind::BootromChecksum(checksum).into())
    }
  }
  pub fn lookup(models: &[Model]) -> Option<Bootrom> {
    let mut candidates = vec![];
    let models = if models.is_empty() { &DEFAULT_MODEL_PRIORITY } else { models };

    if let Ok(dir) = get_app_dir(AppDataType::UserData, &APP_INFO, "bootroms") {
      for model in models {
        candidates.push(dir.join(model.bootrom_file_name()));
      }
    }

    if let Ok(cwd) = env::current_dir() {
      for model in models {
        candidates.push(cwd.join(model.bootrom_file_name()));
      }
    }

    for path in candidates {
      let path_str = path.to_string_lossy();
      debug!("Scanning {} for a boot ROM", path_str);
      match Bootrom::from_path(&path) {
        Err(MooneyeError(MooneyeErrorKind::Io(ref e), _))
          if e.kind() == io::ErrorKind::NotFound => (),
        Err(ref e @ MooneyeError(_, _)) => warn!("Warning: Boot rom \"{}\" ({})", path_str, e),
        Ok(bootrom) => {
          info!("Using {} boot ROM from {}", bootrom.model, path_str);
          return Some(bootrom)
        }
      }
    }
    None
  }
  pub fn save_to_data_dir(&self) -> MooneyeResult<()> {
    if let Ok(dir) = app_dir(AppDataType::UserData, &APP_INFO, "bootroms") {
      let path = dir.join(self.model.bootrom_file_name());
      let mut file = File::create(&path)?;
      try!(file.write_all(&self.data.0));
      info!("Saved {} boot ROM to {}", self.model, path.to_string_lossy());
    }
    Ok(())
  }
}
