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
use std::fmt;

use crate::config::HardwareConfig;
use crate::emulation::{EmuEvents, EmuTime};
use crate::gameboy;
use crate::gameboy::{HiramData, HIRAM_EMPTY};
use crate::hardware::apu::Apu;
use crate::hardware::bootrom::Bootrom;
pub use crate::hardware::bootrom::BootromData;
use crate::hardware::cartridge::Cartridge;
use crate::hardware::gpu::Gpu;
use crate::hardware::irq::{Interrupt, InterruptRequest, Irq};
use crate::hardware::joypad::Joypad;
use crate::hardware::serial::Serial;
use crate::hardware::timer::{Div, Tac, Tima, Timer, Tma};
use crate::hardware::work_ram::WorkRam;
use crate::GbKey;

mod apu;
mod bootrom;
mod cartridge;
mod gpu;
pub mod irq;
mod joypad;
mod serial;
mod timer;
mod work_ram;

pub trait MappedHardware<A, D: From<u8> + Into<u8> = u8> {
  fn read_cycle<I: InterruptRequest>(&mut self, addr: A, intr_req: &mut I) -> D;
  fn write_cycle<I: InterruptRequest>(&mut self, addr: A, data: D, intr_req: &mut I);
}

pub trait Bus {
  fn read_cycle(&mut self, addr: u16) -> u8;
  fn write_cycle(&mut self, addr: u16, data: u8);
  fn tick_cycle(&mut self);
  fn get_mid_interrupt(&self) -> Option<Interrupt>;
  fn get_end_interrupt(&self) -> Option<Interrupt>;
  fn ack_interrupt(&mut self, interrupt: Interrupt);
  fn trigger_emu_events(&mut self, events: EmuEvents);
}

#[derive(Clone)]
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

#[derive(Clone)]
struct OamDma {
  bus: Option<ExternalBus>,
  source: u8,
  requested: Option<u8>,
  starting: Option<u8>,
  addr: u16,
}

