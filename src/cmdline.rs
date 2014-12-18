use getopts::{getopts, optflag, optopt, short_usage, usage};
use std::os;

use util::program_result::ProgramResult;

pub struct CmdLine {
  pub bootrom_path: Option<Path>,
  pub cartridge_path: Path,
  pub benchmark: Option<String>
}

pub fn parse_cmdline() -> Result<CmdLine, ProgramResult> {
  let args = os::args();
  let program = args[0].as_slice();

  let opts = [
    optopt("b", "bootrom", "use boot rom", "FILE"),
    optopt("e", "benchmark", "run a benchmark", "seconds"),
    optflag("h", "help", "print help")
  ];
  let matches = try!(getopts(args.tail(), &opts));
  if matches.opt_present("h") {
    let short = short_usage(program, &opts);
    let brief = format!("{} CARTRIDGE_FILE", short);
    let long = usage(brief.as_slice(), &opts);
    print!("{}", long);
    return Err(ProgramResult::Exit);
  }
  let cartridge = match matches.free.as_slice().head() {
    Some(arg) => arg.clone(),
    None => {
      let short = short_usage(program, &opts);
      let message = format!("Missing cartridge file\n\
                            {} CARTRIDGE_FILE", short);
      return Err(ProgramResult::Error(message));
    }
  };

  Ok(CmdLine {
    bootrom_path: matches.opt_str("b").map(Path::new),
    cartridge_path: Path::new(cartridge),
    benchmark: matches.opt_str("e")
  })
}
