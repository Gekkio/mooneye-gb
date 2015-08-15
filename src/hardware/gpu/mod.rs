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
#![allow(dead_code)]

use std::fmt;
use std::cmp::Ordering;

use emulation::{EmuEvents, EE_VSYNC};
use gameboy;
use gameboy::Color;
use hardware::irq::{Irq, Interrupt};
use util::int::IntExt;

const CHARACTER_RAM_TILES: usize = 384;
const TILE_MAP_SIZE: usize = 0x400;
const OAM_SPRITES: usize = 40;
const ACCESS_OAM_CYCLES: isize = 21;
const ACCESS_VRAM_CYCLES: isize = 43;
const HBLANK_CYCLES: isize = 50;
const VBLANK_LINE_CYCLES: isize = 114;
const UNDEFINED_READ: u8 = 0xff;
const STAT_UNUSED_MASK: u8 = (1 << 7);

pub struct Gpu {
  control: Control,
  stat: Stat,
  current_line: u8,
  compare_line: u8,
  scroll_x: u8,
  scroll_y: u8,
  window_x: u8,
  window_y: u8,
  bg_palette: Palette,
  obj_palette0: Palette,
  obj_palette1: Palette,
  mode: Mode,
  cycles: isize,
  character_ram: [Tile; CHARACTER_RAM_TILES],
  oam: [Sprite; OAM_SPRITES],
  tile_map1: [u8; TILE_MAP_SIZE],
  tile_map2: [u8; TILE_MAP_SIZE],
  pub back_buffer: Box<gameboy::ScreenBuffer>
}

#[derive(Clone, Copy)]
struct Tile {
  data: [u8; 16]
}

impl Tile {
  fn new() -> Tile {
    Tile {
      data: [0; 16]
    }
  }
}

#[derive(Clone, Copy)]
struct Sprite {
  x: u8,
  y: u8,
  tile_num: u8,
  flags: SpriteFlags
}

impl Sprite {
  fn new() -> Sprite {
    Sprite {
      x: 0,
      y: 0,
      tile_num: 0,
      flags: SpriteFlags::empty()
    }
  }
}

bitflags!(
  flags SpriteFlags: u8 {
    const SPRITE_UNUSED_MASK = 0b_0000_1111,
    const SPRITE_PALETTE     = 0b_0001_0000,
    const SPRITE_FLIPX       = 0b_0010_0000,
    const SPRITE_FLIPY       = 0b_0100_0000,
    const SPRITE_PRIORITY    = 0b_1000_0000
  }
);

struct Palette {
  off: Color,
  light: Color,
  dark: Color,
  on: Color,
  bits: u8
}

impl Palette {
  fn new() -> Palette {
    Palette {
      off: Color::On,
      light: Color::On,
      dark: Color::On,
      on: Color::On,
      bits: 0xff
    }
  }
  fn get(&self, color: &Color) -> Color {
    match *color {
      Color::Off => self.off,
      Color::Light => self.light,
      Color::Dark => self.dark,
      Color::On => self.on
    }
  }
  fn set_bits(&mut self, value: u8) {
    self.off = Color::from_u8((value >> 0) & 0x3);
    self.light = Color::from_u8((value >> 2) & 0x3);
    self.dark = Color::from_u8((value >> 4) & 0x3);
    self.on = Color::from_u8((value >> 6) & 0x3);
    self.bits = value;
  }
}

bitflags!(
  flags Control: u8 {
    const CTRL_BG_ON = 1 << 0,
    const CTRL_OBJ_ON = 1 << 1,
    const CTRL_OBJ_SIZE = 1 << 2,
    const CTRL_BG_MAP = 1 << 3,
    const CTRL_BG_ADDR = 1 << 4,
    const CTRL_WINDOW_ON = 1 << 5,
    const CTRL_WINDOW_MAP = 1 << 6,
    const CTRL_LCD_ON = 1 << 7
  }
);

