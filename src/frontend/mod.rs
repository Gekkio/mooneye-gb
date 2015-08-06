use std::sync::Arc;
use std::sync::mpsc::{Receiver, SyncSender, sync_channel};

use gameboy;
use machine::MachineMessage;

pub mod sdl;

pub trait Frontend {
  type SHM: FrontendSharedMemory;
  type Error;
  fn main_loop(self, SyncSender<FrontendMessage>, Receiver<MachineMessage>) -> Result<(), sdl::FrontendError>;
  fn shared_memory(&self) -> Arc<Self::SHM>;
}

pub trait FrontendSharedMemory {
  fn draw_screen(&self, &gameboy::ScreenBuffer);
}

pub enum FrontendMessage {
  KeyDown(GbKey), KeyUp(GbKey), Break, Step, Run, Turbo(bool), Quit
}

#[derive(Debug)]
pub enum GbKey {
  Right, Left, Up, Down, A, B, Select, Start
}

pub fn init() -> Result<sdl::SdlFrontend, sdl::FrontendError> {
  sdl::SdlFrontend::init()
}

pub fn new_channel() -> (SyncSender<FrontendMessage>, Receiver<FrontendMessage>) {
  sync_channel(128)
}
