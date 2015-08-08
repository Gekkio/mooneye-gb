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
use getopts::Fail;
use std::convert::From;
use std::io;
use std::io::{Write, stderr};
use std::process;

#[derive(Debug)]
pub enum ProgramResult {
  Exit,
  Error(String)
}

impl From<io::Error> for ProgramResult {
  fn from(err: io::Error) -> ProgramResult {
    ProgramResult::Error(format!("IO error: {}", err))
  }
}

impl From<Fail> for ProgramResult {
  fn from(err: Fail) -> ProgramResult {
    ProgramResult::Error(format!("Fail: {}", err))
  }
}

impl ProgramResult {
  pub fn apply(&self) {
    match *self {
      ProgramResult::Exit => {
        process::exit(0);
      },
      ProgramResult::Error(ref message) => {
        let mut stderr = stderr();
        writeln!(&mut stderr, "{}", message).unwrap();

        process::exit(1);
      }
    }
  }
}
