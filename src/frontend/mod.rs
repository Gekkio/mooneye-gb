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
use glium::{Api, GliumCreationError, Surface, SwapBuffersError, Version};
use glium_sdl2::{Display, DisplayBuild};
use sdl2;
use sdl2::{Sdl, EventPump, VideoSubsystem};
use sdl2::controller::{Axis, Button};
use sdl2::event::{Event, WindowEventId};
use sdl2::keyboard::Keycode;
use sdl2::video::gl_attr::GLAttr;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use time::{Duration, SteadyTime};
use url::Url;

use config::{Bootrom, Cartridge, HardwareConfig};
use emulation::{EmuTime, MachineCycles, EE_VSYNC};
use gameboy;
use machine::{Machine, PerfCounter};
use self::fps::FpsCounter;
use self::gui::Gui;
use self::renderer::Renderer;

mod fps;
mod gui;
mod renderer;

#[derive(Debug)]
pub enum GbKey {
  Right, Left, Up, Down, A, B, Select, Start
}

pub struct SdlFrontend {
  sdl: Sdl,
  sdl_video: VideoSubsystem,
  event_pump: EventPump,
  display: Display,
  gui: Gui,
  renderer: Renderer
}

enum FrontendState {
  WaitBootrom(Option<Cartridge>),
  WaitRom(Option<Bootrom>),
  InGame(HardwareConfig),
  Exit
}

impl FrontendState {
  pub fn from_roms(bootrom: Option<Bootrom>, cartridge: Option<Cartridge>) -> FrontendState {
    use self::FrontendState::*;
    match (bootrom, cartridge) {
      (Some(bootrom), Some(cartridge)) => InGame((Some(bootrom), cartridge)),
      (None, Some(cartridge)) => WaitBootrom(Some(cartridge)),
      (Some(bootrom), None) => WaitRom(Some(bootrom)),
      _ => WaitBootrom(None)
    }
  }
}

#[derive(Clone, Debug)]
pub enum FrontendError {
  Sdl(String),
  Renderer(String)
}

pub type FrontendResult<T> = Result<T, FrontendError>;

impl From<String> for FrontendError {
  fn from(e: String) -> FrontendError {
    FrontendError::Sdl(e)
  }
}

impl From<GliumCreationError<String>> for FrontendError {
  fn from(e: GliumCreationError<String>) -> FrontendError {
    FrontendError::Renderer(format!("{:?}", e))
  }
}

impl From<SwapBuffersError> for FrontendError {
  fn from(e: SwapBuffersError) -> FrontendError {
    FrontendError::Renderer(format!("{:?}", e))
  }
}

impl Error for FrontendError {
  fn description(&self) -> &str {
    match *self {
      FrontendError::Sdl(ref msg) => msg,
      FrontendError::Renderer(ref msg) => msg
    }
  }
}

impl fmt::Display for FrontendError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      FrontendError::Sdl(ref msg) => f.write_str(msg),
      FrontendError::Renderer(ref msg) => f.write_str(msg)
    }
  }
}

struct FrameTimes {
  last_time: SteadyTime
}

impl FrameTimes {
  pub fn new() -> FrameTimes {
    let now = SteadyTime::now();
    FrameTimes {
      last_time: now
    }
  }
  pub fn update(&mut self) -> Duration {
    let now = SteadyTime::now();
    let delta = now - self.last_time;
    self.last_time = now;
    delta
  }
}

