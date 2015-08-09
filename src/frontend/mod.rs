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
use sdl2;
use sdl2::Sdl;
use sdl2::controller::{Axis, Button, GameController};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use std::convert::From;
use std::error::Error;
use std::iter;
use std::fmt;
use std::fmt::Display;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError, sync_channel};
use time::{Duration, precise_time_ns};

use gameboy;
use machine::MachineMessage;
use self::fps::FpsCounter;

mod fps;

pub enum FrontendMessage {
  KeyDown(GbKey), KeyUp(GbKey), Break, Step, Run, Turbo(bool), Quit
}

#[derive(Debug)]
pub enum GbKey {
  Right, Left, Up, Down, A, B, Select, Start
}

pub fn new_channel() -> (SyncSender<FrontendMessage>, Receiver<FrontendMessage>) {
  sync_channel(128)
}

pub struct SdlFrontend {
  sdl: Sdl,
  relative_speed_stat: f64,
  shared_memory: Arc<SharedMemory>
}

pub struct SharedMemory {
  pixel_buffer_lock: Mutex<Vec<u8>>,
  palette: Palette
}

impl SharedMemory {
  pub fn new() -> SharedMemory {
    SharedMemory {
      pixel_buffer_lock: Mutex::new(iter::repeat(0xff).take(PIXEL_BUFFER_SIZE).collect()),
      palette: Palette::from_colors(&PALETTE)
    }
  }
  pub fn draw_screen(&self, pixels: &gameboy::ScreenBuffer) {
    let mut data = self.pixel_buffer_lock.lock().unwrap();
    let ref palette = self.palette;
    for y in (0..gameboy::SCREEN_HEIGHT) {
      let in_start = y * gameboy::SCREEN_WIDTH;
      let in_end = in_start + gameboy::SCREEN_WIDTH;
      let in_slice = &pixels[in_start .. in_end];

      let out_start = y * PIXEL_BUFFER_STRIDE;
      let out_end = out_start + gameboy::SCREEN_WIDTH * 4;
      let out_slice = &mut data[out_start .. out_end];

      for (pixel, gb_color) in out_slice.chunks_mut(4).zip(in_slice.iter()) {
        let color = palette.get_bytes(gb_color);
        pixel[0] = color[0];
        pixel[1] = color[1];
        pixel[2] = color[2];
        pixel[3] = color[3];
      }
    }
  }
}

#[derive(Clone, Debug)]
pub enum FrontendError {
  Sdl(String)
}

pub type FrontendResult<T> = Result<T, FrontendError>;

impl From<String> for FrontendError {
  fn from(e: String) -> FrontendError {
    FrontendError::Sdl(e)
  }
}

impl Error for FrontendError {
  fn description(&self) -> &str {
    match *self {
      FrontendError::Sdl(..) => "SDL error"
    }
  }
}

impl Display for FrontendError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      FrontendError::Sdl(ref msg) => f.write_str(msg)
    }
  }
}

const PIXEL_BUFFER_ROWS: usize = gameboy::SCREEN_HEIGHT;
const PIXEL_BUFFER_STRIDE: usize = 256 * 4;
const PIXEL_BUFFER_SIZE: usize = PIXEL_BUFFER_STRIDE * PIXEL_BUFFER_ROWS;

struct Palette {
  colors: [[u8; 4]; 4]
}

impl Palette {
  fn from_colors(colors: &[Color; 4]) -> Palette {
    fn convert(color: &Color) -> [u8; 4] {
      match *color {
        Color::RGBA(r, g, b, a) => [a, b, g, r],
        _ => [0, 0, 0, 0]
      }
    }
    let colors = [
      convert(&colors[0]),
      convert(&colors[1]),
      convert(&colors[2]),
      convert(&colors[3])
    ];
    Palette {
      colors: colors
    }
  }
  fn get_bytes<'a>(&'a self, gb_color: &gameboy::Color) -> &'a [u8; 4] {
    match *gb_color {
      gameboy::Color::Off => &self.colors[0],
      gameboy::Color::Light => &self.colors[1],
      gameboy::Color::Dark => &self.colors[2],
      gameboy::Color::On => &self.colors[3]
    }
  }
}

static PALETTE: [Color; 4] =
  [
    Color::RGBA(255, 247, 123, 255),
    Color::RGBA(181, 174, 74,  255),
    Color::RGBA(107, 105, 49,  255),
    Color::RGBA(33,  32,  16,  255)
  ];

