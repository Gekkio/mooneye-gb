// This file is part of Mooneye GB.
// Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use glium_sdl2::{Display, DisplayBuild, GliumSdl2Error};
use imgui::ImGui;
use imgui_glium_renderer;
use sdl2;
use sdl2::{Sdl, EventPump, VideoSubsystem};
use sdl2::controller::{Axis, Button};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::gl_attr::GLAttr;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::path::PathBuf;
use std::thread;
use std::time::{Duration, Instant};
use url::Url;

use config::{Bootrom, Cartridge, HardwareConfig};
use emulation::{EmuTime, EmuDuration, EE_VSYNC};
use gameboy;
use machine::{Machine, PerfCounter};
use self::fps::FpsCounter;
use self::gui::{Screen};
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
  imgui: ImGui,
  gui_renderer: imgui_glium_renderer::Renderer,
  renderer: Renderer,
  times: FrameTimes
}

enum FrontendState {
  WaitBootrom(Option<Cartridge>),
  InGame(HardwareConfig),
  Exit
}

impl FrontendState {
  pub fn from_roms(bootrom: Option<Bootrom>, cartridge: Option<Cartridge>) -> FrontendState {
    use self::FrontendState::*;
    match (bootrom, cartridge) {
      (Some(bootrom), Some(cartridge)) => InGame(HardwareConfig {
        model: bootrom.model,
        bootrom: Some(bootrom.data),
        cartridge: cartridge
      }),
      (None, Some(cartridge)) => WaitBootrom(Some(cartridge)),
      (Some(bootrom), None) => InGame(HardwareConfig {
        model: bootrom.model,
        bootrom: Some(bootrom.data),
        cartridge: Cartridge::no_cartridge(),
      }),
      _ => WaitBootrom(None)
    }
  }
}

#[derive(Clone, Debug)]
pub enum FrontendError {
  Sdl(String),
  Renderer(String),
  Other(String)
}

pub type FrontendResult<T> = Result<T, FrontendError>;

impl From<sdl2::IntegerOrSdlError> for FrontendError {
  fn from(e: sdl2::IntegerOrSdlError) -> FrontendError {
    FrontendError::Sdl(format!("{:?}", e))
  }
}

impl From<GliumCreationError<GliumSdl2Error>> for FrontendError {
  fn from(e: GliumCreationError<GliumSdl2Error>) -> FrontendError {
    FrontendError::Renderer(format!("{:?}", e))
  }
}

impl From<SwapBuffersError> for FrontendError {
  fn from(e: SwapBuffersError) -> FrontendError {
    FrontendError::Renderer(format!("{:?}", e))
  }
}

impl From<String> for FrontendError {
  fn from(e: String) -> FrontendError {
    FrontendError::Other(e)
  }
}

impl Error for FrontendError {
  fn description(&self) -> &str {
    use self::FrontendError::*;
    match *self {
      Sdl(ref msg) | Renderer(ref msg) | Other(ref msg) => msg
    }
  }
}

impl fmt::Display for FrontendError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use self::FrontendError::*;
    match *self {
      Sdl(ref msg) | Renderer(ref msg) | Other(ref msg) => f.write_str(msg)
    }
  }
}

struct FrameTimes {
  frame_duration: Duration,
  last_time: Instant,
  target_time: Instant
}

impl FrameTimes {
  pub fn new(frame_duration: Duration) -> FrameTimes {
    let now = Instant::now();
    FrameTimes {
      frame_duration: frame_duration,
      last_time: now,
      target_time: now + frame_duration
    }
  }
  pub fn reset(&mut self) {
    let now = Instant::now();
    self.last_time = now;
    self.target_time = now + self.frame_duration;
  }
  pub fn update(&mut self) -> Duration {
    let now = Instant::now();
    let delta = now - self.last_time;
    self.last_time = now;
    self.target_time += self.frame_duration;
    delta
  }
  pub fn limit(&self) {
    let now = Instant::now();
    if now < self.target_time {
      thread::sleep(self.target_time - now);
    }
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

    info!("Initialized renderer with {}", match *display.get_opengl_version() {
      Version(Api::Gl, major, minor) => format!("OpenGL {}.{}", major, minor),
      Version(Api::GlEs, major, minor) => format!("OpenGL ES {}.{}", major, minor)
    });

    let renderer = try!(Renderer::new(&display));

    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);
    let gui_renderer = try!(imgui_glium_renderer::Renderer::init(&mut imgui, &display));

