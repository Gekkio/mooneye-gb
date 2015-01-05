use sdl2;
use sdl2::controller::{ControllerAxis, ControllerButton};
use sdl2::event;
use sdl2::event::Event;
use sdl2::keycode::KeyCode;
use sdl2::pixels::{Color, PixelFormatFlag};
use sdl2::rect;
use sdl2::render;
use sdl2::render::{RenderDriverIndex, Renderer, Texture, TextureAccess};
use sdl2::video;
use sdl2::video::{Window, WindowPos};
use std::error::{Error, FromError};
use std::iter;
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
  renderer: Renderer,
  texture: Texture,
  font: Font,
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

enum BackendError {
  Sdl(String)
}

pub type BackendResult<T> = Result<T, BackendError>;

impl FromError<String> for BackendError {
  fn from_error(e: String) -> BackendError {
    BackendError::Sdl(e)
  }
}

impl Error for BackendError {
  fn description(&self) -> &str {
    match *self {
      BackendError::Sdl(..) => "SDL error"
    }
  }
  fn detail(&self) -> Option<String> {
    match *self {
      BackendError::Sdl(ref msg) => Some(msg.to_string())
    }
  }
}

impl BackendSharedMemory for SharedMemory {
  fn draw_screen(&self, pixels: &gameboy::ScreenBuffer) {
    let mut data = self.pixel_buffer_lock.lock().unwrap();
    let ref palette = self.palette;
    for y in range(0, gameboy::SCREEN_HEIGHT) {
      let in_start = y * gameboy::SCREEN_WIDTH;
      let in_end = in_start + gameboy::SCREEN_WIDTH;
      let in_slice = pixels.slice(in_start, in_end);

      let out_start = y * PIXEL_BUFFER_STRIDE;
      let out_end = out_start + gameboy::SCREEN_WIDTH * 4;
      let out_slice = data.slice_mut(out_start, out_end);

      for (pixel, gb_color) in out_slice.chunks_mut(4).zip(in_slice.iter()) {
        bytes::copy_memory(pixel, palette.get_bytes(gb_color));
      }
    }
  }
}

const PIXEL_BUFFER_ROWS: uint = gameboy::SCREEN_HEIGHT;
const PIXEL_BUFFER_STRIDE: uint = 256 * 4;
const PIXEL_BUFFER_SIZE: uint = PIXEL_BUFFER_STRIDE * PIXEL_BUFFER_ROWS;

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
    Color::RGBA(0xbd, 0xe6, 0x12, 255),
    Color::RGBA(0x90, 0xb3, 0x0f, 255),
    Color::RGBA(0x30, 0x62, 0x30, 255),
    Color::RGBA(0x07, 0x1a, 0x07, 255)
  ];

const SCREEN_RECT: rect::Rect = rect::Rect {
  x: 0,
  y: 0,
  w: gameboy::SCREEN_WIDTH as i32,
  h: gameboy::SCREEN_HEIGHT as i32
};

