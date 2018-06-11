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
extern crate app_dirs;
extern crate arrayvec;
#[macro_use]
extern crate bitflags;
extern crate crc;
extern crate failure;
#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate log;
extern crate num_traits;
extern crate url;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
extern crate quickcheck;

pub mod config;
mod cpu;
pub mod emulation;
pub mod gameboy;
mod hardware;
pub mod machine;
mod util;

#[cfg(feature = "acceptance_tests")]
mod acceptance_tests;

#[derive(Debug)]
pub enum GbKey {
  Right, Left, Up, Down, A, B, Select, Start
}

pub use gameboy::*;
