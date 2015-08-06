use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::render::{BlendMode, Renderer, Texture, TextureAccess};
use std::collections::BTreeMap;

use super::FrontendResult;

/// Generated from Inconsolata 22px
/// With BMFont v1.14 beta
/// Settings:
/// *  Outline thickness: 1
/// *  Force offsets to zero
mod inconsolata_20;

const CHAR_WIDTH: u32 = 12;
const CHAR_HEIGHT: u32 = 22;

pub enum TextAlign {
  Left,
  Right
}

pub struct Font {
  outline: Texture,
  glyph: Texture,
  offsets: BTreeMap<u32, (i32, i32)>
}

fn load_image(data: &[u8], renderer: &Renderer) -> FrontendResult<Texture> {
  let mut texture =
    try!(renderer.create_texture(PixelFormatEnum::RGBA8888, TextureAccess::Static, (256, 256)));

  try!(texture.update(None, data, 256 * 4));
  texture.set_blend_mode(BlendMode::Blend);
  Ok(texture)
}

impl Font {
  pub fn init(renderer: &Renderer) -> FrontendResult<Font> {
    let mut outline = try!(load_image(inconsolata_20::OUTLINE_BYTES, renderer));
    outline.set_color_mod(64, 0, 0);

    let mut glyph = try!(load_image(inconsolata_20::GLYPH_BYTES, renderer));
    glyph.set_color_mod(255, 127, 0);

    let offsets = inconsolata_20::offsets();

    Ok(Font {
      outline: outline,
      glyph: glyph,
      offsets: offsets
    })
  }
  pub fn draw_char(&self, renderer: &mut Renderer, ch: char, dst_x: i32, dst_y: i32) -> FrontendResult<()> {
    let value = ch as u32;
    match self.offsets.get(&value) {
      Some(&(x, y)) => {
        let src_rect = try!(Rect::new(x, y, CHAR_WIDTH, CHAR_HEIGHT));
        let dst_rect = try!(Rect::new(dst_x, dst_y, CHAR_WIDTH, CHAR_HEIGHT));
        renderer.copy(&self.outline, src_rect, dst_rect);
        renderer.copy(&self.glyph, src_rect, dst_rect);
      }
      _ => (),
    }
    Ok(())
  }
  pub fn draw_text(&self, renderer: &mut Renderer, x: i32, y: i32, alignment: TextAlign, text: &str) -> FrontendResult<()> {
    let final_x =
      match alignment {
        TextAlign::Left => x,
        TextAlign::Right => x - text.len() as i32 * CHAR_WIDTH as i32
      };
    for (i, ch) in text.chars().enumerate() {
      try!(self.draw_char(renderer, ch, (final_x + CHAR_WIDTH as i32 * i as i32), y));
    }
    Ok(())
  }
}
