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
use std::env;
use std::path::PathBuf;
use time::{Duration, precise_time_ns};

use config;
use emulation::{EmuTime, EE_DEBUG_OP, MachineCycles};
use gameboy;
use machine::Machine;

pub fn run_acceptance_test(name: &str) {
  let bootrom_path = env::home_dir().unwrap().join(".mooneye-gb").join("boot.bin");
  let test_name = format!("tests/build/{}.gb", name);
  let cartridge_path = PathBuf::from(&test_name);
  let hardware_config = config::create_hardware_config(Some(&bootrom_path), &cartridge_path).unwrap();

  let mut machine = Machine::new(hardware_config);
  machine.reset();

  let max_duration = Duration::seconds(30);
  let start_time = precise_time_ns();

  let mut emu_time = EmuTime::zero();

  let mut registers = None;
  loop {
    let time = precise_time_ns();
    if Duration::nanoseconds((time - start_time) as i64) > max_duration {
      break;
    }
    const PULSE_CYCLES: MachineCycles = MachineCycles((gameboy::CPU_SPEED_HZ / 4) as u32);
    let (events, end_time) = machine.emulate(emu_time + PULSE_CYCLES);
    emu_time = end_time;
    if events.contains(EE_DEBUG_OP) {
      registers = Some(machine.regs());
      break;
    }
  }
  match registers {
    None => panic!("Test did not finish"),
    Some(regs) => {
      if regs.a != 0 {
        panic!("{} assertion failures in hardware test", regs.a);
      }
      if regs.b != 3  || regs.c != 5  ||
         regs.d != 8  || regs.e != 13 ||
         regs.h != 21 || regs.l != 34 {
        panic!("Hardware test failed");
      }
    }
  }
}