impl SdlBackend {
  pub fn init() -> BackendResult<SdlBackend> {
    sdl2::init(sdl2::INIT_VIDEO | sdl2::INIT_GAME_CONTROLLER);
    let window =
      try!(Window::new("test", WindowPos::PosUndefined, WindowPos::PosUndefined, 640, 576, video::OPENGL));
    let renderer =
      try!(Renderer::from_window(window, RenderDriverIndex::Auto, render::ACCELERATED | render::PRESENTVSYNC));
    try!(renderer.clear());
    renderer.present();

    let texture =
      try!(renderer.create_texture(PixelFormatFlag::RGBA8888, TextureAccess::Streaming, 256, 256));

    let font = try!(Font::init(&renderer));

    Ok(SdlBackend {
      renderer: renderer,
      texture: texture,
      font: font,
      fps_counter: FpsCounter::new(),
      relative_speed_stat: 0.0,
      shared_memory: Arc::new(SharedMemory::new())
    })
  }
  fn refresh_gb_screen(&self) -> BackendResult<()> {
    {
      let pixels = self.shared_memory.pixel_buffer_lock.lock().unwrap();
      try!(self.texture.update(Some(SCREEN_RECT), pixels[], PIXEL_BUFFER_STRIDE as int));
    }
    try!(self.renderer.set_logical_size(gameboy::SCREEN_WIDTH as int, gameboy::SCREEN_HEIGHT as int));
    try!(self.renderer.copy(&self.texture, Some(SCREEN_RECT), Some(SCREEN_RECT)));
    Ok(())
  }
  fn present(&mut self) -> BackendResult<()> {
    try!(self.refresh_gb_screen());
    try!(self.renderer.set_logical_size(gameboy::SCREEN_WIDTH as int * 4, gameboy::SCREEN_HEIGHT as int * 4));

    let speed_text = format!("{:0.0} %", self.relative_speed_stat);
    try!(self.font.draw_text(&self.renderer, 0, 0, TextAlign::Left, speed_text.as_slice()));

    let fps_text = format!("{:0.0} FPS", self.fps_counter.fps);
    try!(self.font.draw_text(&self.renderer, gameboy::SCREEN_WIDTH as i32 * 4, 0, TextAlign::Right, fps_text.as_slice()));
    self.renderer.present();
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

fn controller_to_joypad_key(button: ControllerButton) -> Option<GbKey> {
  match button {
    ControllerButton::DPadRight => Some(GbKey::Right),
    ControllerButton::DPadLeft => Some(GbKey::Left),
    ControllerButton::DPadUp => Some(GbKey::Up),
    ControllerButton::DPadDown => Some(GbKey::Down),
    ControllerButton::A => Some(GbKey::B),
    ControllerButton::B => Some(GbKey::A),
    ControllerButton::Start => Some(GbKey::Start),
    ControllerButton::Back => Some(GbKey::Select),
    _ => None
  }
}

fn controller_axis_to_message(axis: ControllerAxis, value: i16) -> Option<BackendMessage> {
  match axis {
    ControllerAxis::LeftX => match value {
      -32768...-16384 => Some(BackendMessage::KeyDown(GbKey::Left)),
      -16383...-1 => Some(BackendMessage::KeyUp(GbKey::Left)),
      0...16383 => Some(BackendMessage::KeyUp(GbKey::Right)),
      16384...32767 => Some(BackendMessage::KeyDown(GbKey::Right)),
      _ => None
    },
    ControllerAxis::LeftY => match value {
      -32768...-16384 => Some(BackendMessage::KeyDown(GbKey::Up)),
      -16383...-1 => Some(BackendMessage::KeyUp(GbKey::Up)),
      0...16383 => Some(BackendMessage::KeyUp(GbKey::Down)),
      16384...32767 => Some(BackendMessage::KeyDown(GbKey::Down)),
      _ => None
    },
    _ => None
  }
}

impl Backend<SharedMemory> for SdlBackend {
  fn main_loop(&mut self, to_machine: SyncSender<BackendMessage>, from_machine: Receiver<MachineMessage>) {
    loop {
      match from_machine.try_recv() {
        Err(TryRecvError::Disconnected) => break,
        Ok(MachineMessage::RelativeSpeedStat(value)) => self.relative_speed_stat = value,
        _ => ()
      }

      'event: loop {
        match event::poll_event() {
          Event::Quit(_) => return,
          Event::KeyDown(_, _, key, _, _, _) if key == KeyCode::Escape => return,
          Event::KeyDown(_, _, key, _, _, _) => {
            match to_joypad_key(key) {
              Some(key) => to_machine.send(BackendMessage::KeyDown(key)).unwrap(),
              None => ()
            }
            match key {
              KeyCode::Home => to_machine.send(BackendMessage::Break).unwrap(),
              KeyCode::End => to_machine.send(BackendMessage::Run).unwrap(),
              KeyCode::PageDown => to_machine.send(BackendMessage::Step).unwrap(),
              KeyCode::LShift => to_machine.send(BackendMessage::Turbo(true)).unwrap(),
              _ => ()
            }
          },
          Event::KeyUp(_, _, key, _, _, _) => {
            match to_joypad_key(key) {
              Some(key) => to_machine.send(BackendMessage::KeyUp(key)).unwrap(),
              None => ()
            }
            match key {
              KeyCode::LShift => to_machine.send(BackendMessage::Turbo(false)).unwrap(),
              _ => ()
            }
          },
          Event::ControllerDeviceAdded(_, id) => {
            unsafe { sdl2::controller::ll::SDL_GameControllerOpen(id as i32); }
          },
          Event::ControllerButtonDown(_, _, button) => {
            match controller_to_joypad_key(button) {
              Some(key) => to_machine.send(BackendMessage::KeyDown(key)).unwrap(),
              None => ()
            }
          },
          Event::ControllerButtonUp(_, _, button) => {
            match controller_to_joypad_key(button) {
              Some(key) => to_machine.send(BackendMessage::KeyUp(key)).unwrap(),
              None => ()
            }
          },
          Event::ControllerAxisMotion(_, _, axis, value) => {
            match controller_axis_to_message(axis, value) {
              Some(message) => to_machine.send(message).unwrap(),
              None => ()
            }
          },
          Event::None => break 'event,
          _ => ()
        }
      }
      match self.present() {
        Err(error) => { println!("{}", error.description()); break },
        _ => ()
      }
    }
    sdl2::quit();
  }
  fn shared_memory(&self) -> Arc<SharedMemory> {
    self.shared_memory.clone()
  }
}
