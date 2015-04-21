use num::traits::Float;
use sdl2;
use sdl2::Sdl;
use sdl2::controller::{Axis, Button, GameController};
use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect;
use sdl2::render;
use sdl2::render::{RenderDriverIndex, Renderer, Texture};
use sdl2::video;
use sdl2::video::{Window, WindowPos};
use std::convert::From;
use std::error::Error;
use std::iter;
use std::fmt;
use std::fmt::Display;
use std::slice::bytes;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{Receiver, SyncSender, TryRecvError};

use backend::{
  Backend, BackendSharedMemory, GbKey, BackendMessage
};
use backend::sdl::font::{TextAlign, Font};
use backend::sdl::fps::FpsCounter;
use gameboy;
use machine::MachineMessage;

mod font;
mod fps;

pub struct SdlBackend {
  sdl: Sdl,
  fps_counter: FpsCounter,
  relative_speed_stat: f64,
  shared_memory: Arc<SharedMemory>
}

struct SharedMemory {
  pixel_buffer_lock: Mutex<Vec<u8>>,
  palette: Palette
}

impl SharedMemory {
  fn new() -> SharedMemory {
    SharedMemory {
      pixel_buffer_lock: Mutex::new(iter::repeat(0xff).take(PIXEL_BUFFER_SIZE).collect()),
      palette: Palette::from_colors(&PALETTE)
    }
  }
}

#[derive(Clone, Debug)]
pub enum BackendError {
  Sdl(String)
}

pub type BackendResult<T> = Result<T, BackendError>;

impl From<String> for BackendError {
  fn from(e: String) -> BackendError {
    BackendError::Sdl(e)
  }
}

impl Error for BackendError {
  fn description(&self) -> &str {
    match *self {
      BackendError::Sdl(..) => "SDL error"
    }
  }
}

impl Display for BackendError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      BackendError::Sdl(ref msg) => f.write_str(msg)
    }
  }
}

