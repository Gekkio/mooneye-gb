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
use sdl2::{Sdl, EventPump};
use sdl2::controller::{Axis, Button};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Renderer, Texture};
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use time::{Duration, precise_time_ns};

use emulation::{EmuTime, MachineCycles, EE_VSYNC};
use gameboy;
use machine::{Machine, PerfCounter};
use self::fps::FpsCounter;

mod fps;

#[derive(Debug)]
pub enum GbKey {
  Right, Left, Up, Down, A, B, Select, Start
}

pub struct SdlFrontend {
  sdl: Sdl,
  event_pump: EventPump,
  pixel_buffer: Vec<u8>,
  palette: Palette
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
    let sdl = try!(sdl2::init());
    let event_pump = try!(sdl.event_pump());
    Ok(SdlFrontend {
      sdl: sdl,
      event_pump: event_pump,
      pixel_buffer: vec![0xff; PIXEL_BUFFER_SIZE],
      palette: Palette::from_colors(&PALETTE)
    })
  }
  fn refresh_gb_screen(&self, renderer: &mut Renderer, texture: &mut Texture) -> FrontendResult<()> {
    let rect = Rect::new_unwrap(0, 0, gameboy::SCREEN_WIDTH as u32, gameboy::SCREEN_HEIGHT as u32);
    {
      try!(texture.update(Some(rect), &self.pixel_buffer, PIXEL_BUFFER_STRIDE));
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
  pub fn main_loop_benchmark(mut self, mut machine: Machine, duration: Duration) -> FrontendResult<()> {
    let sdl_video = try!(self.sdl.video());

    let window =
      try!(sdl_video.window("Mooneye GB", 640, 576).build());
    let mut renderer =
      try!(window.renderer().accelerated().build());
    renderer.clear();
    renderer.present();

    let mut fps_counter = FpsCounter::new();
    let mut perf_counter = PerfCounter::new();
    let mut last_stats_time = precise_time_ns();

    let mut emu_time = EmuTime::zero();
    let start_time_ns = precise_time_ns();

    let mut texture = try!(renderer.create_texture_streaming(PixelFormatEnum::RGBA8888, (256, 256)));

    machine.reset();
    'main: loop {
      if Duration::nanoseconds((precise_time_ns() - start_time_ns) as i64) >= duration { break }

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit{..} => break 'main,
          Event::KeyDown{keycode: Some(keycode), ..} if keycode == Keycode::Escape => break 'main,
          _ => ()
        }
      }

      const PULSE_CYCLES: MachineCycles = MachineCycles(((gameboy::CPU_SPEED_HZ / 60) / 4) as u32);
      let target_time = emu_time + PULSE_CYCLES;
      loop {
        let (events, end_time) = machine.emulate(target_time);

        if events.contains(EE_VSYNC) {
          self.update_pixels(machine.screen_buffer());
        }

        perf_counter.update(end_time - emu_time);
        if end_time >= target_time {
          emu_time = end_time;
          break;
        }
      }

      match self.present(&mut renderer, &mut texture) {
        Err(error) => { println!("{}", error.description()); break },
        _ => ()
      }

      let frame_time = precise_time_ns();
      fps_counter.update();
      if Duration::nanoseconds((frame_time - last_stats_time) as i64) > Duration::seconds(2) {
        println!("FPS: {:.0}, speed: {:.0} %", fps_counter.fps, perf_counter.get_relative_speed());
        last_stats_time = frame_time;
      }
    }
    Ok(())
  }
  pub fn main_loop(mut self, mut machine: Machine) -> FrontendResult<()> {
    let sdl_video = try!(self.sdl.video());
    let sdl_game_controller = try!(self.sdl.game_controller());

    let window =
      try!(sdl_video.window("Mooneye GB", 640, 576).build());
    let mut renderer =
      try!(window.renderer().accelerated().present_vsync().build());
    renderer.clear();
    renderer.present();

    let mut fps_counter = FpsCounter::new();
    let mut perf_counter = PerfCounter::new();
    let mut last_stats_time = precise_time_ns();

    let mut emu_time = EmuTime::zero();
    let mut controllers = vec![];

    let mut last_frame = precise_time_ns();
    let mut turbo = false;

    let mut texture = try!(renderer.create_texture_streaming(PixelFormatEnum::RGBA8888, (256, 256)));

    machine.reset();
    'main: loop {
      let current_time = precise_time_ns();
      let delta = current_time - last_frame;
      last_frame = current_time;

      fps_counter.update();
      if Duration::nanoseconds((current_time - last_stats_time) as i64) > Duration::seconds(2) {
        println!("FPS: {:.0}, speed: {:.0} %", fps_counter.fps, perf_counter.get_relative_speed());
        last_stats_time = current_time;
      }

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit{..} => break 'main,
          Event::KeyDown{keycode: Some(keycode), ..} if keycode == Keycode::Escape => break 'main,
          Event::KeyDown{keycode: Some(keycode), ..} => {
            if let Some(key) = map_keycode(keycode) { machine.key_down(key) }
            if keycode == Keycode::LShift && !turbo {
              turbo = true;
              sdl_video.gl_set_swap_interval(0);
            }
          },
          Event::KeyUp{keycode: Some(keycode), ..} => {
            if let Some(key) = map_keycode(keycode) { machine.key_up(key) }
            if keycode == Keycode::LShift && turbo {
              turbo = false;
              sdl_video.gl_set_swap_interval(1);
            }
          },
          Event::ControllerDeviceAdded{which: id, ..} => {
            controllers.push(try!(sdl_game_controller.open(id as u32)))
          },
          Event::ControllerButtonDown{button, ..} => {
            if let Some(key) = map_button(button) { machine.key_down(key) }
          },
          Event::ControllerButtonUp{button, ..} => {
            if let Some(key) = map_button(button) { machine.key_up(key) }
          },
          Event::ControllerAxisMotion{axis, value, ..} => {
            if let Some((key, state)) = map_axis(axis, value) {
              if state { machine.key_down(key) } else { machine.key_up(key) }
            }
          },
          _ => ()
        }
      }

      let cycles =
        if turbo {
          MachineCycles((gameboy::CPU_SPEED_HZ / 4) as u32 / 60)
        } else {
          MachineCycles(((delta * (gameboy::CPU_SPEED_HZ / 4) as u64) / 1_000_000_000) as u32)
        };

      let target_time = emu_time + cycles;
      loop {
        let (events, end_time) = machine.emulate(target_time);

        if events.contains(EE_VSYNC) {
          self.update_pixels(machine.screen_buffer());
        }

        if end_time >= target_time {
          perf_counter.update(end_time - emu_time);
          emu_time = end_time;
          break;
        }
      }

      match self.present(&mut renderer, &mut texture) {
        Err(error) => { println!("{}", error.description()); break },
        _ => ()
      }
    }
    Ok(())
  }
  fn update_pixels(&mut self, pixels: &gameboy::ScreenBuffer) {
    let ref mut data = self.pixel_buffer;
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

fn map_keycode(key: Keycode) -> Option<GbKey> {
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

fn map_button(button: Button) -> Option<GbKey> {
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

fn map_axis(axis: Axis, value: i16) -> Option<(GbKey, bool)> {
  match axis {
    Axis::LeftX => match value {
      -32768...-16384 => Some((GbKey::Left, true)),
      -16383...-1 => Some((GbKey::Left, false)),
      0...16383 => Some((GbKey::Right, false)),
      16384...32767 => Some((GbKey::Right, true)),
      _ => None
    },
    Axis::LeftY => match value {
      -32768...-16384 => Some((GbKey::Up, true)),
      -16383...-1 => Some((GbKey::Up, false)),
      0...16383 => Some((GbKey::Down, false)),
      16384...32767 => Some((GbKey::Down, true)),
      _ => None
    },
    _ => None
  }
}

