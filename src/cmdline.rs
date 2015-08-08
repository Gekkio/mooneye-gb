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
use getopts::Options;
use std::env;
use std::path::PathBuf;

use util::program_result::ProgramResult;

pub struct CmdLine {
  pub bootrom_path: Option<PathBuf>,
  pub cartridge_path: PathBuf,
  pub benchmark: Option<String>
}

impl CmdLine {
  pub fn parse_env_args() -> Result<CmdLine, ProgramResult> {
    let args: Vec<String> = env::args().collect();
    let program = &args[0];

    let mut opts = Options::new();

    opts.optopt("b", "bootrom", "use boot rom", "FILE");
    opts.optopt("e", "benchmark", "run a benchmark", "seconds");
    opts.optflag("h", "help", "print help");

    let matches = try!(opts.parse(&args[1..]));
    if matches.opt_present("h") {
      let short = opts.short_usage(&program);
      let brief = format!("{} CARTRIDGE_FILE", short);
      let long = opts.usage(&brief);
      print!("{}", long);
      return Err(ProgramResult::Exit);
    }

    if let Some(ref cartridge) = matches.free.first() {
      Ok(CmdLine {
        bootrom_path: matches.opt_str("b").as_ref().map(PathBuf::from),
        cartridge_path: PathBuf::from(cartridge),
        benchmark: matches.opt_str("e")
      })
    } else {
      let message = format!("Missing cartridge file\n\
                            Try '{} --help' for more information", program);
      Err(ProgramResult::Error(message))
    }
  }
}
