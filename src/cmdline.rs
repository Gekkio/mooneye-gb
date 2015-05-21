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