bitflags!(
  flags Stat: u8 {
    const STAT_COMPARE = 1 << 2,
    const STAT_HBLANK_INT = 1 << 3,
    const STAT_VBLANK_INT = 1 << 4,
    const STAT_ACCESS_OAM_INT = 1 << 5,
    const STAT_COMPARE_INT = 1 << 6
  }
);

#[derive(PartialEq, Eq)]
enum Mode {
  AccessOam, AccessVram, HBlank, VBlank
}

impl Mode {
  fn cycles(&self, scroll_x: u8) -> isize {
    // FIXME: This is basically an ugly hack to pass a test. Most likely we don't just want
    // to adjust the cycle counts, but to actually emulate the behaviour that causes the adjustment
    let scroll_adjust = match scroll_x % 0x08 {
      5...7 => 2,
      1...4 => 1,
      _ => 0
    };
    match *self {
      Mode::AccessOam => ACCESS_OAM_CYCLES,
      Mode::AccessVram => ACCESS_VRAM_CYCLES + scroll_adjust,
      Mode::HBlank => HBLANK_CYCLES - scroll_adjust,
      Mode::VBlank => VBLANK_LINE_CYCLES
    }
  }
  fn bits(&self) -> u8 {
    match *self {
      Mode::AccessOam => 2,
      Mode::AccessVram => 3,
      Mode::HBlank => 0,
      Mode::VBlank => 1
    }
  }
}

