// This file is part of Mooneye GB.
// Copyright (C) 2014-2018 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use failure::Error;
use glium::{Api, Surface, Version};
use glium_sdl2::{Display, DisplayBuild};
use imgui::{FrameSize, ImGui};
use imgui_glium_renderer;
use sdl2;
use sdl2::controller::{Axis, Button};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::video::gl_attr::GLAttr;
use sdl2::{EventPump, GameControllerSubsystem};
use std::path::PathBuf;
use std::time::Duration;
use url::Url;

use self::gui::Screen;
use self::renderer::Renderer;
use fps_counter::FpsCounter;
use frame_times::FrameTimes;
use mooneye_gb::config::{Bootrom, Cartridge, HardwareConfig};
use mooneye_gb::emulation::{EmuEvents, EmuTime};
use mooneye_gb::machine::Machine;
use mooneye_gb::*;
use perf_counter::PerfCounter;

mod emu_thread;
mod gui;
mod renderer;

pub struct SdlFrontend {
  sdl_game_controller: GameControllerSubsystem,
  event_pump: EventPump,
  display: Display,
  imgui: ImGui,
  gui_renderer: imgui_glium_renderer::Renderer,
  renderer: Renderer,
  times: FrameTimes,
}

enum FrontendState {
  WaitBootrom(Option<Cartridge>),
  InGame(InGameState),
  InGameTurbo(InGameState),
  Exit,
}

struct InGameState {
  config: HardwareConfig,
  machine: Machine,
  screen: gui::InGameScreen,
  fps_counter: FpsCounter,
  perf_counter: PerfCounter,
}

impl InGameState {
  pub fn from_config(config: HardwareConfig) -> InGameState {
    let machine = Machine::new(config.clone());
    let screen = gui::InGameScreen::new(&config);
    let fps_counter = FpsCounter::new();
    let perf_counter = PerfCounter::new();
    InGameState {
      config,
      machine,
      screen,
      fps_counter,
      perf_counter,
    }
  }
}

impl FrontendState {
  pub fn from_roms(bootrom: Option<Bootrom>, cartridge: Option<Cartridge>) -> FrontendState {
    use self::FrontendState::*;
    match (bootrom, cartridge) {
      (Some(bootrom), Some(cartridge)) => InGame(InGameState::from_config(HardwareConfig {
        model: bootrom.model,
        bootrom: Some(bootrom.data),
        cartridge,
      })),
      (None, Some(cartridge)) => WaitBootrom(Some(cartridge)),
      (Some(bootrom), None) => InGame(InGameState::from_config(HardwareConfig {
        model: bootrom.model,
        bootrom: Some(bootrom.data),
        cartridge: Cartridge::no_cartridge(),
      })),
      _ => WaitBootrom(None),
    }
  }
}

