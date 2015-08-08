// This file is part of Mooneye GB.
// Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
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
