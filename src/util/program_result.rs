use getopts::Fail;
use std::convert::From;
use std::env;
use std::io;
use std::io::Write;

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
        env::set_exit_status(0)
      },
      ProgramResult::Error(ref message) => {
        env::set_exit_status(1);

        let mut stderr = io::stderr();
        stderr.write(message.as_bytes()).unwrap();
        stderr.write(b"\n").unwrap();
      }
    }
  }
}
