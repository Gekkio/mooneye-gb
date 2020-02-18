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
use bitflags::bitflags;

use self::ch1::Ch1;
use self::ch2::Ch2;
use self::ch3::Ch3;
use self::ch4::Ch4;

mod ch1;
mod ch2;
mod ch3;
mod ch4;
mod envelope;
mod sweep;
mod wave_duty;

#[derive(Clone)]
pub struct Apu {
  enabled: bool,
  term1_volume: Volume,
  term2_volume: Volume,
  term1_vin: bool,
  term2_vin: bool,
  term1_channels: Channels,
  term2_channels: Channels,
  ch1: Ch1,
  ch2: Ch2,
  ch3: Ch3,
  ch4: Ch4,
  cycles: usize,
}

#[derive(Clone, Copy)]
enum Volume {
  Vol0 = 0x00,
  Vol1 = 0x01,
  Vol2 = 0x02,
  Vol3 = 0x03,
  Vol4 = 0x04,
  Vol5 = 0x05,
  Vol6 = 0x06,
  Vol7 = 0x07,
}

impl Volume {
  pub fn from_u8(value: u8) -> Option<Volume> {
    use self::Volume::*;
    match value {
      0x00 => Some(Vol0),
      0x01 => Some(Vol1),
      0x02 => Some(Vol2),
      0x03 => Some(Vol3),
      0x04 => Some(Vol4),
      0x05 => Some(Vol5),
      0x06 => Some(Vol6),
      0x07 => Some(Vol7),
      _ => None,
    }
  }
}

impl Apu {
  pub fn new() -> Apu {
    Apu {
      enabled: false,
      term1_volume: Volume::Vol0,
      term2_volume: Volume::Vol0,
      term1_vin: false,
      term2_vin: false,
      term1_channels: Channels::empty(),
      term2_channels: Channels::empty(),
      ch1: Ch1::new(),
      ch2: Ch2::new(),
      ch3: Ch3::new(),
      ch4: Ch4::new(),
      cycles: 4096,
    }
  }
  pub fn read(&self, addr: u16) -> u8 {
    match addr & 0xff {
      0x10 => self.ch1.sweep.read_reg(),
      0x11 => self.ch1.read_reg1(),
      0x12 => self.ch1.envelope.read_reg(),
      0x14 => self.ch1.read_reg4(),
      0x16 => self.ch2.read_reg1(),
      0x17 => self.ch2.envelope.read_reg(),
      0x19 => self.ch2.read_reg4(),
      0x1a => self.ch3.read_reg0(),
      0x1c => self.ch3.read_reg2(),
      0x1e => self.ch3.read_reg4(),
      0x21 => self.ch4.envelope.read_reg(),
      0x22 => self.ch4.read_reg3(),
      0x23 => self.ch4.read_reg4(),
      0x24 => self.get_ctrl_volume(),
      0x25 => self.get_terminal_channels(),
      0x26 => self.get_ctrl_master(),
      0x30..=0x3f => self.ch3.read_wave_ram(addr - 0xff30),
      _ => 0xff,
    }
  }
  pub fn write(&mut self, addr: u16, value: u8) {
    if !self.enabled {
      match addr & 0xff {
        0x26 => self.set_ctrl_master(value),
        0x30..=0x3f => self.ch3.write_wave_ram(addr - 0xff30, value),
        _ => (),
      }
      return;
    }
    match addr & 0xff {
      0x10 => self.ch1.sweep.write_reg(value),
      0x11 => self.ch1.write_reg1(value),
      0x12 => self.ch1.envelope.write_reg(value),
      0x13 => self.ch1.write_reg3(value),
      0x14 => self.ch1.write_reg4(value),
      0x16 => self.ch2.write_reg1(value),
      0x17 => self.ch2.envelope.write_reg(value),
      0x18 => self.ch2.write_reg3(value),
      0x19 => self.ch2.write_reg4(value),
      0x1a => self.ch3.write_reg0(value),
      0x1b => self.ch3.write_reg1(value),
      0x1c => self.ch3.write_reg2(value),
      0x1d => self.ch3.write_reg3(value),
      0x1e => self.ch3.write_reg4(value),
      0x20 => self.ch4.write_reg1(value),
      0x21 => self.ch4.envelope.write_reg(value),
      0x22 => self.ch4.write_reg3(value),
      0x23 => self.ch4.write_reg4(value),
      0x24 => self.set_ctrl_volume(value),
      0x25 => self.set_terminal_channels(value),
      0x26 => self.set_ctrl_master(value),
      0x30..=0x3f => self.ch3.write_wave_ram(addr - 0xff30, value),
      _ => (),
    }
  }
  pub fn get_terminal_channels(&self) -> u8 {
    self.term1_channels.bits | (self.term2_channels.bits << 4)
  }
  pub fn set_terminal_channels(&mut self, value: u8) {
    self.term1_channels = Channels::from_bits_truncate(value);
    self.term2_channels = Channels::from_bits_truncate(value >> 4);
  }
  pub fn get_ctrl_master(&self) -> u8 {
    const CTRL_MASTER_MASK: u8 = 0x70;

    CTRL_MASTER_MASK
      | if self.enabled { 1 << 7 } else { 0 }
      | if self.ch4.status { 1 << 3 } else { 0 }
      | if self.ch3.status { 1 << 2 } else { 0 }
      | if self.ch2.status { 1 << 1 } else { 0 }
      | if self.ch1.status { 1 << 0 } else { 0 }
  }
  pub fn set_ctrl_master(&mut self, value: u8) {
    self.enabled = value & (1 << 7) != 0;
    if !self.enabled {
      self.ch1.reset();
      self.ch2.reset();
      self.ch3.reset();
      self.ch4.reset();
      self.term1_volume = Volume::Vol0;
      self.term2_volume = Volume::Vol0;
      self.term1_vin = false;
      self.term2_vin = false;
      self.term1_channels = Channels::empty();
      self.term2_channels = Channels::empty();
    }
  }
  pub fn get_ctrl_volume(&self) -> u8 {
    (self.term1_volume as u8)
      | if self.term1_vin { 1 << 3 } else { 0 }
      | (self.term2_volume as u8) << 4
      | if self.term2_vin { 1 << 7 } else { 0 }
  }
  pub fn set_ctrl_volume(&mut self, value: u8) {
    self.term1_volume = Volume::from_u8(value & 0x07).unwrap();
    self.term1_vin = value & (1 << 3) != 0;
    self.term2_volume = Volume::from_u8((value >> 4) & 0x07).unwrap();
    self.term2_vin = value & (1 << 7) != 0;
  }
  pub fn emulate(&mut self) {
    if self.cycles > 0 {
      self.cycles -= 1;
    } else {
      self.cycles = 4096;
      self.ch1.clock();
      self.ch2.clock();
      self.ch3.clock();
      self.ch4.clock();
    }
  }
}

bitflags!(
  struct Channels: u8 {
    const CH_1 = 1 << 0;
    const CH_2 = 1 << 1;
    const CH_3 = 1 << 2;
    const CH_4 = 1 << 3;
  }
);
