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
use std::path::PathBuf;
use time::{Duration, SteadyTime};

use config::{Bootrom, Cartridge, HardwareConfig, Model};
use config::DEFAULT_MODEL_PRIORITY;
use emulation::{EmuDuration, EmuTime, EE_DEBUG_OP};
use gameboy;
use machine::Machine;

pub fn run_test_with_model(name: &str, model: Model) {
  let bootrom = Bootrom::lookup(&[model])
    .unwrap_or_else(|| panic!("No boot ROM found ({:?})", model));

  let test_name = format!("tests/build/{}.gb", name);
  let cartridge_path = PathBuf::from(&test_name);
  let cartridge = Cartridge::from_path(&cartridge_path).unwrap();

  let hardware_config = HardwareConfig {
    model: model,
    bootrom: Some(bootrom.data),
    cartridge: cartridge
  };

  let max_duration = Duration::seconds(60);
  let start_time = SteadyTime::now();
  let pulse_duration = EmuDuration::clock_cycles(gameboy::CPU_SPEED_HZ as u32);

  let mut machine = Machine::new(hardware_config);
  let mut registers = None;
  let mut emu_time = EmuTime::zero();
  loop {
    let time = SteadyTime::now();
    if time - start_time > max_duration {
      break;
    }
    let (events, end_time) = machine.emulate(emu_time + pulse_duration);
    emu_time = end_time;
    if events.contains(EE_DEBUG_OP) {
      registers = Some(machine.regs());
      break;
    }
  }
  match registers {
    None => panic!("Test did not finish ({:?})", model),
    Some(regs) => {
      if regs.a != 0 {
        panic!("{} assertion failures in hardware test ({:?})", regs.a, model);
      }
      if regs.b != 3  || regs.c != 5  ||
         regs.d != 8  || regs.e != 13 ||
         regs.h != 21 || regs.l != 34 {
        panic!("Hardware test failed ({:?})", model);
      }
    }
  }
}

pub fn run_test_with_models(name: &str, models: &[Model]) {
  for &model in models {
    run_test_with_model(name, model);
  }
}

pub fn run_test(name: &str) {
  run_test_with_models(name, &DEFAULT_MODEL_PRIORITY);
}
