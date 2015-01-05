use gameboy;
use std::error::Error;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use machine::MachineMessage;

pub mod sdl;

pub trait Backend<C: BackendSharedMemory> {
  fn main_loop(&mut self, SyncSender<BackendMessage>, Receiver<MachineMessage>);
  fn shared_memory(&self) -> Arc<C>;
}

pub trait BackendSharedMemory {
  fn draw_screen(&self, &gameboy::ScreenBuffer);
}

pub enum BackendMessage {
  KeyDown(GbKey), KeyUp(GbKey), Break, Step, Run, Turbo(bool)
}

#[derive(Show)]
pub enum GbKey {
  Right, Left, Up, Down, A, B, Select, Start
}

pub fn init() -> Result<sdl::SdlBackend, Box<Error>> {
  match sdl::SdlBackend::init() {
    Ok(backend) => Ok(backend),
    Err(error) => Err(box error as Box<Error>)
  }
}

pub fn new_channel() -> (SyncSender<BackendMessage>, Receiver<BackendMessage>) {
  sync_channel(128)
}
