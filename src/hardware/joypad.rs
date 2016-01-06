// This file is part of Mooneye GB.
// Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use std::fmt;
use std::fmt::{Binary, Formatter, LowerHex, UpperHex};

use frontend::GbKey;
use hardware::irq::{Irq, Interrupt};

/// Gameboy joypad.
///
/// Gameboy has a dpad, and four buttons: A, B, Select, Start.
/// From a program's point of view there are two ways to interact with it:
///
/// # Joypad register (P1)
/// The joypad register can be used to access the state of four keys at a time.
///
/// # Joypad interrupt
/// Whenever a key is pressed, a joypad interrupt is requested.
pub struct Joypad {
  pressed_directional: JoypadReg,
  pressed_button: JoypadReg,
  register: JoypadReg
}

impl Joypad {
  pub fn new() -> Joypad {
    Joypad {
      pressed_directional: JoypadReg::empty(),
      pressed_button: JoypadReg::empty(),
      register: JOYPAD_INITIAL_STATE
    }
  }

  pub fn get_register(&self) -> u8 {
    // Invert bits, so 0 means "set". Unused bits in JoypadReg are 0,
    // so they automatically become 1 and no mask is needed
    !self.register.bits
  }
  pub fn set_register(&mut self, value: u8) {
    // Invert bits before converting to JoypadReg
    self.register = JoypadReg::from_bits_truncate(!value);
    self.update_register();
  }

  pub fn key_down(&mut self, key: GbKey, irq: &mut Irq) {
    self.pressed_directional.insert(JoypadReg::directional(&key));
    self.pressed_button.insert(JoypadReg::button(&key));
    self.update_register();
    irq.request_interrupt(Interrupt::Joypad);
  }
  pub fn key_up(&mut self, key: GbKey) {
    self.pressed_directional.remove(JoypadReg::directional(&key));
    self.pressed_button.remove(JoypadReg::button(&key));
    self.update_register();
  }

  /// Updates the register state based on select bits P14-P15 and the
  /// pressed buttons
  fn update_register(&mut self) {
    self.register = self.register & JOYPAD_WRITABLE;
    if self.register.contains(JOYPAD_SELECT_DIRECTIONAL) {
      self.register.insert(self.pressed_directional);
    }
    if self.register.contains(JOYPAD_SELECT_BUTTON) {
      self.register.insert(self.pressed_button);
    }
  }
}

impl Binary for Joypad {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{:08b}", !self.register.bits)
  }
}
impl LowerHex for Joypad {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{:02x}", !self.register.bits)
  }
}
impl UpperHex for Joypad {
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    write!(f, "{:02X}", !self.register.bits)
  }
}

/// P1 register
///
/// Bits are inverted in get_register/set_register, so in JoypadReg
/// a set bit is 1 as usual.
bitflags!(
  flags JoypadReg: u8 {
    const JOYPAD_P10                = 1 << 0, // P10: →, A
    const JOYPAD_P11                = 1 << 1, // P11: ←, B
    const JOYPAD_P12                = 1 << 2, // P12: ↑, Select
    const JOYPAD_P13                = 1 << 3, // P13: ↓, Start
    const JOYPAD_SELECT_DIRECTIONAL = 1 << 4, // P14: Select dpad
    const JOYPAD_SELECT_BUTTON      = 1 << 5, // P15: Select buttons

    /// Only select bits are writable
    const JOYPAD_WRITABLE =
      JOYPAD_SELECT_DIRECTIONAL.bits | JOYPAD_SELECT_BUTTON.bits,

    /// DMG: initial state 0xCF
    /// See docs/accuracy/joypad.markdown
    const JOYPAD_INITIAL_STATE = JOYPAD_WRITABLE.bits
  }
);

impl JoypadReg {
  fn directional(key: &GbKey) -> JoypadReg {
    match *key {
      GbKey::Right => JOYPAD_P10,
      GbKey::Left => JOYPAD_P11,
      GbKey::Up => JOYPAD_P12,
      GbKey::Down => JOYPAD_P13,
      _ => JoypadReg::empty()
    }
  }
  fn button(key: &GbKey) -> JoypadReg {
    match *key {
      GbKey::A => JOYPAD_P10,
      GbKey::B => JOYPAD_P11,
      GbKey::Select => JOYPAD_P12,
      GbKey::Start => JOYPAD_P13,
      _ => JoypadReg::empty()
    }
  }
}