impl SdlFrontend {
  pub fn init() -> Result<SdlFrontend, Error> {
    let sdl = sdl2::init().map_err(|msg| format_err!("SDL2 initialization failed: {}", msg))?;
    let sdl_video = sdl
      .video()
      .map_err(|msg| format_err!("SDL2 video initialization failed: {}", msg))?;
    configure_gl_attr(&mut sdl_video.gl_attr());

    let sdl_game_controller = sdl
      .game_controller()
      .map_err(|msg| format_err!("SDL2 game controller initialization failure: {}", msg))?;

    let event_pump = sdl
      .event_pump()
      .map_err(|msg| format_err!("SDL2 event pump failure: {}", msg))?;

    let display = sdl_video
      .window("Mooneye GB", 640, 576)
      .opengl()
      .position_centered()
      .build_glium()?;

    info!(
      "Initialized renderer with {}",
      match *display.get_opengl_version() {
        Version(Api::Gl, major, minor) => format!("OpenGL {}.{}", major, minor),
        Version(Api::GlEs, major, minor) => format!("OpenGL ES {}.{}", major, minor),
      }
    );

    let renderer = Renderer::new(&display)?;

    let mut imgui = ImGui::init();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);
    let gui_renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &display)
      .map_err(|e| format_err!("Failed to initialize renderer: {}", e))?;

    Ok(SdlFrontend {
      sdl_game_controller,
      event_pump,
      display,
      imgui,
      gui_renderer,
      renderer,
      times: FrameTimes::new(Duration::from_secs(1) / 60),
    })
  }
  pub fn main(
    mut self,
    bootrom: Option<Bootrom>,
    cartridge: Option<Cartridge>,
  ) -> Result<(), Error> {
    let mut state = FrontendState::from_roms(bootrom, cartridge);
    loop {
      state = match state {
        FrontendState::WaitBootrom(cartridge) => self.main_wait_bootrom(cartridge)?,
        FrontendState::InGame(state) => self.main_in_game(state)?,
        FrontendState::InGameTurbo(state) => self.main_in_game_turbo(state)?,
        FrontendState::Exit => break,
      }
    }
    Ok(())
  }
  fn main_wait_bootrom(&mut self, cartridge: Option<Cartridge>) -> Result<FrontendState, Error> {
    let mut screen = gui::WaitBootromScreen::default();

    self.times.reset();

    'main: loop {
      let delta = self.times.update();
      let delta_s = delta.as_secs() as f64 + delta.subsec_nanos() as f64 / 1_000_000_000.0;

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit { .. } => break 'main,
          Event::KeyDown {
            keycode: Some(keycode),
            ..
          } if keycode == Keycode::Escape =>
          {
            break 'main
          }
          Event::MouseMotion { x, y, .. } => self.imgui.set_mouse_pos(x as f32, y as f32),
          Event::DropFile { filename, .. } => {
            let result =
              resolve_sdl_filename(filename).and_then(|path| Ok(Bootrom::from_path(&path)?));
            match result {
              Ok(bootrom) => {
                if let Err(error) = bootrom.save_to_data_dir() {
                  println!("Failed to save boot rom: {}", error);
                }
                return Ok(FrontendState::from_roms(Some(bootrom), cartridge));
              }
              Err(e) => screen.set_error(format!("{}", e)),
            };
          }
          _ => (),
        }
      }
      let mut target = self.display.draw();
      target.clear_color(1.0, 1.0, 1.0, 1.0);

      let (width, height) = target.get_dimensions();
      let frame_size = FrameSize {
        logical_size: (width.into(), height.into()),
        hidpi_factor: 1.0,
      };
      let ui = self.imgui.frame(frame_size, delta_s as f32);
      screen.render(&ui);
      self
        .gui_renderer
        .render(&mut target, ui)
        .map_err(|e| format_err!("GUI rendering failed: {}", e))?;
      target.finish()?;

      self.times.limit();
    }
    Ok(FrontendState::Exit)
  }
  fn main_in_game(&mut self, state: InGameState) -> Result<FrontendState, Error> {
    let InGameState {
      config,
      mut machine,
      mut screen,
      mut fps_counter,
      mut perf_counter,
    } = state;
    let mut emu_time = machine.emu_time();
    self.times.reset();

    'main: loop {
      let delta = self.times.update();
      let delta_s = delta.as_secs() as f64 + delta.subsec_nanos() as f64 / 1_000_000_000.0;

      fps_counter.update(delta_s);
      screen.fps = fps_counter.get_fps();
      screen.perf = 100.0 * perf_counter.get_machine_cycles_per_s() * 4.0 / CPU_SPEED_HZ as f64;

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit { .. } => break 'main,
          Event::Window {
            win_event: WindowEvent::SizeChanged(..),
            ..
          } => {
            self.renderer.update_dimensions(&self.display);
          }
          Event::KeyDown {
            keycode: Some(keycode),
            ..
          } => {
            if let Some(key) = map_keycode(keycode) {
              machine.key_down(key);
            }
            match keycode {
              Keycode::LShift => {
                return Ok(FrontendState::InGameTurbo(InGameState {
                  config,
                  machine,
                  screen,
                  fps_counter,
                  perf_counter,
                }))
              }
              Keycode::F2 => screen.toggle_info_overlay(),
              Keycode::Escape => break 'main,
              _ => (),
            }
          }
          Event::MouseMotion { x, y, .. } => self.imgui.set_mouse_pos(x as f32, y as f32),
          Event::KeyUp {
            keycode: Some(keycode),
            ..
          } => {
            if let Some(key) = map_keycode(keycode) {
              machine.key_up(key);
            }
          }
          Event::ControllerDeviceAdded { which: id, .. } => {
            self.sdl_game_controller.open(id as u32)?;
          }
          Event::ControllerButtonDown { button, .. } => {
            if let Some(key) = map_button(button) {
              machine.key_down(key);
            }
          }
          Event::ControllerButtonUp { button, .. } => {
            if let Some(key) = map_button(button) {
              machine.key_up(key);
            }
          }
          Event::ControllerAxisMotion { axis, value, .. } => {
            if let Some((key, state)) = map_axis(axis, value) {
              if state {
                machine.key_down(key);
              } else {
                machine.key_up(key);
              }
            }
          }
          Event::DropFile { filename, .. } => {
            let result =
              resolve_sdl_filename(filename).and_then(|path| Ok(Cartridge::from_path(&path)?));
            match result {
              Ok(cartridge) => {
                return Ok(FrontendState::InGame(InGameState::from_config(
                  HardwareConfig {
                    cartridge,
                    ..config
                  },
                )))
              }
              Err(e) => screen.set_error(format!("{}", e)),
            };
          }
          _ => (),
        }
      }

      let mut target = self.display.draw();
      target.clear_color(0.0, 0.0, 0.0, 1.0);

      let (width, height) = target.get_dimensions();
      let frame_size = FrameSize {
        logical_size: (width.into(), height.into()),
        hidpi_factor: 1.0,
      };
      let ui = self.imgui.frame(frame_size, delta_s as f32);

      let machine_cycles =
        EmuTime::from_machine_cycles(((delta * CPU_SPEED_HZ as u32).as_secs() as u64) / 4);

      let target_time = emu_time + machine_cycles;
      loop {
        let (events, end_time) = machine.emulate(target_time);

        if events.contains(EmuEvents::VSYNC) {
          self.renderer.update_pixels(machine.screen_buffer());
        }

        if end_time >= target_time {
          perf_counter.update(end_time - emu_time, delta_s);
          emu_time = end_time;
          break;
        }
      }
      self.renderer.draw(&mut target)?;

      screen.render(&ui);
      self
        .gui_renderer
        .render(&mut target, ui)
        .map_err(|e| format_err!("GUI rendering failed: {}", e))?;
      target.finish()?;

      self.times.limit();
    }
    Ok(FrontendState::Exit)
  }
  fn main_in_game_turbo(&mut self, state: InGameState) -> Result<FrontendState, Error> {
    let InGameState {
      config,
      machine,
      mut screen,
      mut fps_counter,
      perf_counter,
    } = state;
    self.times.reset();

    let handle = emu_thread::spawn(machine, perf_counter);
    handle.next_tick(Box::new(SCREEN_EMPTY))?;

    'main: loop {
      let delta = self.times.update();
      let delta_s = delta.as_secs() as f64 + delta.subsec_nanos() as f64 / 1_000_000_000.0;

      fps_counter.update(delta_s);
      screen.fps = fps_counter.get_fps();

      for event in self.event_pump.poll_iter() {
        match event {
          Event::Quit { .. } => break 'main,
          Event::Window {
            win_event: WindowEvent::SizeChanged(..),
            ..
          } => {
            self.renderer.update_dimensions(&self.display);
          }
          Event::KeyDown {
            keycode: Some(keycode),
            ..
          } => {
            if let Some(key) = map_keycode(keycode) {
              handle.key_down(key)?;
            }
            match keycode {
              Keycode::F2 => screen.toggle_info_overlay(),
              Keycode::Escape => break 'main,
              _ => (),
            }
          }
          Event::MouseMotion { x, y, .. } => self.imgui.set_mouse_pos(x as f32, y as f32),
          Event::KeyUp {
            keycode: Some(keycode),
            ..
          } => {
            if let Some(key) = map_keycode(keycode) {
              handle.key_up(key)?;
            }
            if keycode == Keycode::LShift {
              let (machine, perf_counter) = handle.stop()?;
              return Ok(FrontendState::InGame(InGameState {
                config,
                machine,
                screen,
                fps_counter,
                perf_counter,
              }));
            }
          }
          Event::ControllerDeviceAdded { which: id, .. } => {
            self.sdl_game_controller.open(id as u32)?;
          }
          Event::ControllerButtonDown { button, .. } => {
            if let Some(key) = map_button(button) {
              handle.key_down(key)?;
            }
          }
          Event::ControllerButtonUp { button, .. } => {
            if let Some(key) = map_button(button) {
              handle.key_up(key)?;
            }
          }
          Event::ControllerAxisMotion { axis, value, .. } => {
            if let Some((key, state)) = map_axis(axis, value) {
              if state {
                handle.key_down(key)?;
              } else {
                handle.key_up(key)?;
              }
            }
          }
          Event::DropFile { filename, .. } => {
            let result =
              resolve_sdl_filename(filename).and_then(|path| Ok(Cartridge::from_path(&path)?));
            match result {
              Ok(cartridge) => {
                return Ok(FrontendState::InGame(InGameState::from_config(
                  HardwareConfig {
                    cartridge,
                    ..config
                  },
                )))
              }
              Err(e) => screen.set_error(format!("{}", e)),
            };
          }
          _ => (),
        }
      }

      let mut target = self.display.draw();
      target.clear_color(0.0, 0.0, 0.0, 1.0);

      let (width, height) = target.get_dimensions();
      let frame_size = FrameSize {
        logical_size: (width.into(), height.into()),
        hidpi_factor: 1.0,
      };
      let ui = self.imgui.frame(frame_size, delta_s as f32);

      if let Some(tick) = handle.check_tick() {
        screen.perf = 100.0 * tick.cycles_per_s * 4.0 / CPU_SPEED_HZ as f64;
        if tick.screen_buffer_updated {
          self.renderer.update_pixels(&tick.screen_buffer);
        }
        handle.next_tick(tick.screen_buffer)?;
      }

      self.renderer.draw(&mut target)?;

      screen.render(&ui);
      self
        .gui_renderer
        .render(&mut target, ui)
        .map_err(|e| format_err!("GUI rendering failed: {}", e))?;
      target.finish()?;
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
    _ => None,
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
    _ => None,
  }
}