impl SdlFrontend {
  pub fn init() -> FrontendResult<SdlFrontend> {
    let sdl = try!(sdl2::init().video().game_controller().build());
    Ok(SdlFrontend {
      sdl: sdl,
      relative_speed_stat: 0.0,
      shared_memory: Arc::new(SharedMemory::new())
    })
  }
  fn refresh_gb_screen(&self, renderer: &mut Renderer, texture: &mut Texture) -> FrontendResult<()> {
    let rect = Rect::new_unwrap(0, 0, gameboy::SCREEN_WIDTH as u32, gameboy::SCREEN_HEIGHT as u32);
    {
      let pixels = self.shared_memory.pixel_buffer_lock.lock().unwrap();
      try!(texture.update(Some(rect), &pixels, PIXEL_BUFFER_STRIDE));
    }
    renderer.clear();
    try!(renderer.set_logical_size(gameboy::SCREEN_WIDTH as u32, gameboy::SCREEN_HEIGHT as u32));
    renderer.copy(&texture, Some(rect), Some(rect));
    Ok(())
  }
  fn present(&mut self, renderer: &mut Renderer, texture: &mut Texture) -> FrontendResult<()> {
    try!(self.refresh_gb_screen(renderer, texture));
    try!(renderer.set_logical_size(gameboy::SCREEN_WIDTH as u32 * 4, gameboy::SCREEN_HEIGHT as u32 * 4));

    renderer.present();
    Ok(())
  }
  pub fn main_loop(mut self, to_machine: SyncSender<FrontendMessage>, from_machine: Receiver<MachineMessage>) -> FrontendResult<()> {
    let window =
      try!(self.sdl.window("Mooneye GB", 640, 576).build());
    let mut renderer =
      try!(window.renderer().accelerated().present_vsync().build());
    renderer.clear();
    renderer.present();

    let mut controllers = vec!();

    let mut fps_counter = FpsCounter::new();
    let mut last_stats_time = precise_time_ns();

    let mut texture = try!(renderer.create_texture_streaming(PixelFormatEnum::RGBA8888, (256, 256)));

    'main: loop {
      match from_machine.try_recv() {
        Err(TryRecvError::Disconnected) => break,
        Ok(MachineMessage::RelativeSpeedStat(value)) => self.relative_speed_stat = value,
        _ => ()
      }

      for event in self.sdl.event_pump().poll_iter() {
        match event {
          Event::Quit{..} => break 'main,
          Event::KeyDown{keycode: Some(keycode), ..} if keycode == Keycode::Escape => break 'main,
          Event::KeyDown{keycode: Some(keycode), ..} => {
            match to_joypad_key(keycode) {
              Some(key) => to_machine.send(FrontendMessage::KeyDown(key)).unwrap(),
              None => ()
            }
            match keycode {
              Keycode::Home => to_machine.send(FrontendMessage::Break).unwrap(),
              Keycode::End => to_machine.send(FrontendMessage::Run).unwrap(),
              Keycode::PageDown => to_machine.send(FrontendMessage::Step).unwrap(),
              Keycode::LShift => to_machine.send(FrontendMessage::Turbo(true)).unwrap(),
              _ => ()
            }
          },
          Event::KeyUp{keycode: Some(keycode), ..} => {
            match to_joypad_key(keycode) {
              Some(key) => to_machine.send(FrontendMessage::KeyUp(key)).unwrap(),
              None => ()
            }
            match keycode {
              Keycode::LShift => to_machine.send(FrontendMessage::Turbo(false)).unwrap(),
              _ => ()
            }
          },
          Event::ControllerDeviceAdded{which: id, ..} => {
            controllers.push(try!(GameController::open(id)))
          },
          Event::ControllerButtonDown{button, ..} => {
            match controller_to_joypad_key(button) {
              Some(key) => to_machine.send(FrontendMessage::KeyDown(key)).unwrap(),
              None => ()
            }
          },
          Event::ControllerButtonUp{button, ..} => {
            match controller_to_joypad_key(button) {
              Some(key) => to_machine.send(FrontendMessage::KeyUp(key)).unwrap(),
              None => ()
            }
          },
          Event::ControllerAxisMotion{axis, value, ..} => {
            match controller_axis_to_message(axis, value) {
              Some(message) => to_machine.send(message).unwrap(),
              None => ()
            }
          },
          _ => ()
        }
      }

      match self.present(&mut renderer, &mut texture) {
        Err(error) => { println!("{}", error.description()); break },
        _ => ()
      }

      let frame_time = precise_time_ns();
      fps_counter.update();
      if Duration::nanoseconds((frame_time - last_stats_time) as i64) > Duration::seconds(2) {
        println!("FPS: {:.0}, speed: {:.0} %", fps_counter.fps, self.relative_speed_stat);
        last_stats_time = frame_time;
      }
    }
    Ok(())
  }
  pub fn shared_memory(&self) -> Arc<SharedMemory> {
    self.shared_memory.clone()
  }
}

fn to_joypad_key(key: Keycode) -> Option<GbKey> {
  match key {
    Keycode::Right => Some(GbKey::Right),
    Keycode::Left => Some(GbKey::Left),
    Keycode::Up => Some(GbKey::Up),
    Keycode::Down => Some(GbKey::Down),
    Keycode::Z => Some(GbKey::A),
    Keycode::X => Some(GbKey::B),
    Keycode::Return => Some(GbKey::Start),
    Keycode::Backspace => Some(GbKey::Select),
    _ => None
  }
}

fn controller_to_joypad_key(button: Button) -> Option<GbKey> {
  match button {
    Button::DPadRight => Some(GbKey::Right),
    Button::DPadLeft => Some(GbKey::Left),
    Button::DPadUp => Some(GbKey::Up),
    Button::DPadDown => Some(GbKey::Down),
    Button::A => Some(GbKey::B),
    Button::B => Some(GbKey::A),
    Button::Start => Some(GbKey::Start),
    Button::Back => Some(GbKey::Select),
    _ => None
  }
}

fn controller_axis_to_message(axis: Axis, value: i16) -> Option<FrontendMessage> {
  match axis {
    Axis::LeftX => match value {
      -32768...-16384 => Some(FrontendMessage::KeyDown(GbKey::Left)),
      -16383...-1 => Some(FrontendMessage::KeyUp(GbKey::Left)),
      0...16383 => Some(FrontendMessage::KeyUp(GbKey::Right)),
      16384...32767 => Some(FrontendMessage::KeyDown(GbKey::Right)),
      _ => None
    },
    Axis::LeftY => match value {
      -32768...-16384 => Some(FrontendMessage::KeyDown(GbKey::Up)),
      -16383...-1 => Some(FrontendMessage::KeyUp(GbKey::Up)),
      0...16383 => Some(FrontendMessage::KeyUp(GbKey::Down)),
      16384...32767 => Some(FrontendMessage::KeyDown(GbKey::Down)),
      _ => None
    },
    _ => None
  }
}

