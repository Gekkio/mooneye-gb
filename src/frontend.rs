// This file is part of Mooneye GB.
// Copyright (C) 2014-2020 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use failure::{format_err, Error};
use gilrs::{Axis, Button, EventType, Gilrs};
use glium::{glutin, Api, Display, Surface, Version};
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use log::info;
use std::time::Duration;

use self::gui::Screen;
use self::renderer::Renderer;
use crate::fps_counter::FpsCounter;
use crate::frame_times::FrameTimes;
use crate::perf_counter::PerfCounter;
use mooneye_gb::config::{Bootrom, Cartridge, HardwareConfig};
use mooneye_gb::emulation::{EmuEvents, EmuTime};
use mooneye_gb::machine::Machine;
use mooneye_gb::*;

mod emu_thread;
mod gui;
mod renderer;

pub struct SdlFrontend {
  gilrs: Gilrs,
  events_loop: glutin::EventsLoop,
  display: Display,
  imgui: imgui::Context,
  imgui_renderer: imgui_glium_renderer::Renderer,
  imgui_platform: WinitPlatform,
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

enum InGameEvent {
  Exit,
  Turbo,
  TurboOff,
  LoadCartridge(Cartridge),
}

impl SdlFrontend {
  pub fn init() -> Result<SdlFrontend, Error> {
    let gilrs = Gilrs::new().map_err(|_| format_err!("Failed to initialize gamepad support"))?;

    use glium::glutin::dpi::LogicalSize;

    let events_loop = glutin::EventsLoop::new();

    let window = glutin::WindowBuilder::new()
      .with_min_dimensions(LogicalSize::new(160., 144.))
      .with_dimensions(LogicalSize::new(640., 576.))
      .with_title("Mooneye GB");

    let context = glutin::ContextBuilder::new()
      .with_hardware_acceleration(Some(true))
      .with_vsync(true)
      .with_srgb(true)
      .with_double_buffer(Some(true));

    let display = Display::new(window, context, &events_loop).unwrap();

    info!(
      "Initialized renderer with {}",
      match *display.get_opengl_version() {
        Version(Api::Gl, major, minor) => format!("OpenGL {}.{}", major, minor),
        Version(Api::GlEs, major, minor) => format!("OpenGL ES {}.{}", major, minor),
      }
    );

    let renderer = Renderer::new(&display)?;

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    imgui.set_log_filename(None);
    let imgui_renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &display)
      .map_err(|e| format_err!("Failed to initialize renderer: {}", e))?;
    let mut imgui_platform = WinitPlatform::init(&mut imgui);

    {
      let gl_window = display.gl_window();
      let window = gl_window.window();
      imgui_platform.attach_window(imgui.io_mut(), &window, HiDpiMode::Default);
    }

