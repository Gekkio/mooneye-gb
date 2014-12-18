use std::comm;
use std::error::Error;
use std::sync::Arc;
use machine::MachineMessage;

pub mod sdl;

pub trait Backend<C: BackendSharedMemory> {
  fn main_loop(&mut self, SyncSender<BackendMessage>, Receiver<MachineMessage>);
  fn shared_memory(&self) -> Arc<C>;
}

pub trait BackendSharedMemory {
  fn draw_scanline(&self, &[GbColor, ..160], y: u8);
}

pub enum BackendMessage {
  KeyDown(GbKey), KeyUp(GbKey), Break, Step, Run, Turbo(bool)
}

#[deriving(Show)]
pub enum GbKey {
  Right, Left, Up, Down, A, B, Select, Start
}

#[deriving(PartialEq, FromPrimitive, Copy)]
pub enum GbColor {
  Off = 0,
  Light = 1,
  Dark = 2,
  On = 3
}

impl GbColor {
  pub fn from_u8(value: u8) -> GbColor {
    FromPrimitive::from_u8(value).unwrap_or(GbColor::Off)
  }
}

pub fn init() -> Result<sdl::SdlBackend, Box<Error>> {
  match sdl::SdlBackend::init() {
    Ok(backend) => Ok(backend),
    Err(error) => Err(box error as Box<Error>)
  }
}

pub fn new_channel() -> (SyncSender<BackendMessage>, Receiver<BackendMessage>) {
  comm::sync_channel(128)
}
