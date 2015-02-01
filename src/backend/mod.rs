use gameboy;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};
use machine::MachineMessage;

pub mod sdl;

pub trait Backend {
  type SHM: BackendSharedMemory;
  type Error;
  fn main_loop(self, SyncSender<BackendMessage>, Receiver<MachineMessage>) -> Result<(), sdl::BackendError>;
  fn shared_memory(&self) -> Arc<Self::SHM>;
}

pub trait BackendSharedMemory {
  fn draw_screen(&self, &gameboy::ScreenBuffer);
}

pub enum BackendMessage {
  KeyDown(GbKey), KeyUp(GbKey), Break, Step, Run, Turbo(bool)
}

#[derive(Debug)]
pub enum GbKey {
  Right, Left, Up, Down, A, B, Select, Start
}

pub fn init() -> Result<sdl::SdlBackend, sdl::BackendError> {
  sdl::SdlBackend::init()
}

pub fn new_channel() -> (SyncSender<BackendMessage>, Receiver<BackendMessage>) {
  sync_channel(128)
}
