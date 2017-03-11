// This file is part of Mooneye GB.
// Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
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
extern crate app_dirs;
#[macro_use]
extern crate bitflags;
extern crate crc;
extern crate docopt;
#[macro_use]
extern crate glium;
extern crate glium_sdl2;
#[macro_use]
extern crate imgui;
#[macro_use]
extern crate log;
extern crate nalgebra;
extern crate num_traits;
extern crate podio;
extern crate rustc_serialize;
extern crate sdl2;
extern crate simplelog;
extern crate url;

#[cfg(test)]
extern crate quickcheck;

use docopt::Docopt;
use simplelog::{LogLevelFilter, TermLogger};
use std::path::Path;
use std::process;

use config::{Bootrom, Cartridge, Model};
use frontend::SdlFrontend;

mod config;
mod cpu;
mod emulation;
mod frontend;
mod gameboy;
mod hardware;
mod machine;
mod util;

#[cfg(feature = "acceptance_tests")]
mod acceptance_tests;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const USAGE: &'static str = concat!("Mooneye GB v", env!("CARGO_PKG_VERSION"), "

Usage:
  mooneye-gb [options] [<rom>]
  mooneye-gb (-h | --help)

Options:
  -h, --help               Help
  -m MODEL, --model MODEL  Emulate a specific Game Boy model.
                           Valid values: dmg0, dmg, mgb, sgb, sgb2.
  -b FILE, --bootrom FILE  Use a boot ROM
");

#[derive(Debug, RustcDecodable)]
struct Args {
  arg_rom: Option<String>,
  flag_bootrom: Option<String>,
  flag_model: Option<Model>
}

fn read_boot_rom(path: &str, expected_model: Option<Model>) -> Bootrom {
  let bootrom = Bootrom::from_path(&Path::new(path)).unwrap_or_else(|err| {
    error!("Failed to read boot rom from \"{}\" ({})", path, err);
    process::exit(1)
  });
  if let Some(model) = expected_model {
    if model != bootrom.model {
      warn!("Warning: boot ROM is not for {}", model);
    }
  }
  bootrom
}

fn main() {
  let args: Args =
    Docopt::new(USAGE)
    .and_then(|d| d.decode())
    .unwrap_or_else(|e| e.exit());

  TermLogger::init(LogLevelFilter::Debug, simplelog::Config::default())
    .expect("Failed to initialize logging");

  let bootrom =
    match (args.flag_model, args.flag_bootrom) {
      (_, Some(path)) => Some(read_boot_rom(&path, args.flag_model)),
      (Some(model), None) => {
        let result = Bootrom::lookup(&[model]);
        if result.is_none() {
          error!("Could not find a boot rom for {}", model);
          process::exit(1)
        }
        result
      },
      (None, None) => Bootrom::lookup(&[])
    };

  let cartridge =
    args.arg_rom.map(|path| {
      Cartridge::from_path(&Path::new(&path)).unwrap_or_else(|err| {
        error!("Failed to read rom from \"{}\" ({})", path, err);
        process::exit(1)
      })
    });

  let frontend = match SdlFrontend::init() {
    Err(error) => panic!("{}", error),
    Ok(frontend) => frontend
  };

  let result = frontend.main(bootrom, cartridge);
  if let Err(error) = result {
    panic!("{}", error);
  }
}