impl SdlFrontend {
  pub fn init() -> FrontendResult<SdlFrontend> {
    let sdl = try!(sdl2::init());
    let sdl_video = try!(sdl.video());
    configure_gl_attr(&mut sdl_video.gl_attr());

    let event_pump = try!(sdl.event_pump());

    let display =
      try!(sdl_video.window("Mooneye GB", 640, 576).opengl().position_centered().build_glium());

    println!("Initialized renderer with {}", match *display.get_opengl_version() {
      Version(Api::Gl, major, minor) => format!("OpenGL {}.{}", major, minor),
      Version(Api::GlEs, major, minor) => format!("OpenGL ES {}.{}", major, minor)
    });

    let renderer = try!(Renderer::new(&display));
    let gui = try!(Gui::init(&display));

    Ok(SdlFrontend {
      sdl: sdl,
      sdl_video: sdl_video,
      event_pump: event_pump,
      display: display,
      gui: gui,
      renderer: renderer
    })
  }
  pub fn main(mut self, bootrom: Option<Bootrom>, cartridge: Option<Cartridge>) -> FrontendResult<()> {
    let mut state = FrontendState::from_roms(bootrom, cartridge);
    loop {
      state =
        match state {
          FrontendState::WaitBootrom(cartridge) => try!(self.main_wait_bootrom(cartridge)),
          FrontendState::WaitRom(bootrom) => try!(self.main_wait_rom(bootrom)),
          FrontendState::InGame(config) => try!(self.main_in_game(config)),
          FrontendState::Exit => break
        }
    }
    Ok(())
  }
  fn main_wait_bootrom(&mut self, cartridge: Option<Cartridge>) -> FrontendResult<FrontendState> {
    self.sdl_video.gl_set_swap_interval(1);
    let mut times = FrameTimes::new();

    let mut scene = gui::WaitBootromScene::default();

    'main: loop {
      let delta = times.update();

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit{..} => break 'main,
          Event::KeyDown{keycode: Some(keycode), ..} if keycode == Keycode::Escape => break 'main,
          Event::DropFile{filename, ..} => {
            let result = resolve_sdl_filename(filename)
              .and_then(|path| Bootrom::from_path(&path).map_err(|e| format!("{}", e)));
            match result {
              Ok(bootrom) => {
                if let Err(error) = bootrom.save_to_home() {
                  println!("Failed to save boot rom: {}", error);
                }
                return Ok(FrontendState::from_roms(Some(bootrom), cartridge))
              },
              Err(e) => scene.set_error(format!("{}", e))
            };
          },
          _ => ()
        }
      }
      let mut target = self.display.draw();
      target.clear_color(1.0, 1.0, 1.0, 1.0);
      try!(self.gui.render(&mut target, delta, &self.sdl.mouse(), &mut scene));
      try!(target.finish());
    }
    Ok(FrontendState::Exit)
  }
  fn main_wait_rom(&mut self, bootrom: Option<Bootrom>) -> FrontendResult<FrontendState> {
    self.sdl_video.gl_set_swap_interval(1);
    let mut times = FrameTimes::new();

    let mut scene = gui::WaitRomScene::new();

    'main: loop {
      let delta = times.update();

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit{..} => break 'main,
          Event::KeyDown{keycode: Some(keycode), ..} if keycode == Keycode::Escape => break 'main,
          Event::DropFile{filename, ..} => {
            let result = resolve_sdl_filename(filename)
              .and_then(|path| Cartridge::from_path(&path));
            match result {
              Ok(cartridge) => return Ok(FrontendState::from_roms(bootrom, Some(cartridge))),
              Err(e) => scene.set_error(format!("{}", e))
            };
          },
          _ => ()
        }
      }
      let mut target = self.display.draw();
      target.clear_color(1.0, 1.0, 1.0, 1.0);
      try!(self.gui.render(&mut target, delta, &self.sdl.mouse(), &mut scene));
      try!(target.finish());
    }
    Ok(FrontendState::Exit)
  }
  fn main_in_game(&mut self, config: HardwareConfig) -> FrontendResult<FrontendState> {
    let mut machine = Machine::new(config);
    let sdl_game_controller = try!(self.sdl.game_controller());

    let mut fps_counter = FpsCounter::new();
    let mut perf_counter = PerfCounter::new();

    let mut scene = gui::InGameScene::default();

    let mut emu_time = EmuTime::zero();
    let mut controllers = vec![];

    let mut turbo = false;
    let mut times = FrameTimes::new();

    'main: loop {
      let delta = times.update();

      fps_counter.update(times.last_time);
      scene.fps = fps_counter.get_fps();
      scene.perf = 100.0 * perf_counter.get_cps() / gameboy::CPU_SPEED_HZ as f64;

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit{..} => break 'main,
          Event::Window { win_event_id: WindowEventId::SizeChanged, ..} => {
            self.renderer.update_dimensions(&self.display);
          },
          Event::KeyDown{keycode: Some(keycode), ..} if keycode == Keycode::Escape => break 'main,
          Event::KeyDown{keycode: Some(keycode), ..} => {
            if let Some(key) = map_keycode(keycode) { machine.key_down(key) }
            if keycode == Keycode::LShift && !turbo {
              turbo = true;
              self.sdl_video.gl_set_swap_interval(0);
            }
            if keycode == Keycode::F2 {
              scene.toggle_perf_overlay();
            }
          },
          Event::KeyUp{keycode: Some(keycode), ..} => {
            if let Some(key) = map_keycode(keycode) { machine.key_up(key) }
            if keycode == Keycode::LShift && turbo {
              turbo = false;
              self.sdl_video.gl_set_swap_interval(1);
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
          MachineCycles((delta * (gameboy::CPU_SPEED_HZ as i32 / 4)).num_seconds() as u32)
        };

      let target_time = emu_time + cycles;
      loop {
        let (events, end_time) = machine.emulate(target_time);

        if events.contains(EE_VSYNC) {
          self.renderer.update_pixels(machine.screen_buffer());
        }

        if end_time >= target_time {
          perf_counter.update(end_time - emu_time, times.last_time);
          emu_time = end_time;
          break;
        }
      }

      let mut target = self.display.draw();
      target.clear_color(0.0, 0.0, 0.0, 1.0);
      try!(self.renderer.draw(&mut target));
      try!(self.gui.render(&mut target, delta, &self.sdl.mouse(), &mut scene));
      try!(target.finish());
    }
    Ok(FrontendState::Exit)
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

fn resolve_sdl_filename(filename: String) -> Result<PathBuf, String> {
  let mut url = "file://".to_owned();
  url.push_str(&filename);
  Url::parse(&url).map_err(|e| format!("{}", e))
    .and_then(|url| url.to_file_path().map_err(|_| "Failed to parse path".to_owned()))
}

#[cfg(not(target_os = "macos"))]
fn configure_gl_attr<'a>(_: &mut GLAttr<'a>) { }

#[cfg(target_os = "macos")]
fn configure_gl_attr<'a>(gl_attr: &mut GLAttr<'a>) {
  use sdl2::video::GLProfile;
  gl_attr.set_context_major_version(3);
  gl_attr.set_context_minor_version(2);
  gl_attr.set_context_profile(GLProfile::Core);
  gl_attr.set_context_flags().forward_compatible().set();
}