fn map_axis(axis: Axis, value: i16) -> Option<(GbKey, bool)> {
  match axis {
    Axis::LeftX => match value {
      -32768...-16384 => Some((GbKey::Left, true)),
      -16383...-1 => Some((GbKey::Left, false)),
      0...16383 => Some((GbKey::Right, false)),
      16384...32767 => Some((GbKey::Right, true)),
      _ => None,
    },
    Axis::LeftY => match value {
      -32768...-16384 => Some((GbKey::Up, true)),
      -16383...-1 => Some((GbKey::Up, false)),
      0...16383 => Some((GbKey::Down, false)),
      16384...32767 => Some((GbKey::Down, true)),
      _ => None,
    },
    _ => None,
  }
}

fn resolve_sdl_filename(filename: String) -> Result<PathBuf, Error> {
  let mut url_str = "file://".to_owned();
  url_str.push_str(&filename);
  let url = Url::parse(&url_str)?;
  url
    .to_file_path()
    .map_err(|_| format_err!("Failed to parse path"))
}

#[cfg(not(target_os = "macos"))]
fn configure_gl_attr(_: &mut GLAttr) {}

#[cfg(target_os = "macos")]
fn configure_gl_attr(gl_attr: &mut GLAttr) {
  use sdl2::video::GLProfile;
  gl_attr.set_context_major_version(3);
  gl_attr.set_context_minor_version(2);
  gl_attr.set_context_profile(GLProfile::Core);
  gl_attr.set_context_flags().forward_compatible().set();
}
