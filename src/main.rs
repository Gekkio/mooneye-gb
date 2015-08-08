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
#[macro_use]
extern crate bitflags;
extern crate getopts;
extern crate num;
extern crate podio;
extern crate sdl2;
extern crate snooze;
extern crate time;

#[cfg(test)]
extern crate quickcheck;

use std::env;
use std::fs;
use std::thread;
use time::Duration;

use cmdline::CmdLine;
use config::HardwareConfig;
use frontend::Frontend;
use machine::Machine;
use util::program_result::ProgramResult;

mod cmdline;
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

fn parse_seconds(text: &str) -> Result<Duration, ProgramResult> {
  let seconds = try!(text.parse().map_err(
      |_| ProgramResult::Error(format!("Invalid duration {}", text))
  ));
  Ok(Duration::seconds(seconds))
}

struct MiscConfig {
  benchmark_duration: Option<Duration>
}

fn prepare_emulator() -> Result<(HardwareConfig, MiscConfig), ProgramResult> {
  let cmdline = try!(CmdLine::parse_env_args());

  let home_dir = env::home_dir().map(|home| {
    home.join(".mooneye-gb")
  });

  let bootrom_default = home_dir.and_then(|home| {
    let path = home.join("boot.bin");
    if fs::metadata(&path).is_ok() { Some(path) } else { None }
  });

  let mut benchmark_duration = None;
  if let Some(text) = cmdline.benchmark {
    benchmark_duration = Some(try!(parse_seconds(&text)))
  }

  let bootrom_path = cmdline.bootrom_path.or(bootrom_default);
  let cartridge_path = cmdline.cartridge_path;

  let hw_config = try!(
    config::create_hardware_config(bootrom_path.as_ref().map(|x| x.as_path()), &cartridge_path));

  Ok((hw_config,
    MiscConfig { benchmark_duration: benchmark_duration }))
}

fn main() {
  let (hardware_config, misc_config) = match prepare_emulator() {
    Ok(configs) => configs,
    Err(result) => {
      result.apply();
      return;
    }
  };

  println!("{:?}", hardware_config);

  let frontend = match frontend::init() {
    Err(error) => panic!("{}", error),
    Ok(frontend) => frontend
  };

  let shared_memory = frontend.shared_memory();
  let (frontend_tx, frontend_rx) = frontend::new_channel();
  let (machine_tx, machine_rx) = machine::new_channel();

  thread::spawn(move || {
    let mut mach = Machine::new(&*shared_memory, hardware_config);
    let channels = machine::Channels::new(machine_tx, frontend_rx);

    match misc_config.benchmark_duration {
      Some(duration) => { mach.main_benchmark(channels, duration); },
      None => { mach.main_loop(channels); }
    }
  });

  if let Err(error) = frontend.main_loop(frontend_tx, machine_rx) {
    panic!("{}", error);
  }
}
