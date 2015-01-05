use getopts::Fail;
use std::error::FromError;
use std::io::stdio;
use std::io::IoError;
use std::os;
use std::str::SendStr;

#[derive(Show)]
pub enum ProgramResult {
  Exit,
  Error(String)
}

impl<'a> FromError<SendStr> for ProgramResult {
  fn from_error(err: SendStr) -> ProgramResult {
    ProgramResult::Error(format!("SendStr: {}", err))
  }
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
        os::set_exit_status(0)
      },
      ProgramResult::Error(ref message) => {
        os::set_exit_status(1);

        let mut stderr = stdio::stderr();
        stderr.write_line(message.as_slice()).unwrap();
      }
    }
  }
}