impl BackendSharedMemory for SharedMemory {
  fn draw_screen(&self, pixels: &gameboy::ScreenBuffer) {
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
        bytes::copy_memory(palette.get_bytes(gb_color), pixel);
      }
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

const SCREEN_RECT: rect::Rect = rect::Rect {
  x: 0,
  y: 0,
  w: gameboy::SCREEN_WIDTH as i32,
  h: gameboy::SCREEN_HEIGHT as i32
};

impl SdlBackend {
  pub fn init() -> BackendResult<SdlBackend> {
    let sdl = try!(sdl2::init(sdl2::INIT_VIDEO | sdl2::INIT_GAME_CONTROLLER));
    Ok(SdlBackend {
      sdl: sdl,
      fps_counter: FpsCounter::new(),
      relative_speed_stat: 0.0,
      shared_memory: Arc::new(SharedMemory::new())
    })
  }
  fn refresh_gb_screen(&self, renderer: &mut Renderer, texture: &mut Texture) -> BackendResult<()> {
    {
      let pixels = self.shared_memory.pixel_buffer_lock.lock().unwrap();
      try!(texture.update(Some(SCREEN_RECT), &pixels, PIXEL_BUFFER_STRIDE as i32));
    }
    let mut drawer = renderer.drawer();
    drawer.clear();
    drawer.set_logical_size(gameboy::SCREEN_WIDTH as i32, gameboy::SCREEN_HEIGHT as i32);
    drawer.copy(&texture, Some(SCREEN_RECT), Some(SCREEN_RECT));
    Ok(())
  }
  fn present(&mut self, renderer: &mut Renderer, texture: &mut Texture, font: &Font) -> BackendResult<()> {
    try!(self.refresh_gb_screen(renderer, texture));
    let mut drawer = renderer.drawer();
    drawer.set_logical_size(gameboy::SCREEN_WIDTH as i32 * 4, gameboy::SCREEN_HEIGHT as i32 * 4);

    let speed_text = format!("{} %", self.relative_speed_stat.round());
    try!(font.draw_text(&mut drawer, 0, 0, TextAlign::Left, &speed_text));

    let fps_text = format!("{} FPS", self.fps_counter.fps.round());
    try!(font.draw_text(&mut drawer, gameboy::SCREEN_WIDTH as i32 * 4, 0, TextAlign::Right, &fps_text));
    drawer.present();
    self.fps_counter.update();
    Ok(())
  }
}

fn to_joypad_key(key: KeyCode) -> Option<GbKey> {
  match key {
    KeyCode::Right => Some(GbKey::Right),
    KeyCode::Left => Some(GbKey::Left),
    KeyCode::Up => Some(GbKey::Up),
    KeyCode::Down => Some(GbKey::Down),
    KeyCode::Z => Some(GbKey::A),
    KeyCode::X => Some(GbKey::B),
    KeyCode::Return => Some(GbKey::Start),
    KeyCode::Backspace => Some(GbKey::Select),
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

fn controller_axis_to_message(axis: Axis, value: i16) -> Option<BackendMessage> {
  match axis {
    Axis::LeftX => match value {
      -32768...-16384 => Some(BackendMessage::KeyDown(GbKey::Left)),
      -16383...-1 => Some(BackendMessage::KeyUp(GbKey::Left)),
      0...16383 => Some(BackendMessage::KeyUp(GbKey::Right)),
      16384...32767 => Some(BackendMessage::KeyDown(GbKey::Right)),
      _ => None
    },
    Axis::LeftY => match value {
      -32768...-16384 => Some(BackendMessage::KeyDown(GbKey::Up)),
      -16383...-1 => Some(BackendMessage::KeyUp(GbKey::Up)),
      0...16383 => Some(BackendMessage::KeyUp(GbKey::Down)),
      16384...32767 => Some(BackendMessage::KeyDown(GbKey::Down)),
      _ => None
    },
    _ => None
  }
}

impl Backend for SdlBackend {
  type SHM = SharedMemory;
  type Error = BackendError;
  fn main_loop(mut self, to_machine: SyncSender<BackendMessage>, from_machine: Receiver<MachineMessage>) -> BackendResult<()> {
    let window =
      try!(Window::new(&self.sdl, "Mooneye GB", WindowPos::PosUndefined, WindowPos::PosUndefined, 640, 576, video::OPENGL));
    let mut renderer =
      try!(Renderer::from_window(window, RenderDriverIndex::Auto, render::ACCELERATED | render::PRESENTVSYNC));
    {
      let mut drawer = renderer.drawer();
      drawer.clear();
      drawer.present();
    }

    let mut controllers = vec!();

    let mut texture = try!(renderer.create_texture_streaming(PixelFormatEnum::RGBA8888, (256, 256)));

    let font = try!(Font::init(&renderer));

    'main: loop {
      match from_machine.try_recv() {
        Err(TryRecvError::Disconnected) => break,
        Ok(MachineMessage::RelativeSpeedStat(value)) => self.relative_speed_stat = value,
        _ => ()
      }

      for event in self.sdl.event_pump().poll_iter() {
        match event {
          Event::Quit{..} => break 'main,
          Event::KeyDown{keycode, ..} if keycode == KeyCode::Escape => break 'main,
          Event::KeyDown{keycode, ..} => {
            match to_joypad_key(keycode) {
              Some(key) => to_machine.send(BackendMessage::KeyDown(key)).unwrap(),
              None => ()
            }
            match keycode {
              KeyCode::Home => to_machine.send(BackendMessage::Break).unwrap(),
              KeyCode::End => to_machine.send(BackendMessage::Run).unwrap(),
              KeyCode::PageDown => to_machine.send(BackendMessage::Step).unwrap(),
              KeyCode::LShift => to_machine.send(BackendMessage::Turbo(true)).unwrap(),
              _ => ()
            }
          },
          Event::KeyUp{keycode, ..} => {
            match to_joypad_key(keycode) {
              Some(key) => to_machine.send(BackendMessage::KeyUp(key)).unwrap(),
              None => ()
            }
            match keycode {
              KeyCode::LShift => to_machine.send(BackendMessage::Turbo(false)).unwrap(),
              _ => ()
            }
          },
          Event::ControllerDeviceAdded{which: id, ..} => {
            controllers.push(try!(GameController::open(id)))
          },
          Event::ControllerButtonDown{button, ..} => {
            match controller_to_joypad_key(button) {
              Some(key) => to_machine.send(BackendMessage::KeyDown(key)).unwrap(),
              None => ()
            }
          },
          Event::ControllerButtonUp{button, ..} => {
            match controller_to_joypad_key(button) {
              Some(key) => to_machine.send(BackendMessage::KeyUp(key)).unwrap(),
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
      match self.present(&mut renderer, &mut texture, &font) {
        Err(error) => { println!("{}", error.description()); break },
        _ => ()
      }
    }
    Ok(())
  }
  fn shared_memory(&self) -> Arc<SharedMemory> {
    self.shared_memory.clone()
  }
}
