use getopts::Options;
use std::env;
use std::ffi::OsString;

use util::program_result::ProgramResult;

pub struct CmdLine {
  pub bootrom_path: Option<Path>,
  pub cartridge_path: Path,
  pub benchmark: Option<String>
}

pub fn parse_cmdline() -> Result<CmdLine, ProgramResult> {
  let args: Vec<OsString> = env::args().collect();
  let program = args[0].to_str().unwrap_or("");

  let mut opts = Options::new();

  opts.optopt("b", "bootrom", "use boot rom", "FILE");
  opts.optopt("e", "benchmark", "run a benchmark", "seconds");
  opts.optflag("h", "help", "print help");

  let matches = try!(opts.parse(args.tail()));
  if matches.opt_present("h") {
    let short = opts.short_usage(program.as_slice());
    let brief = format!("{} CARTRIDGE_FILE", short);
    let long = opts.usage(brief.as_slice());
    print!("{}", long);
    return Err(ProgramResult::Exit);
  }
  let cartridge = match matches.free.as_slice().first() {
    Some(arg) => arg.clone(),
    None => {
      let short = opts.short_usage(program.as_slice());
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
