use getopts::Fail;
use std::env;
use std::error::FromError;
use std::old_io::stdio;
use std::old_io::IoError;

#[derive(Debug)]
pub enum ProgramResult {
  Exit,
  Error(String)
}

impl FromError<IoError> for ProgramResult {
  fn from_error(err: IoError) -> ProgramResult {
    ProgramResult::Error(format!("IoError: {}", err))
  }
}

impl FromError<Fail> for ProgramResult {
  fn from_error(err: Fail) -> ProgramResult {
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

        let mut stderr = stdio::stderr();
        stderr.write_line(message.as_slice()).unwrap();
      }
    }
  }
}