impl OamDma {
  fn new() -> OamDma {
    OamDma {
      bus: None,
      source: 0xff,
      requested: None,
      starting: None,
      addr: 0x0000,
    }
  }
  fn request(&mut self, value: u8) {
    self.requested = Some(value);
  }
  fn start(&mut self, value: u8) {
    self.bus = Some(ExternalBus::from_oam_dma_source(value));
    self.source = value;
    self.addr = (value as u16) << 8;
  }
  fn stop(&mut self) {
    self.bus = None;
  }
  fn emulate(&mut self) -> Option<u16> {
    if self.is_active() {
      let addr = self.addr;
      self.addr = self.addr.wrapping_add(1);
      if self.addr as u8 >= 0xa0 {
        self.stop();
      }
      Some(addr)
    } else {
      None
    }
  }
  fn is_active(&self) -> bool {
    self.bus.is_some()
  }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ExternalBus {
  Video,
  Main,
}

impl ExternalBus {
  fn from_oam_dma_source(source: u8) -> ExternalBus {
    match source {
      0x80...0x9f => ExternalBus::Video,
      _ => ExternalBus::Main,
    }
  }
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
  pub fn emu_events(&self) -> EmuEvents {
    self.emu_events
  }
  pub fn emu_time(&self) -> EmuTime {
    self.emu_time
  }
  pub fn screen_buffer(&self) -> &gameboy::ScreenBuffer {
    &self.gpu.back_buffer
  }
  pub fn key_down(&mut self, key: GbKey) {
    self.joypad.key_down(key, &mut self.irq);
  }
  pub fn key_up(&mut self, key: GbKey) {
    self.joypad.key_up(key);
  }
  fn write_internal(&mut self, addr: u16, value: u8) {
    match addr >> 8 {
      0x00 if self.bootrom.is_active() => self.write_cycle_generic(|_| ()),
      0x00...0x7f => self.write_cycle_generic(|hw| hw.cartridge.write_control(addr, value)),
      0x80...0x97 => {
        self.write_cycle_generic(|hw| hw.gpu.write_character_ram(addr - 0x8000, value))
      }
      0x98...0x9b => self.write_cycle_generic(|hw| hw.gpu.write_tile_map1(addr - 0x9800, value)),
      0x9c...0x9f => self.write_cycle_generic(|hw| hw.gpu.write_tile_map2(addr - 0x9c00, value)),
      0xa0...0xbf => self.write_cycle_generic(|hw| hw.cartridge.write_a000_bfff(addr, value)),
      0xc0...0xcf => self.write_cycle_generic(|hw| hw.work_ram.write_lower(addr, value)),
      0xd0...0xdf => self.write_cycle_generic(|hw| hw.work_ram.write_upper(addr, value)),
      // Echo RAM
      0xe0...0xef => self.write_cycle_generic(|hw| hw.work_ram.write_lower(addr, value)),
      0xf0...0xfd => self.write_cycle_generic(|hw| hw.work_ram.write_upper(addr, value)),
      0xfe => {
        match addr & 0xff {
          0x00...0x9f => self.write_cycle_generic(|hw| {
            if !hw.oam_dma.is_active() {
              hw.gpu.write_oam(addr as u8, value)
            }
          }),
          _ => (), // _ => panic!("Unsupported write at ${:04x} = {:02x}", addr, value)
        }
      }
      0xff => match addr & 0xff {
        0x00 => self.write_cycle_generic(|hw| hw.joypad.set_register(value)),
        0x01 => self.write_cycle_generic(|hw| hw.serial.set_data(value)),
        0x02 => self.write_cycle_generic(|hw| hw.serial.set_control(value)),
        0x04 => self.write_cycle_timer(Div, value),
        0x05 => self.write_cycle_timer(Tima, value),
        0x06 => self.write_cycle_timer(Tma, value),
        0x07 => self.write_cycle_timer(Tac, value),
        0x0f => self.write_cycle_generic(|hw| hw.irq.set_interrupt_flag(value)),
        0x10...0x3f => self.write_cycle_generic(|hw| hw.apu.write(addr, value)),
        0x40 => self.write_cycle_generic(|hw| hw.gpu.set_control(value)),
        0x41 => self.write_cycle_generic(|hw| hw.gpu.set_stat(value)),
        0x42 => self.write_cycle_generic(|hw| hw.gpu.set_scroll_y(value)),
        0x43 => self.write_cycle_generic(|hw| hw.gpu.set_scroll_x(value)),
        0x44 => self.write_cycle_generic(|hw| hw.gpu.reset_current_line()),
        0x45 => self.write_cycle_generic(|hw| hw.gpu.set_compare_line(value)),
        0x46 => self.write_cycle_generic(|hw| hw.oam_dma.request(value)),
        0x47 => self.write_cycle_generic(|hw| hw.gpu.set_bg_palette(value)),
        0x48 => self.write_cycle_generic(|hw| hw.gpu.set_obj_palette0(value)),
        0x49 => self.write_cycle_generic(|hw| hw.gpu.set_obj_palette1(value)),
        0x4a => self.write_cycle_generic(|hw| hw.gpu.set_window_y(value)),
        0x4b => self.write_cycle_generic(|hw| hw.gpu.set_window_x(value)),
        0x50 => {
          if self.bootrom.is_active() {
            self.write_cycle_generic(|hw| hw.bootrom.deactivate());
            self.emu_events.insert(EmuEvents::BOOTROM_DISABLED);
          }
        }
        0x80...0xfe => self.write_cycle_generic(|hw| hw.hiram[(addr as usize) & 0x7f] = value),
        0xff => self.write_cycle_generic(|hw| hw.irq.set_interrupt_enable(value)),
        _ => (),
      },
      _ => panic!("Unsupported write at ${:04x} = {:02x}", addr, value),
    }
  }
  fn read_internal(&mut self, addr: u16) -> u8 {
    match addr >> 8 {
      0x00 if self.bootrom.is_active() => self.read_cycle_generic(|hw| hw.bootrom[addr]),
      0x00...0x3f => self.read_cycle_generic(|hw| hw.cartridge.read_0000_3fff(addr)),
      0x40...0x7f => self.read_cycle_generic(|hw| hw.cartridge.read_4000_7fff(addr)),
      0x80...0x97 => self.read_cycle_generic(|hw| hw.gpu.read_character_ram(addr - 0x8000)),
      0x98...0x9b => self.read_cycle_generic(|hw| hw.gpu.read_tile_map1(addr - 0x9800)),
      0x9c...0x9f => self.read_cycle_generic(|hw| hw.gpu.read_tile_map2(addr - 0x9c00)),
      0xa0...0xbf => self.read_cycle_generic(|hw| hw.cartridge.read_a000_bfff(addr, 0xff)),
      0xc0...0xcf => self.read_cycle_generic(|hw| hw.work_ram.read_lower(addr)),
      0xd0...0xdf => self.read_cycle_generic(|hw| hw.work_ram.read_upper(addr)),
      // Echo RAM
      0xe0...0xef => self.read_cycle_generic(|hw| hw.work_ram.read_lower(addr)),
      0xf0...0xfd => self.read_cycle_generic(|hw| hw.work_ram.read_upper(addr)),
      0xfe => {
        match addr & 0xff {
          0x00...0x9f => self.read_cycle_generic(|hw| {
            if hw.oam_dma.is_active() {
              0xff
            } else {
              hw.gpu.read_oam(addr as u8)
            }
          }),
          // 0x00 ... 0x9f => handle_oam!(),
          // 0xa0 ... 0xff => handle_unusable!(),
          _ => panic!("Unsupported read at ${:04x}", addr),
        }
      }
      0xff => match addr & 0xff {
        0x00 => self.read_cycle_generic(|hw| hw.joypad.get_register()),
        0x01 => self.read_cycle_generic(|hw| hw.serial.get_data()),
        0x02 => self.read_cycle_generic(|hw| hw.serial.get_control()),
        0x04 => self.read_cycle_timer(Div),
        0x05 => self.read_cycle_timer(Tima),
        0x06 => self.read_cycle_timer(Tma),
        0x07 => self.read_cycle_timer(Tac),
        0x0f => self.read_cycle_generic(|hw| hw.irq.get_interrupt_flag()),
        0x10...0x3f => self.read_cycle_generic(|hw| hw.apu.read(addr)),
        0x40 => self.read_cycle_generic(|hw| hw.gpu.get_control()),
        0x41 => self.read_cycle_generic(|hw| hw.gpu.get_stat()),
        0x42 => self.read_cycle_generic(|hw| hw.gpu.get_scroll_y()),
        0x43 => self.read_cycle_generic(|hw| hw.gpu.get_scroll_x()),
        0x44 => self.read_cycle_generic(|hw| hw.gpu.get_current_line()),
        0x45 => self.read_cycle_generic(|hw| hw.gpu.get_compare_line()),
        0x46 => self.read_cycle_generic(|hw| hw.oam_dma.source),
        0x47 => self.read_cycle_generic(|hw| hw.gpu.get_bg_palette()),
        0x48 => self.read_cycle_generic(|hw| hw.gpu.get_obj_palette0()),
        0x49 => self.read_cycle_generic(|hw| hw.gpu.get_obj_palette1()),
        0x4a => self.read_cycle_generic(|hw| hw.gpu.get_window_y()),
        0x4b => self.read_cycle_generic(|hw| hw.gpu.get_window_x()),
        0x80...0xfe => self.read_cycle_generic(|hw| hw.hiram[(addr as usize) & 0x7f]),
        0xff => self.read_cycle_generic(|hw| hw.irq.get_interrupt_enable()),
        _ => self.read_cycle_generic(|_| 0xff),
      },
      _ => panic!("Unsupported read at ${:04x}", addr),
    }
  }
  fn emulate_oam_dma(&mut self) {
    if let Some(addr) = self.oam_dma.emulate() {
      let value = match addr >> 8 {
        0x00...0x3f => self.cartridge.read_0000_3fff(addr),
        0x40...0x7f => self.cartridge.read_4000_7fff(addr),
        0x80...0x97 => self.gpu.read_character_ram(addr - 0x8000),
        0x98...0x9b => self.gpu.read_tile_map1(addr - 0x9800),
        0x9c...0x9f => self.gpu.read_tile_map2(addr - 0x9c00),
        0xa0...0xbf => self.cartridge.read_a000_bfff(addr, 0xff),
        0xc0...0xcf => self.work_ram.read_lower(addr),
        0xd0...0xdf => self.work_ram.read_upper(addr),
        0xe0...0xef => self.work_ram.read_lower(addr),
        0xf0...0xff => self.work_ram.read_upper(addr),
        _ => unreachable!("Unreachable OAM DMA read from ${:04x}", addr),
      };
      self.gpu.write_oam(addr as u8, value);
    }
    if let Some(source) = self.oam_dma.starting.take() {
      self.oam_dma.start(source);
    }
    if let Some(source) = self.oam_dma.requested.take() {
      self.oam_dma.starting = Some(source);
    }
  }
  fn emulate_internal(&mut self) {
    self.emu_time += EmuTime::from_machine_cycles(1);
    self.emulate_oam_dma();
    self.gpu.emulate(&mut self.irq, &mut self.emu_events);
    self.apu.emulate();
  }
  fn read_cycle_generic<F: FnOnce(&mut Self) -> u8>(&mut self, f: F) -> u8 {
    self.emulate_internal();
    self.timer.tick_cycle(&mut self.irq);
    f(self)
  }
  fn read_cycle_timer<T>(&mut self, addr: T) -> u8
  where
    Timer: MappedHardware<T>,
  {
    self.emulate_internal();
    self.timer.read_cycle(addr, &mut self.irq)
  }
  fn write_cycle_generic<F: FnOnce(&mut Self)>(&mut self, f: F) {
    self.emulate_internal();
    self.timer.tick_cycle(&mut self.irq);
    f(self)
  }
  fn write_cycle_timer<T>(&mut self, addr: T, value: u8)
  where
    Timer: MappedHardware<T>,
  {
    self.emulate_internal();
    self.timer.write_cycle(addr, value, &mut self.irq)
  }
}

impl Bus for Hardware {
  fn read_cycle(&mut self, addr: u16) -> u8 {
    self.irq.begin_cycle();
    self.read_internal(addr)
  }
  fn write_cycle(&mut self, addr: u16, value: u8) {
    self.irq.begin_cycle();
    self.write_internal(addr, value)
  }
  fn tick_cycle(&mut self) {
    self.irq.begin_cycle();
    self.emulate_internal();
    self.timer.tick_cycle(&mut self.irq);
  }
  fn get_mid_interrupt(&self) -> Option<Interrupt> {
    self.irq.get_mid_interrupt()
  }
  fn get_end_interrupt(&self) -> Option<Interrupt> {
    self.irq.get_end_interrupt()
  }
  fn ack_interrupt(&mut self, interrupt: Interrupt) {
    self.irq.ack_interrupt(interrupt);
  }
  fn trigger_emu_events(&mut self, events: EmuEvents) {
    self.emu_events.insert(events)
  }
}

impl fmt::Debug for Hardware {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.gpu)
  }
}