    Ok(SdlFrontend {
      gilrs,
      events_loop,
      display,
      imgui,
      imgui_renderer,
      imgui_platform,
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

    let mut loaded_bootrom = None;

    'main: loop {
      let delta = self.times.update();
      let delta_s = delta.as_secs() as f64 + f64::from(delta.subsec_nanos()) / 1_000_000_000.0;

      let renderer = &mut self.renderer;
      let display = &self.display;
      let gl_window = display.gl_window();
      let window = gl_window.window();
      let imgui = &mut self.imgui;
      let imgui_platform = &mut self.imgui_platform;
      let mut exit = false;
      self.events_loop.poll_events(|event| {
        imgui_platform.handle_event(imgui.io_mut(), &window, &event);
        if let glutin::Event::WindowEvent { event, .. } = event {
          use glium::glutin::WindowEvent;
          match event {
            WindowEvent::Resized(..) => renderer.update_dimensions(display),
            WindowEvent::CloseRequested | WindowEvent::Destroyed => exit = true,
            WindowEvent::DroppedFile(path) => match Bootrom::from_path(&path) {
              Ok(bootrom) => {
                if let Err(error) = bootrom.save_to_data_dir() {
                  println!("Failed to save boot rom: {}", error);
                }
                loaded_bootrom = Some(bootrom);
              }
              Err(e) => screen.set_error(format!("{}", e)),
            },
            _ => (),
          }
        }
      });
      while let Some(_) = self.gilrs.next_event() {}
      if let Some(bootrom) = loaded_bootrom {
        return Ok(FrontendState::from_roms(Some(bootrom), cartridge));
      }
      if exit {
        break 'main;
      }

      let mut target = self.display.draw();
      target.clear_color(1.0, 1.0, 1.0, 1.0);

      self
        .imgui_platform
        .prepare_frame(self.imgui.io_mut(), &window)
        .map_err(|e| format_err!("Failed to prepare imgui frame: {}", e))?;
      self.imgui.io_mut().delta_time = delta_s as f32;
      let ui = self.imgui.frame();
      screen.render(&ui);
      self.imgui_platform.prepare_render(&ui, &window);
      let draw_data = ui.render();
      self
        .imgui_renderer
        .render(&mut target, draw_data)
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
      let delta_s = delta.as_secs() as f64 + f64::from(delta.subsec_nanos()) / 1_000_000_000.0;

      fps_counter.update(delta_s);
      screen.fps = fps_counter.get_fps();
      screen.perf = 100.0 * perf_counter.get_machine_cycles_per_s() * 4.0 / CPU_SPEED_HZ as f64;

      while let Some(gilrs::Event { event, .. }) = self.gilrs.next_event() {
        match event {
          EventType::ButtonPressed(button, _) => {
            if let Some(key) = map_button(button) {
              machine.key_down(key);
            }
          }
          EventType::ButtonReleased(button, _) => {
            if let Some(key) = map_button(button) {
              machine.key_up(key);
            }
          }
          EventType::AxisChanged(axis, value, _) => {
            if let Some((key, state)) = map_axis(axis, value) {
              if state {
                machine.key_down(key);
              } else {
                machine.key_up(key);
              }
            }
          }
          _ => (),
        }
      }

      let renderer = &mut self.renderer;
      let display = &self.display;
      let gl_window = display.gl_window();
      let window = gl_window.window();
      let imgui = &mut self.imgui;
      let imgui_platform = &mut self.imgui_platform;
      let mut ig_event = None;
      self.events_loop.poll_events(|event| {
        imgui_platform.handle_event(imgui.io_mut(), &window, &event);
        if let glutin::Event::WindowEvent { event, .. } = event {
          use glium::glutin::ElementState;
          use glium::glutin::KeyboardInput;
          use glium::glutin::VirtualKeyCode;
          use glium::glutin::WindowEvent;
          match event {
            WindowEvent::Resized(..) => renderer.update_dimensions(display),
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
              ig_event = Some(InGameEvent::Exit)
            }
            WindowEvent::KeyboardInput {
              input:
                KeyboardInput {
                  state: ElementState::Pressed,
                  virtual_keycode: Some(keycode),
                  ..
                },
              ..
            } => {
              if let Some(key) = map_keycode(keycode) {
                machine.key_down(key);
              }
              match keycode {
                VirtualKeyCode::LShift => ig_event = Some(InGameEvent::Turbo),
                VirtualKeyCode::F2 => screen.toggle_info_overlay(),
                VirtualKeyCode::Escape => ig_event = Some(InGameEvent::Exit),
                _ => (),
              }
            }
            WindowEvent::KeyboardInput {
              input:
                KeyboardInput {
                  state: ElementState::Released,
                  virtual_keycode: Some(keycode),
                  ..
                },
              ..
            } => {
              if let Some(key) = map_keycode(keycode) {
                machine.key_up(key);
              }
            }
            WindowEvent::DroppedFile(path) => match Cartridge::from_path(&path) {
              Ok(cartridge) => {
                ig_event = Some(InGameEvent::LoadCartridge(cartridge));
              }
              Err(e) => screen.set_error(format!("{}", e)),
            },
            _ => (),
          }
        }
      });
      match ig_event {
        Some(InGameEvent::Exit) => break 'main,
        Some(InGameEvent::Turbo) => {
          return Ok(FrontendState::InGameTurbo(InGameState {
            config,
            machine,
            screen,
            fps_counter,
            perf_counter,
          }));
        }
        Some(InGameEvent::LoadCartridge(cartridge)) => {
          return Ok(FrontendState::InGame(InGameState::from_config(
            HardwareConfig {
              cartridge,
              ..config
            },
          )));
        }
        _ => (),
      }

      let mut target = self.display.draw();
      target.clear_color(0.0, 0.0, 0.0, 1.0);

      self
        .imgui_platform
        .prepare_frame(self.imgui.io_mut(), &window)
        .map_err(|e| format_err!("Failed to prepare imgui frame: {}", e))?;
      self.imgui.io_mut().delta_time = delta_s as f32;
      let ui = self.imgui.frame();

      let machine_cycles =
        EmuTime::from_machine_cycles(((delta * CPU_SPEED_HZ as u32).as_secs() as u64) / 4);

      let target_time = emu_time + machine_cycles;
      loop {
        let (events, end_time) = machine.emulate(target_time);

        if events.contains(EmuEvents::VSYNC) {
          renderer.update_pixels(machine.screen_buffer());
        }

        if end_time >= target_time {
          perf_counter.update(end_time - emu_time, delta_s);
          emu_time = end_time;
          break;
        }
      }
      renderer.draw(&mut target)?;

      screen.render(&ui);
      self.imgui_platform.prepare_render(&ui, &window);
      let draw_data = ui.render();
      self
        .imgui_renderer
        .render(&mut target, draw_data)
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
      let delta_s = delta.as_secs() as f64 + f64::from(delta.subsec_nanos()) / 1_000_000_000.0;

      fps_counter.update(delta_s);
      screen.fps = fps_counter.get_fps();

      let renderer = &mut self.renderer;
      let display = &self.display;
      let gl_window = display.gl_window();
      let window = gl_window.window();
      let imgui = &mut self.imgui;
      let imgui_platform = &mut self.imgui_platform;
      let mut ig_event = None;
      self.events_loop.poll_events(|event| {
        imgui_platform.handle_event(imgui.io_mut(), &window, &event);
        if let glutin::Event::WindowEvent { event, .. } = event {
          use glium::glutin::ElementState;
          use glium::glutin::KeyboardInput;
          use glium::glutin::VirtualKeyCode;
          use glium::glutin::WindowEvent;
          match event {
            WindowEvent::Resized(..) => renderer.update_dimensions(display),
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
              ig_event = Some(InGameEvent::Exit)
            }
            WindowEvent::KeyboardInput {
              input:
                KeyboardInput {
                  state: ElementState::Pressed,
                  virtual_keycode: Some(keycode),
                  ..
                },
              ..
            } => {
              if let Some(key) = map_keycode(keycode) {
                let _ = handle.key_down(key);
              }
              match keycode {
                VirtualKeyCode::F2 => screen.toggle_info_overlay(),
                VirtualKeyCode::Escape => ig_event = Some(InGameEvent::Exit),
                _ => (),
              }
            }
            WindowEvent::KeyboardInput {
              input:
                KeyboardInput {
                  state: ElementState::Released,
                  virtual_keycode: Some(keycode),
                  ..
                },
              ..
            } => {
              if let Some(key) = map_keycode(keycode) {
                let _ = handle.key_up(key);
              }
              if keycode == VirtualKeyCode::LShift {
                ig_event = Some(InGameEvent::TurboOff);
              }
            }
            WindowEvent::DroppedFile(path) => match Cartridge::from_path(&path) {
              Ok(cartridge) => {
                ig_event = Some(InGameEvent::LoadCartridge(cartridge));
              }
              Err(e) => screen.set_error(format!("{}", e)),
            },
            _ => (),
          }
        }
      });
      match ig_event {
        Some(InGameEvent::Exit) => break 'main,
        Some(InGameEvent::TurboOff) => {
          let (machine, perf_counter) = handle.stop()?;
          return Ok(FrontendState::InGame(InGameState {
            config,
            machine,
            screen,
            fps_counter,
            perf_counter,
          }));
        }
        Some(InGameEvent::LoadCartridge(cartridge)) => {
          return Ok(FrontendState::InGame(InGameState::from_config(
            HardwareConfig {
              cartridge,
              ..config
            },
          )));
        }
        _ => (),
      }

