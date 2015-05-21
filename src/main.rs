#[macro_use]
extern crate bitflags;
extern crate clock_ticks;
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

use backend::Backend;
use cmdline::CmdLine;
use config::HardwareConfig;
use machine::Machine;
use util::program_result::ProgramResult;

mod backend;
mod cmdline;
mod config;
mod cpu;
mod emulation;
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

  let backend = match backend::init() {
    Err(error) => panic!("{}", error),
    Ok(backend) => backend
  };

  let shared_memory = backend.shared_memory();
  let (backend_tx, backend_rx) = backend::new_channel();
  let (machine_tx, machine_rx) = machine::new_channel();

  thread::spawn(move || {
    let mut mach = Machine::new(&*shared_memory, hardware_config);
    let channels = machine::Channels::new(machine_tx, backend_rx);

    match misc_config.benchmark_duration {
      Some(duration) => { mach.main_benchmark(channels, duration); },
      None => { mach.main_loop(channels); }
    }
  });

  if let Err(error) = backend.main_loop(backend_tx, machine_rx) {
    panic!("{}", error);
  }
}
