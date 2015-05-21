use getopts::Fail;
use std::convert::From;
use std::env;
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