      while let Some(gilrs::Event { event, .. }) = self.gilrs.next_event() {
        println!("{:?}", event);
        match event {
          EventType::ButtonPressed(button, _) => {
            if let Some(key) = map_button(button) {
              handle.key_down(key)?;
            }
          }
          EventType::ButtonReleased(button, _) => {
            if let Some(key) = map_button(button) {
              handle.key_up(key)?;
            }
          }
          EventType::AxisChanged(axis, value, _) => {
            if let Some((key, state)) = map_axis(axis, value) {
              if state {
                handle.key_down(key)?;
              } else {
                handle.key_up(key)?;
              }
            }
          }
          _ => (),
        }
      }

      let mut target = self.display.draw();
      target.clear_color(0.0, 0.0, 0.0, 1.0);

      self
        .imgui_platform
        .prepare_frame(self.imgui.io_mut(), &window)
        .map_err(|e| format_err!("Failed to prepare imgui frame: {}", e))?;
      self.imgui.io_mut().delta_time = delta_s as f32;
      let ui = self.imgui.frame();

      if let Some(tick) = handle.check_tick() {
        screen.perf = 100.0 * tick.cycles_per_s * 4.0 / CPU_SPEED_HZ as f64;
        if tick.screen_buffer_updated {
          renderer.update_pixels(&tick.screen_buffer);
        }
        handle.next_tick(tick.screen_buffer)?;
      }

