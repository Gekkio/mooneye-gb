use sdl2::pixels::PixelFormatFlag;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Renderer, Texture, TextureAccess};
use std::collections::VecMap;

use super::BackendResult;

/// Generated from Inconsolata 22px
/// With BMFont v1.14 beta
/// Settings:
/// *  Outline thickness: 1
/// *  Force offsets to zero
mod inconsolata_20;

const CHAR_WIDTH: i32 = 12;
const CHAR_HEIGHT: i32 = 22;

pub enum TextAlign {
  Left,
  Right
}

pub struct Font {
  pub outline: Texture,
  pub glyph: Texture,
  offsets: VecMap<(i32, i32)>
}

fn load_image(data: &[u8], renderer: &Renderer) -> BackendResult<Texture> {
  let texture =
    try!(renderer.create_texture(PixelFormatFlag::RGBA8888, TextureAccess::Static, 256, 256));

  try!(texture.update(None, data, 256 * 4));
  try!(texture.set_blend_mode(BlendMode::Blend));
  Ok(texture)
}

impl Font {
  pub fn init(renderer: &Renderer) -> BackendResult<Font> {
    let outline = try!(load_image(inconsolata_20::OUTLINE_BYTES, renderer));
    try!(outline.set_color_mod(64, 0, 0));

    let glyph = try!(load_image(inconsolata_20::GLYPH_BYTES, renderer));
    try!(glyph.set_color_mod(255, 127, 0));

    let offsets = inconsolata_20::offsets();

    Ok(Font {
      outline: outline,
      glyph: glyph,
      offsets: offsets
    })
  }
  pub fn draw_char(&self, renderer: &Renderer, ch: char, dst_x: i32, dst_y: i32) -> BackendResult<()> {
    let value = ch as usize;
    match self.offsets.get(&value) {
      Some(&(x, y)) => {
        let src_rect = Rect {
          x: x,
          y: y,
          w: CHAR_WIDTH,
          h: CHAR_HEIGHT
        };
        let dst_rect = Rect {
          x: dst_x as i32,
          y: dst_y as i32,
          w: CHAR_WIDTH,
          h: CHAR_HEIGHT
        };
        try!(renderer.copy(&self.outline, Some(src_rect), Some(dst_rect)));
        try!(renderer.copy(&self.glyph, Some(src_rect), Some(dst_rect)));
      }
      _ => (),
    }
    Ok(())
  }
  pub fn draw_text(&self, renderer: &Renderer, x: i32, y: i32, alignment: TextAlign, text: &str) -> BackendResult<()> {
    let final_x =
      match alignment {
        TextAlign::Left => x,
        TextAlign::Right => x - text.len() as i32 * CHAR_WIDTH
      };
    for (i, ch) in text.chars().enumerate() {
      try!(self.draw_char(renderer, ch, (final_x + CHAR_WIDTH * i as i32), y));
    }
    Ok(())
  }
}
