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
use std::fmt;

use config::HardwareConfig;
use emulation::{EmuDuration, EmuEvents, EmuTime};
use frontend::{GbKey};
use gameboy;
use gameboy::{HiramData, HIRAM_EMPTY};
use hardware::apu::Apu;
use hardware::bootrom::Bootrom;
use hardware::cartridge::Cartridge;
use hardware::gpu::Gpu;
use hardware::work_ram::WorkRam;
use hardware::irq::{
  Irq, Interrupt
};
use hardware::joypad::Joypad;
use hardware::serial::Serial;
use hardware::timer::Timer;

mod apu;
mod bootrom;
mod cartridge;
mod gpu;
mod work_ram;
pub mod irq;
mod joypad;
mod serial;
mod timer;

pub trait Bus {
  fn fetch_cycle(&mut self, u16) -> u8;
  fn read_cycle(&mut self, u16) -> u8;
  fn write_cycle(&mut self, u16, u8);
  fn emulate(&mut self);
  fn read(&self, u16) -> u8;
  fn ack_interrupt(&mut self) -> Option<Interrupt>;
  fn has_interrupt(&self) -> bool;
  fn trigger_emu_events(&mut self, EmuEvents);
}

pub struct Hardware {
  pub bootrom: Bootrom,
  pub cartridge: Cartridge,
  work_ram: WorkRam,
  hiram: HiramData,
  gpu: Gpu,
  apu: Apu,
  joypad: Joypad,
  serial: Serial,
  pub timer: Timer,
  oam_dma: OamDma,
  irq: Irq,
  emu_events: EmuEvents,
  emu_time: EmuTime,
}

#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum OamDmaState {
  Inactive,
  Requested,
  Active
}

struct OamDma {
  state: OamDmaState,
  source: u8,
  addr: u8
}

impl OamDma {
  fn new() -> OamDma {
    OamDma {
      state: OamDmaState::Inactive,
      source: 0x00,
      addr: 0x00
    }
  }
  fn start(&mut self, value: u8) {
    if value > 0xdf {
      panic!("Invalid OAM DMA {:02x}", value);
    }
    self.source = value;
    self.state = OamDmaState::Requested;
  }
  fn is_oam_available(&self) -> bool { self.addr == 0x00 }
}