      renderer.draw(&mut target)?;

      screen.render(&ui);
      self.imgui_platform.prepare_render(&ui, &window);
      let draw_data = ui.render();
      self
        .imgui_renderer
        .render(&mut target, draw_data)
        .map_err(|e| format_err!("GUI rendering failed: {}", e))?;
      target.finish()?;
    }
    Ok(FrontendState::Exit)
  }
}

fn map_keycode(key: glutin::VirtualKeyCode) -> Option<GbKey> {
  use glium::glutin::VirtualKeyCode::*;
  match key {
    Right => Some(GbKey::Right),
    Left => Some(GbKey::Left),
    Up => Some(GbKey::Up),
    Down => Some(GbKey::Down),
    Z => Some(GbKey::A),
    X => Some(GbKey::B),
    Return => Some(GbKey::Start),
    Back => Some(GbKey::Select),
    _ => None,
  }
}

fn map_button(button: Button) -> Option<GbKey> {
  match button {
    Button::DPadRight => Some(GbKey::Right),
    Button::DPadLeft => Some(GbKey::Left),
    Button::DPadUp => Some(GbKey::Up),
    Button::DPadDown => Some(GbKey::Down),
    Button::South => Some(GbKey::B),
    Button::East => Some(GbKey::A),
    Button::Start => Some(GbKey::Start),
    Button::Select => Some(GbKey::Select),
    _ => None,
  }
}

fn map_axis(axis: Axis, value: f32) -> Option<(GbKey, bool)> {
  match axis {
    Axis::LeftStickX => {
      if value < -0.5 {
        Some((GbKey::Left, true))
      } else if value < 0.0 {
        Some((GbKey::Left, false))
      } else if value < 0.5 {
        Some((GbKey::Right, false))
      } else {
        Some((GbKey::Right, true))
      }
    }
    Axis::LeftStickY => {
      if value < -0.5 {
        Some((GbKey::Up, true))
      } else if value < 0.0 {
        Some((GbKey::Up, false))
      } else if value < 0.5 {
        Some((GbKey::Down, false))
      } else {
        Some((GbKey::Down, true))
      }
    }
    _ => None,
  }
}