impl Gpu {
  pub fn new() -> Gpu {
    Gpu {
      control: Control::empty(),
      stat: Stat::empty(),
      current_line: 0,
      compare_line: 0,
      scroll_x: 0,
      scroll_y: 0,
      window_x: 0,
      window_y: 0,
      bg_palette: Palette::new(),
      obj_palette0: Palette::new(),
      obj_palette1: Palette::new(),
      mode: Mode::AccessOam,
      cycles: ACCESS_OAM_CYCLES,
      character_ram: [Tile::new(); CHARACTER_RAM_TILES],
      oam: [Sprite::new(); OAM_SPRITES],
      tile_map1: [0; TILE_MAP_SIZE],
      tile_map2: [0; TILE_MAP_SIZE],
      back_buffer: Box::new(gameboy::SCREEN_EMPTY)
    }
  }
  pub fn get_control(&self) -> u8 {
    self.control.bits
  }
  pub fn get_stat(&self) -> u8 {
    if !self.control.contains(CTRL_LCD_ON) { STAT_UNUSED_MASK } else {
      self.mode.bits() | self.stat.bits | STAT_UNUSED_MASK
    }
  }
  pub fn get_scroll_y(&self) -> u8 {
    self.scroll_y
  }
  pub fn get_scroll_x(&self) -> u8 {
    self.scroll_x
  }
  pub fn get_current_line(&self) -> u8 {
    self.current_line
  }
  pub fn get_compare_line(&self) -> u8 {
    self.compare_line
  }
  pub fn get_bg_palette(&self) -> u8 {
    self.bg_palette.bits
  }
  pub fn get_obj_palette0(&self) -> u8 {
    self.obj_palette0.bits
  }
  pub fn get_obj_palette1(&self) -> u8 {
    self.obj_palette1.bits
  }
  pub fn get_window_x(&self) -> u8 {
    self.window_x
  }
  pub fn get_window_y(&self) -> u8 {
    self.window_y
  }
  pub fn set_control(&mut self, value: u8) {
    let new_control = Control::from_bits_truncate(value);
    if !new_control.contains(CTRL_LCD_ON) && self.control.contains(CTRL_LCD_ON) {
      if self.mode != Mode::VBlank {
        panic!("Warning! LCD off, but not in VBlank");
      }
      self.current_line = 0;
    }
    if new_control.contains(CTRL_LCD_ON) && !self.control.contains(CTRL_LCD_ON) {
      self.mode = Mode::HBlank;
      self.cycles = Mode::AccessOam.cycles(self.scroll_x);
      self.stat.insert(STAT_COMPARE);
    }
    self.control = new_control;
  }
  pub fn set_stat(&mut self, value: u8) {
    let new_stat = Stat::from_bits_truncate(value);
    self.stat = (self.stat & STAT_COMPARE) |
                (new_stat & STAT_HBLANK_INT) |
                (new_stat & STAT_VBLANK_INT) |
                (new_stat & STAT_ACCESS_OAM_INT) |
                (new_stat & STAT_COMPARE_INT);
  }
  pub fn set_scroll_y(&mut self, value: u8) {
    self.scroll_y = value;
  }
  pub fn set_scroll_x(&mut self, value: u8) {
    self.scroll_x = value;
  }
  pub fn reset_current_line(&mut self) {
    self.current_line = 0;
  }
  pub fn set_compare_line(&mut self, value: u8) {
    self.compare_line = value;
  }
  pub fn set_bg_palette(&mut self, value: u8) {
    self.bg_palette.set_bits(value);
  }
  pub fn set_obj_palette0(&mut self, value: u8) {
    self.obj_palette0.set_bits(value);
  }
  pub fn set_obj_palette1(&mut self, value: u8) {
    self.obj_palette1.set_bits(value);
  }
  pub fn set_window_x(&mut self, value: u8) {
    self.window_x = value;
  }
  pub fn set_window_y(&mut self, value: u8) {
    self.window_y = value;
  }
  pub fn write_character_ram(&mut self, reladdr: u16, value: u8) {
    if self.mode == Mode::AccessVram {
      return;
    }
    let tile = &mut self.character_ram[reladdr as usize / 16];
    tile.data[reladdr as usize % 16] = value;
  }
  pub fn write_tile_map1(&mut self, reladdr: u16, value: u8) {
    if self.mode == Mode::AccessVram {
      return;
    }
    self.tile_map1[reladdr as usize] = value;
  }
  pub fn write_tile_map2(&mut self, reladdr: u16, value: u8) {
    if self.mode == Mode::AccessVram {
      return;
    }
    self.tile_map2[reladdr as usize] = value;
  }
  pub fn write_oam(&mut self, reladdr: u16, value: u8) {
    if self.mode == Mode::AccessVram || self.mode == Mode::AccessOam {
      return;
    }
    let sprite = &mut self.oam[reladdr as usize / 4];
    match reladdr as usize % 4 {
      3 => {
        sprite.flags = SpriteFlags::from_bits_truncate(value);
      },
      2 => sprite.tile_num = value,
      1 => sprite.x = value.wrapping_sub(8),
      _ => sprite.y = value.wrapping_sub(16)
    }
  }
  pub fn read_character_ram(&self, reladdr: u16) -> u8 {
    if self.mode == Mode::AccessVram {
      return UNDEFINED_READ;
    }
    let tile = &self.character_ram[reladdr as usize / 16];
    tile.data[reladdr as usize % 16]
  }
  pub fn read_tile_map1(&self, reladdr: u16) -> u8 {
    if self.mode == Mode::AccessVram {
      return UNDEFINED_READ;
    }
    self.tile_map1[reladdr as usize]
  }
  pub fn read_tile_map2(&self, reladdr: u16) -> u8 {
    if self.mode == Mode::AccessVram {
      return UNDEFINED_READ;
    }
    self.tile_map2[reladdr as usize]
  }
  pub fn read_oam(&self, reladdr: u16) -> u8 {
    if self.mode == Mode::AccessVram || self.mode == Mode::AccessOam {
      return UNDEFINED_READ;
    }
    let sprite = &self.oam[reladdr as usize / 4];
    match reladdr as usize % 4 {
      3 => sprite.flags.bits(),
      2 => sprite.tile_num,
      1 => sprite.x.wrapping_add(8),
      _ => sprite.y.wrapping_add(16)
    }
  }
  fn switch_mode(&mut self, mode: Mode, irq: &mut Irq) {
    self.mode = mode;
    self.cycles += self.mode.cycles(self.scroll_x);
    match self.mode {
      Mode::AccessOam => {
        if self.stat.contains(STAT_ACCESS_OAM_INT) {
          irq.request_interrupt(Interrupt::LcdStat);
        }
      },
      Mode::HBlank => {
      },
      Mode::VBlank => {
        irq.request_interrupt(Interrupt::VBlank);
        if self.stat.contains(STAT_VBLANK_INT) {
          irq.request_interrupt(Interrupt::LcdStat);
        }
      },
      _ => ()
    }
  }
  pub fn emulate(&mut self, irq: &mut Irq, emu_events: &mut EmuEvents) {
    if !self.control.contains(CTRL_LCD_ON) {
      return;
    }

    self.cycles -= 1;
    if self.cycles == 1 && self.mode == Mode::AccessVram {
      // STAT mode=0 interrupt happens one cycle before the actual mode switch!
      if self.stat.contains(STAT_HBLANK_INT) {
        irq.request_interrupt(Interrupt::LcdStat);
      }
    }
    if self.cycles > 0 {
      return;
    }

    match self.mode {
      Mode::AccessOam => {
        self.switch_mode(Mode::AccessVram, irq)
      },
      Mode::AccessVram => {
        self.draw_line();
        self.switch_mode(Mode::HBlank, irq)
      },
      Mode::HBlank => {
        self.current_line += 1;
        if self.current_line < 144 {
          self.switch_mode(Mode::AccessOam, irq);
        } else {
          emu_events.insert(EE_VSYNC);
          self.switch_mode(Mode::VBlank, irq);
        }
        self.check_compare_interrupt(irq);
      },
      Mode::VBlank => {
        self.current_line += 1;
        if self.current_line > 153 {
          self.current_line = 0;
          self.switch_mode(Mode::AccessOam, irq);
        } else {
          self.cycles += VBLANK_LINE_CYCLES;
        }
        self.check_compare_interrupt(irq);
      }
    };
  }
  fn check_compare_interrupt(&mut self, irq: &mut Irq) {
    if self.current_line != self.compare_line {
      self.stat.remove(STAT_COMPARE);
    } else {
      self.stat.insert(STAT_COMPARE);
      if self.stat.contains(STAT_COMPARE_INT) {
        irq.request_interrupt(Interrupt::LcdStat);
      }
    }
  }
  fn draw_line(&mut self) {
    let slice_start = gameboy::SCREEN_WIDTH * self.current_line as usize;
    let slice_end = gameboy::SCREEN_WIDTH + slice_start;
    let pixels = &mut self.back_buffer[slice_start .. slice_end];
    let mut bg_prio = [false; gameboy::SCREEN_WIDTH];

    if self.control.contains(CTRL_BG_ON) {
      let addr_select = self.control.contains(CTRL_BG_ADDR);
      let tile_map =
        if self.control.contains(CTRL_BG_MAP) { &self.tile_map2 }
        else { &self.tile_map1 };

      let y = self.current_line.wrapping_add(self.scroll_y);
      let row = (y / 8) as usize;
      for i in (0..gameboy::SCREEN_WIDTH) {
        let x = (i as u8).wrapping_add(self.scroll_x);
        let col = (x / 8) as usize;
        let raw_tile_num = tile_map[row * 32 + col];

        let tile_num =
          if addr_select { raw_tile_num as usize }
          else { 128 + ((raw_tile_num as i8 as i16) + 128) as usize };
        let tile = &self.character_ram[tile_num];

        let line = (y % 8) * 2;
        let data1 = tile.data[(line as u16) as usize];
        let data2 = tile.data[(line as u16 + 1) as usize];

        let bit = (x % 8).wrapping_sub(7).wrapping_mul(0xff) as usize;
        let color_value = (data2.bit(bit) << 1) | data1.bit(bit);
        let raw_color = Color::from_u8(color_value);
        let color = self.bg_palette.get(&raw_color);
        bg_prio[i] = raw_color != Color::Off;
        pixels[i] = color;
      }
    }
    if self.control.contains(CTRL_WINDOW_ON) && self.window_y <= self.current_line {
      let window_x = self.window_x.wrapping_sub(7);
      let addr_select = self.control.contains(CTRL_BG_ADDR);
      let tile_map =
        if self.control.contains(CTRL_WINDOW_MAP) { &self.tile_map2 }
        else { &self.tile_map1 };

      let y = self.current_line - self.window_y;
      let row = (y / 8) as usize;
      for i in ((window_x as usize)..gameboy::SCREEN_WIDTH) {
        let mut x = (i as u8).wrapping_add(self.scroll_x);
        if x >= window_x {
          x = i as u8 - window_x;
        }
        let col = (x / 8) as usize;
        let raw_tile_num = tile_map[row * 32 + col];

        let tile_num =
          if addr_select { raw_tile_num as usize }
          else { 128 + ((raw_tile_num as i8 as i16) + 128) as usize };
        let tile = &self.character_ram[tile_num];

        let line = (y % 8) * 2;
        let data1 = tile.data[(line as u16) as usize];
        let data2 = tile.data[(line as u16 + 1) as usize];

        let bit = (x % 8).wrapping_sub(7).wrapping_mul(0xff) as usize;
        let color_value = (data2.bit(bit) << 1) | data1.bit(bit);
        let raw_color = Color::from_u8(color_value);
        let color = self.bg_palette.get(&raw_color);
        bg_prio[i] = raw_color != Color::Off;
        pixels[i] = color;
      }
    }
    if self.control.contains(CTRL_OBJ_ON) {
      let size =
        if self.control.contains(CTRL_OBJ_SIZE) { 16 } else { 8 };

      let current_line = self.current_line;

      let mut sprites_to_draw: Vec<(usize, &Sprite)> = self.oam.iter()
        .filter(|sprite| current_line.wrapping_sub(sprite.y) < size)
        .take(10)
        .enumerate()
        .collect();

      sprites_to_draw.sort_by(|&(a_index, a), &(b_index, b)| {
        match a.x.cmp(&b.x) {
          // If X coordinates are the same, use OAM index as priority (low index => draw last)
          Ordering::Equal => a_index.cmp(&b_index).reverse(),
          // Use X coordinate as priority (low X => draw last)
          other => other.reverse()
        }
      });

      for (_, sprite) in sprites_to_draw {
        let palette =
          if sprite.flags.contains(SPRITE_PALETTE) { &self.obj_palette1 }
          else { &self.obj_palette0 };
        let mut tile_num = sprite.tile_num as usize;
        let mut line =
          if sprite.flags.contains(SPRITE_FLIPY) {
            size - (self.current_line - sprite.y) - 1
          } else {
            self.current_line - sprite.y
          };
        if line >= 8 {
          tile_num += 1;
          line -= 8;
        }
        line *= 2;
        let tile = &self.character_ram[tile_num];
        let data1 = tile.data[(line as u16) as usize];
        let data2 = tile.data[(line as u16 + 1) as usize];

        for x in (0..8).rev() {
          let bit =
            if sprite.flags.contains(SPRITE_FLIPX) {
              7 - x
            } else { x } as usize;
          let raw_color = Color::from_u8((data2.bit(bit) << 1) | data1.bit(bit));
          let color = palette.get(&raw_color);
          let target_x = sprite.x.wrapping_add(7 - x);
          if target_x < gameboy::SCREEN_WIDTH as u8 && raw_color != Color::Off {
            if !sprite.flags.contains(SPRITE_PRIORITY) || !bg_prio[target_x as usize] {
              pixels[target_x as usize] = color;
            }
          }
        }
      }
    }
  }
}

impl fmt::Debug for Gpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "LCDC:{:08b} STAT:{:08b} LY:{:02x} ", self.get_control(), self.get_stat(), self.get_current_line())
  }
}