    Ok(SdlFrontend {
      sdl: sdl,
      sdl_video: sdl_video,
      event_pump: event_pump,
      display: display,
      imgui: imgui,
      gui_renderer: gui_renderer,
      renderer: renderer,
      times: FrameTimes::new(Duration::from_secs(1) / 60)
    })
  }
  pub fn main(mut self, bootrom: Option<Bootrom>, cartridge: Option<Cartridge>) -> FrontendResult<()> {
    let mut state = FrontendState::from_roms(bootrom, cartridge);
    loop {
      state =
        match state {
          FrontendState::WaitBootrom(cartridge) => try!(self.main_wait_bootrom(cartridge)),
          FrontendState::InGame(config) => try!(self.main_in_game(config)),
          FrontendState::Exit => break
        }
    }
    Ok(())
  }
  fn main_wait_bootrom(&mut self, cartridge: Option<Cartridge>) -> FrontendResult<FrontendState> {
    self.sdl_video.gl_set_swap_interval(1);

    let mut screen = gui::WaitBootromScreen::default();

    self.times.reset();

    'main: loop {
      let delta = self.times.update();

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit{..} => break 'main,
          Event::KeyDown{keycode: Some(keycode), ..} if keycode == Keycode::Escape => break 'main,
          Event::MouseMotion{x, y, ..} => { self.imgui.set_mouse_pos(x as f32, y as f32) },
          Event::DropFile{filename, ..} => {
            let result = resolve_sdl_filename(filename)
              .and_then(|path| Bootrom::from_path(&path).map_err(|e| format!("{}", e)));
            match result {
              Ok(bootrom) => {
                if let Err(error) = bootrom.save_to_data_dir() {
                  println!("Failed to save boot rom: {}", error);
                }
                return Ok(FrontendState::from_roms(Some(bootrom), cartridge))
              },
              Err(e) => screen.set_error(format!("{}", e))
            };
          },
          _ => ()
        }
      }
      let mut target = self.display.draw();
      target.clear_color(1.0, 1.0, 1.0, 1.0);

      let delta_s = delta.as_secs() as f32 / 1_000_000_000.0;
      let (width, height) = target.get_dimensions();

      let ui = self.imgui.frame((width, height), (width, height), delta_s);
      screen.render(&ui);
      try!(self.gui_renderer.render(&mut target, ui));
      try!(target.finish());

      self.times.limit();
    }
    Ok(FrontendState::Exit)
  }
  fn main_in_game(&mut self, config: HardwareConfig) -> FrontendResult<FrontendState> {
    let mut screen = gui::InGameScreen::new(&config);
    let mut machine = Machine::new(config.clone());
    let sdl_game_controller = try!(self.sdl.game_controller());

    let mut fps_counter = FpsCounter::new();
    let mut perf_counter = PerfCounter::new();

    let mut emu_time = EmuTime::zero();
    let mut controllers = vec![];

    let mut turbo = false;
    self.times.reset();

    'main: loop {
      let delta = self.times.update();

      fps_counter.update(self.times.last_time);
      screen.fps = fps_counter.get_fps();
      screen.perf =
        100.0 * perf_counter.get_clock_edges_per_s() / 2.0 / gameboy::CPU_SPEED_HZ as f64;

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit{..} => break 'main,
          Event::Window { win_event: WindowEvent::SizeChanged(..), ..} => {
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
              screen.toggle_info_overlay();
            }
          },
          Event::MouseMotion{x, y, ..} => { self.imgui.set_mouse_pos(x as f32, y as f32) },
          Event::KeyUp{keycode: Some(keycode), ..} => {
            if let Some(key) = map_keycode(keycode) { machine.key_up(key) }
            if keycode == Keycode::LShift && turbo {
              turbo = false;
              self.times.reset();
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
          Event::DropFile{filename, ..} => {
            let result = resolve_sdl_filename(filename)
              .and_then(|path| Cartridge::from_path(&path));
            match result {
              Ok(cartridge) => return Ok(FrontendState::InGame(HardwareConfig {
                cartridge: cartridge,
                ..config
              })),
              Err(e) => println!("{}", e)
            };
          },
          _ => ()
        }
      }

      let mut target = self.display.draw();
      target.clear_color(0.0, 0.0, 0.0, 1.0);

      let delta_s = delta.as_secs() as f32 / 1_000_000_000.0;
      let (width, height) = target.get_dimensions();
      let ui = self.imgui.frame((width, height), (width, height), delta_s);

      let clock_edges =
        if turbo {
          EmuDuration::clock_cycles(gameboy::CPU_SPEED_HZ as u32 / 60)
        } else {
          EmuDuration::clock_cycles((delta * gameboy::CPU_SPEED_HZ as u32).as_secs() as u32)
        };

      let target_time = emu_time + clock_edges;
      loop {
        let (events, end_time) = machine.emulate(target_time);

        if events.contains(EE_VSYNC) {
          self.renderer.update_pixels(machine.screen_buffer());
        }

        if end_time >= target_time {
          perf_counter.update(end_time - emu_time, self.times.last_time);
          emu_time = end_time;
          break;
        }
      }
      try!(self.renderer.draw(&mut target));

      screen.render(&ui);
      try!(self.gui_renderer.render(&mut target, ui));
      try!(target.finish());

      if !turbo {
        self.times.limit();
      }
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
fn configure_gl_attr(_: &mut GLAttr) { }

#[cfg(target_os = "macos")]
fn configure_gl_attr(gl_attr: &mut GLAttr) {
  use sdl2::video::GLProfile;
  gl_attr.set_context_major_version(3);
  gl_attr.set_context_minor_version(2);
  gl_attr.set_context_profile(GLProfile::Core);
  gl_attr.set_context_flags().forward_compatible().set();
}