impl Hardware {
  pub fn new(config: HardwareConfig) -> Hardware {
    Hardware {
      bootrom: Bootrom::new(config.bootrom),
      cartridge: Cartridge::new(config.cartridge),
      work_ram: WorkRam::new(),
      hiram: HIRAM_EMPTY,
      gpu: Gpu::new(),
      apu: Apu::new(),
      joypad: Joypad::new(),
      serial: Serial::new(),
      timer: Timer::new(),
      oam_dma: OamDma::new(),
      irq: Irq::new(),
      emu_events: EmuEvents::empty(),
      emu_time: EmuTime::zero(),
    }
  }
  pub fn ack_emu_events(&mut self) -> EmuEvents {
    let events = self.emu_events;
    self.emu_events = EmuEvents::empty();
    events
  }  
  pub fn emu_events(&self) -> EmuEvents { self.emu_events }
  pub fn emu_time(&self) -> EmuTime { self.emu_time }
  pub fn screen_buffer(&self) -> &gameboy::ScreenBuffer { &self.gpu.back_buffer }
  pub fn key_down(&mut self, key: GbKey) {
    self.joypad.key_down(key, &mut self.irq);
  }
  pub fn key_up(&mut self, key: GbKey) {
    self.joypad.key_up(key);
  }
  fn write_internal(&mut self, addr: u16, value: u8) {
    match addr >> 8 {
      0x00 ... 0x7f => self.cartridge.write_control(addr, value),
      0x80 ... 0x97 => self.gpu.write_character_ram(addr - 0x8000, value),
      0x98 ... 0x9b => self.gpu.write_tile_map1(addr - 0x9800, value),
      0x9c ... 0x9f => self.gpu.write_tile_map2(addr - 0x9c00, value),
      0xa0 ... 0xbf => self.cartridge.write_ram(addr - 0xa000, value),
      0xc0 ... 0xcf => self.work_ram.write_lower(addr, value),
      0xd0 ... 0xdf => self.work_ram.write_upper(addr, value),
      // Echo RAM
      0xe0 ... 0xef => self.work_ram.write_lower(addr, value),
      0xf0 ... 0xfd => self.work_ram.write_upper(addr, value),
      0xfe => {
        match addr & 0xff {
          0x00 ... 0x9f =>
            if self.oam_dma.is_oam_available() {
              self.gpu.write_oam(addr as u8, value)
            },
          _ => ()
          // _ => panic!("Unsupported write at ${:04x} = {:02x}", addr, value)
        }
      },
      0xff => {
        match addr & 0xff {
          0x00 => self.joypad.set_register(value),
          0x01 => self.serial.set_data(value),
          0x02 => self.serial.set_control(value),
          0x04 => self.timer.reset_divider(),
          0x05 => self.timer.set_counter(value),
          0x06 => self.timer.set_modulo(value),
          0x07 => self.timer.set_control(value),
          0x0f => self.irq.set_interrupt_flag(value),
          0x10 ... 0x3f => self.apu.write(addr, value),
          0x40 => self.gpu.set_control(value),
          0x41 => self.gpu.set_stat(value),
          0x42 => self.gpu.set_scroll_y(value),
          0x43 => self.gpu.set_scroll_x(value),
          0x44 => self.gpu.reset_current_line(),
          0x45 => self.gpu.set_compare_line(value),
          0x46 => self.oam_dma.start(value),
          0x47 => self.gpu.set_bg_palette(value),
          0x48 => self.gpu.set_obj_palette0(value),
          0x49 => self.gpu.set_obj_palette1(value),
          0x4a => self.gpu.set_window_y(value),
          0x4b => self.gpu.set_window_x(value),
          0x50 => self.bootrom.deactivate(),
          0x80 ... 0xfe => self.hiram[(addr & 0x7f) as usize] = value,
          0xff => self.irq.set_interrupt_enable(value),
          _ => ()
        }
      },
      _ => panic!("Unsupported write at ${:04x} = {:02x}", addr, value)
    }
  }
  fn read_internal(&self, addr: u16) -> u8 {
    match addr >> 8 {
      0x00 ... 0x3f => {
        if addr < 0x100 && self.bootrom.is_active() { self.bootrom[addr] }
        else { self.cartridge.read_rom_bank0(addr) }
      },
      0x40 ... 0x7f => self.cartridge.read_rom_bankx(addr - 0x4000),
      0x80 ... 0x97 => self.gpu.read_character_ram(addr - 0x8000),
      0x98 ... 0x9b => self.gpu.read_tile_map1(addr - 0x9800),
      0x9c ... 0x9f => self.gpu.read_tile_map2(addr - 0x9c00),
      0xa0 ... 0xbf => self.cartridge.read_ram(addr - 0xa000),
      0xc0 ... 0xcf => self.work_ram.read_lower(addr),
      0xd0 ... 0xdf => self.work_ram.read_upper(addr),
      // Echo RAM
      0xe0 ... 0xef => self.work_ram.read_lower(addr),
      0xf0 ... 0xfd => self.work_ram.read_upper(addr),
      0xfe => {
        match addr & 0xff {
          0x00 ... 0x9f =>
            if !self.oam_dma.is_oam_available() { 0xff } else {
              self.gpu.read_oam(addr as u8)
            },
          // 0x00 ... 0x9f => handle_oam!(),
          // 0xa0 ... 0xff => handle_unusable!(),
          _ => panic!("Unsupported read at ${:04x}", addr)
        }
      },
      0xff => {
        match addr & 0xff {
          0x00 => self.joypad.get_register(),
          0x01 => self.serial.get_data(),
          0x02 => self.serial.get_control(),
          0x04 => self.timer.get_divider(),
          0x05 => self.timer.get_counter(),
          0x06 => self.timer.get_modulo(),
          0x07 => self.timer.get_control(),
          0x0f => self.irq.get_interrupt_flag(),
          0x10 ... 0x3f => self.apu.read(addr),
          0x40 => self.gpu.get_control(),
          0x41 => self.gpu.get_stat(),
          0x42 => self.gpu.get_scroll_y(),
          0x43 => self.gpu.get_scroll_x(),
          0x44 => self.gpu.get_current_line(),
          0x45 => self.gpu.get_compare_line(),
          0x47 => self.gpu.get_bg_palette(),
          0x48 => self.gpu.get_obj_palette0(),
          0x49 => self.gpu.get_obj_palette1(),
          0x4a => self.gpu.get_window_y(),
          0x4b => self.gpu.get_window_x(),
          0x80 ... 0xfe => self.hiram[(addr & 0x7f) as usize],
          0xff => self.irq.get_interrupt_enable(),
          _ => 0xff
        }
      },
      _ => panic!("Unsupported read at ${:04x}", addr)
    }
  }
}

impl Bus for Hardware {
  fn fetch_cycle(&mut self, addr: u16) -> u8 {
    self.emu_time += EmuDuration::machine_cycles(1);
    self.emulate();
    self.read_internal(addr)
  }
  fn read_cycle(&mut self, addr: u16) -> u8 {
    self.emu_time += EmuDuration::machine_cycles(1);
    self.emulate();
    self.read_internal(addr)
  }
  fn write_cycle(&mut self, addr: u16, value: u8) {
    self.emu_time += EmuDuration::machine_cycles(1);
    self.emulate();
    self.write_internal(addr, value)
  }
  fn read(&self, addr: u16) -> u8 {
    self.read_internal(addr)
  }
  fn emulate(&mut self) {
    self.emu_time += EmuDuration::machine_cycles(1);
    match self.oam_dma.state {
      OamDmaState::Requested => {
        self.oam_dma.addr = 0x00;
        self.oam_dma.state = OamDmaState::Active;
      },
      OamDmaState::Active if self.oam_dma.addr >= 0xa0 => {
        self.oam_dma.addr = 0x00;
        self.oam_dma.state = OamDmaState::Inactive;
      },
      OamDmaState::Active => {
        let source_addr = ((self.oam_dma.source as u16) << 8) | self.oam_dma.addr as u16;
        let value = self.read_internal(source_addr);
        self.gpu.write_oam(self.oam_dma.addr, value);
        self.oam_dma.addr += 1;
      },
      _ => ()
    }
    self.timer.emulate(&mut self.irq);
    self.gpu.emulate(&mut self.irq, &mut self.emu_events);
    self.apu.emulate();
  }
  fn ack_interrupt(&mut self) -> Option<Interrupt> { self.irq.ack_interrupt() }
  fn has_interrupt(&self) -> bool { self.irq.has_interrupt() }
  fn trigger_emu_events(&mut self, events: EmuEvents) { self.emu_events.insert(events) }
}

impl fmt::Debug for Hardware {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.gpu)
  }
}
