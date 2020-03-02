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
use crate::config::HardwareConfig;
use crate::cpu::CpuContext;
use crate::emulation::{EmuEvents, EmuTime};
use crate::gameboy;
use crate::gameboy::{HiramData, HIRAM_EMPTY};
use crate::hardware::apu::Apu;
use crate::hardware::bootrom::Bootrom;
pub use crate::hardware::bootrom::BootromData;
use crate::hardware::cartridge::Cartridge;
use crate::hardware::gpu::Gpu;
use crate::hardware::interrupts::{InterruptLine, InterruptRequest, Interrupts};
use crate::hardware::joypad::Joypad;
use crate::hardware::serial::Serial;
use crate::hardware::timer::Timer;
use crate::hardware::work_ram::WorkRam;
use crate::GbKey;
use crate::{Callbacks, CoreContext};

mod apu;
mod bootrom;
mod cartridge;
mod gpu;
pub mod interrupts;
mod joypad;
mod serial;
mod timer;
mod work_ram;

#[derive(Clone)]
pub struct Hardware {
  pub peripherals: Peripherals,
  interrupts: Interrupts,
  emu_events: EmuEvents,
  emu_time: EmuTime,
}

#[derive(Clone)]
pub struct Peripherals {
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
      0x80..=0x9f => ExternalBus::Video,
      _ => ExternalBus::Main,
    }
  }
}

impl Peripherals {
  pub fn new(config: HardwareConfig) -> Peripherals {
    Peripherals {
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
    }
  }
}

impl Hardware {
  pub fn new(config: HardwareConfig) -> Hardware {
    Hardware {
      peripherals: Peripherals::new(config),
      interrupts: Interrupts::new(),
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
    &self.peripherals.gpu.back_buffer
  }
  pub fn key_down(&mut self, key: GbKey) {
    self.peripherals.joypad.key_down(key, &mut self.interrupts);
  }
  pub fn key_up(&mut self, key: GbKey) {
    self.peripherals.joypad.key_up(key);
  }
}

pub trait PeripheralsContext: CoreContext + InterruptRequest {
  fn interrupts(&self) -> &Interrupts;
  fn interrupts_mut(&mut self) -> &mut Interrupts;
}

impl<'a> InterruptRequest for (&'a mut Interrupts, &'a mut EmuEvents) {
  fn request_t12_interrupt(&mut self, interrupt: InterruptLine) {
    self.0.request_t12_interrupt(interrupt);
  }
  fn request_t34_interrupt(&mut self, interrupt: InterruptLine) {
    self.0.request_t34_interrupt(interrupt);
  }
}

impl<'a, T> CoreContext for (T, &'a mut EmuEvents) {
  fn callbacks(&mut self) -> Option<&mut dyn Callbacks> {
    Some(self.1)
  }
}

impl<'a> PeripheralsContext for (&'a mut Interrupts, &'a mut EmuEvents) {
  fn interrupts(&self) -> &Interrupts {
    &self.0
  }
  fn interrupts_mut(&mut self) -> &mut Interrupts {
    &mut self.0
  }
}

impl Peripherals {
  fn emulate_oam_dma(&mut self) {
    if let Some(addr) = self.oam_dma.emulate() {
      let value = match addr >> 8 {
        0x00..=0x3f => self.cartridge.read_0000_3fff(addr),
        0x40..=0x7f => self.cartridge.read_4000_7fff(addr),
        0x80..=0x97 => self.gpu.read_character_ram(addr - 0x8000),
        0x98..=0x9b => self.gpu.read_tile_map1(addr - 0x9800),
        0x9c..=0x9f => self.gpu.read_tile_map2(addr - 0x9c00),
        0xa0..=0xbf => self.cartridge.read_a000_bfff(addr, 0xff),
        0xc0..=0xcf => self.work_ram.read_lower(addr),
        0xd0..=0xdf => self.work_ram.read_upper(addr),
        0xe0..=0xef => self.work_ram.read_lower(addr),
        0xf0..=0xff => self.work_ram.read_upper(addr),
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
  fn write_high<C: PeripheralsContext>(&mut self, ctx: &mut C, addr: u16, value: u8) {
    match addr as u8 {
      0x00 => self.generic_mem_cycle(ctx, |hw| hw.joypad.set_register(value)),
      0x01 => self.generic_mem_cycle(ctx, |hw| hw.serial.set_data(value)),
      0x02 => self.generic_mem_cycle(ctx, |hw| hw.serial.set_control(value)),
      0x04 => self.timer_mem_cycle(ctx, |timer, ctx| timer.div_write_cycle(ctx)),
      0x05 => self.timer_mem_cycle(ctx, |timer, ctx| timer.tima_write_cycle(value, ctx)),
      0x06 => self.timer_mem_cycle(ctx, |timer, ctx| timer.tma_write_cycle(value, ctx)),
      0x07 => self.timer_mem_cycle(ctx, |timer, ctx| timer.tac_write_cycle(value, ctx)),
      0x0f => {
        self.generic_cycle(ctx);
        ctx.interrupts_mut().set_interrupt_flag(value);
      }
      0x10..=0x3f => self.generic_mem_cycle(ctx, |hw| hw.apu.write(addr, value)),
      0x40 => self.generic_mem_cycle(ctx, |hw| hw.gpu.set_control(value)),
      0x41 => self.generic_mem_cycle(ctx, |hw| hw.gpu.set_stat(value)),
      0x42 => self.generic_mem_cycle(ctx, |hw| hw.gpu.set_scroll_y(value)),
      0x43 => self.generic_mem_cycle(ctx, |hw| hw.gpu.set_scroll_x(value)),
      0x44 => self.generic_mem_cycle(ctx, |hw| hw.gpu.reset_current_line()),
      0x45 => self.generic_mem_cycle(ctx, |hw| hw.gpu.set_compare_line(value)),
      0x46 => self.generic_mem_cycle(ctx, |hw| hw.oam_dma.request(value)),
      0x47 => self.generic_mem_cycle(ctx, |hw| hw.gpu.set_bg_palette(value)),
      0x48 => self.generic_mem_cycle(ctx, |hw| hw.gpu.set_obj_palette0(value)),
      0x49 => self.generic_mem_cycle(ctx, |hw| hw.gpu.set_obj_palette1(value)),
      0x4a => self.generic_mem_cycle(ctx, |hw| hw.gpu.set_window_y(value)),
      0x4b => self.generic_mem_cycle(ctx, |hw| hw.gpu.set_window_x(value)),
      0x50 => {
        self.generic_cycle(ctx);
        if self.bootrom.is_active() && value & 0b1 != 0 {
          self.bootrom.deactivate();
          if let Some(callbacks) = ctx.callbacks() {
            callbacks.bootrom_disabled();
          }
        }
      }
      0x80..=0xfe => self.generic_mem_cycle(ctx, |hw| hw.hiram[(addr as usize) & 0x7f] = value),
      0xff => {
        self.generic_cycle(ctx);
        ctx.interrupts_mut().set_interrupt_enable(value);
      }
      _ => self.generic_cycle(ctx),
    }
  }
  fn read_high<C: PeripheralsContext>(&mut self, ctx: &mut C, addr: u16) -> u8 {
    match addr as u8 {
      0x00 => self.generic_mem_cycle(ctx, |hw| hw.joypad.get_register()),
      0x01 => self.generic_mem_cycle(ctx, |hw| hw.serial.get_data()),
      0x02 => self.generic_mem_cycle(ctx, |hw| hw.serial.get_control()),
      0x04 => self.timer_mem_cycle(ctx, |timer, ctx| timer.div_read_cycle(ctx)),
      0x05 => self.timer_mem_cycle(ctx, |timer, ctx| timer.tima_read_cycle(ctx)),
      0x06 => self.timer_mem_cycle(ctx, |timer, ctx| timer.tma_read_cycle(ctx)),
      0x07 => self.timer_mem_cycle(ctx, |timer, ctx| timer.tac_read_cycle(ctx)),
      0x0f => {
        self.generic_cycle(ctx);
        ctx.interrupts().get_interrupt_flag()
      }
      0x10..=0x3f => self.generic_mem_cycle(ctx, |hw| hw.apu.read(addr)),
      0x40 => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_control()),
      0x41 => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_stat()),
      0x42 => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_scroll_y()),
      0x43 => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_scroll_x()),
      0x44 => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_current_line()),
      0x45 => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_compare_line()),
      0x46 => self.generic_mem_cycle(ctx, |hw| hw.oam_dma.source),
      0x47 => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_bg_palette()),
      0x48 => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_obj_palette0()),
      0x49 => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_obj_palette1()),
      0x4a => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_window_y()),
      0x4b => self.generic_mem_cycle(ctx, |hw| hw.gpu.get_window_x()),
      0x80..=0xfe => self.generic_mem_cycle(ctx, |hw| hw.hiram[(addr as usize) & 0x7f]),
      0xff => {
        self.generic_cycle(ctx);
        ctx.interrupts().get_interrupt_enable()
      }
      _ => self.generic_mem_cycle(ctx, |_| 0xff),
    }
  }
  fn write<C: PeripheralsContext>(&mut self, ctx: &mut C, addr: u16, value: u8) {
    match (addr >> 8) as u8 {
      0x00 if self.bootrom.is_active() => self.generic_cycle(ctx),
      0x00..=0x7f => self.generic_mem_cycle(ctx, |hw| hw.cartridge.write_control(addr, value)),
      0x80..=0x97 => {
        self.generic_mem_cycle(ctx, |hw| hw.gpu.write_character_ram(addr - 0x8000, value))
      }
      0x98..=0x9b => self.generic_mem_cycle(ctx, |hw| hw.gpu.write_tile_map1(addr - 0x9800, value)),
      0x9c..=0x9f => self.generic_mem_cycle(ctx, |hw| hw.gpu.write_tile_map2(addr - 0x9c00, value)),
      0xa0..=0xbf => self.generic_mem_cycle(ctx, |hw| hw.cartridge.write_a000_bfff(addr, value)),
      0xc0..=0xcf => self.generic_mem_cycle(ctx, |hw| hw.work_ram.write_lower(addr, value)),
      0xd0..=0xdf => self.generic_mem_cycle(ctx, |hw| hw.work_ram.write_upper(addr, value)),
      // Echo RAM
      0xe0..=0xef => self.generic_mem_cycle(ctx, |hw| hw.work_ram.write_lower(addr, value)),
      0xf0..=0xfd => self.generic_mem_cycle(ctx, |hw| hw.work_ram.write_upper(addr, value)),
      0xfe => match addr & 0xff {
        0x00..=0x9f => self.generic_mem_cycle(ctx, |hw| {
          if !hw.oam_dma.is_active() {
            hw.gpu.write_oam(addr as u8, value)
          }
        }),
        _ => self.generic_cycle(ctx),
      },
      0xff => self.write_high(ctx, addr, value),
    }
  }
  fn read<C: PeripheralsContext>(&mut self, ctx: &mut C, addr: u16) -> u8 {
    match (addr >> 8) as u8 {
      0x00 if self.bootrom.is_active() => self.generic_mem_cycle(ctx, |hw| hw.bootrom[addr]),
      0x00..=0x3f => self.generic_mem_cycle(ctx, |hw| hw.cartridge.read_0000_3fff(addr)),
      0x40..=0x7f => self.generic_mem_cycle(ctx, |hw| hw.cartridge.read_4000_7fff(addr)),
      0x80..=0x97 => self.generic_mem_cycle(ctx, |hw| hw.gpu.read_character_ram(addr - 0x8000)),
      0x98..=0x9b => self.generic_mem_cycle(ctx, |hw| hw.gpu.read_tile_map1(addr - 0x9800)),
      0x9c..=0x9f => self.generic_mem_cycle(ctx, |hw| hw.gpu.read_tile_map2(addr - 0x9c00)),
      0xa0..=0xbf => self.generic_mem_cycle(ctx, |hw| hw.cartridge.read_a000_bfff(addr, 0xff)),
      0xc0..=0xcf => self.generic_mem_cycle(ctx, |hw| hw.work_ram.read_lower(addr)),
      0xd0..=0xdf => self.generic_mem_cycle(ctx, |hw| hw.work_ram.read_upper(addr)),
      // Echo RAM
      0xe0..=0xef => self.generic_mem_cycle(ctx, |hw| hw.work_ram.read_lower(addr)),
      0xf0..=0xfd => self.generic_mem_cycle(ctx, |hw| hw.work_ram.read_upper(addr)),
      0xfe => {
        match addr & 0xff {
          0x00..=0x9f => self.generic_mem_cycle(ctx, |hw| {
            if hw.oam_dma.is_active() {
              0xff
            } else {
              hw.gpu.read_oam(addr as u8)
            }
          }),
          // 0x00 ..= 0x9f => handle_oam!(),
          // 0xa0 ..= 0xff => handle_unusable!(),
          _ => panic!("Unsupported read at ${:04x}", addr),
        }
      }
      0xff => self.read_high(ctx, addr),
    }
  }
  fn emulate<C: PeripheralsContext>(&mut self, ctx: &mut C) {
    self.emulate_oam_dma();
    self.gpu.emulate(ctx);
    self.apu.emulate();
  }
  fn generic_cycle<C: PeripheralsContext>(&mut self, ctx: &mut C) {
    self.emulate(ctx);
    self.timer.tick_cycle(ctx);
  }
  fn generic_mem_cycle<T, C: PeripheralsContext, F: FnOnce(&mut Self) -> T>(
    &mut self,
    ctx: &mut C,
    f: F,
  ) -> T {
    self.generic_cycle(ctx);
    f(self)
  }
  fn timer_mem_cycle<T, C: PeripheralsContext, F: FnOnce(&mut Timer, &mut C) -> T>(
    &mut self,
    ctx: &mut C,
    f: F,
  ) -> T {
    self.emulate(ctx);
    f(&mut self.timer, ctx)
  }
}

impl CoreContext for Hardware {
  fn callbacks(&mut self) -> Option<&mut dyn Callbacks> {
    Some(&mut self.emu_events)
  }
}

impl Callbacks for EmuEvents {
  fn debug_opcode(&mut self) {
    self.insert(EmuEvents::DEBUG_OP);
  }
  fn bootrom_disabled(&mut self) {
    self.insert(EmuEvents::BOOTROM_DISABLED);
  }
  fn trigger_emu_events(&mut self, emu_events: EmuEvents) {
    self.insert(emu_events);
  }
}

struct InterruptCheck<'a> {
  check: Interrupts,
  interrupts: &'a mut Interrupts,
  emu_events: &'a mut EmuEvents,
}

impl<'a> InterruptRequest for InterruptCheck<'a> {
  fn request_t12_interrupt(&mut self, interrupt: InterruptLine) {
    self.check.request_t12_interrupt(interrupt);
    self.interrupts.request_t12_interrupt(interrupt);
  }
  fn request_t34_interrupt(&mut self, interrupt: InterruptLine) {
    self.interrupts.request_t34_interrupt(interrupt);
  }
}

impl<'a> CoreContext for InterruptCheck<'a> {
  fn callbacks(&mut self) -> Option<&mut dyn Callbacks> {
    Some(self.emu_events)
  }
}

impl<'a> PeripheralsContext for InterruptCheck<'a> {
  fn interrupts(&self) -> &Interrupts {
    &self.interrupts
  }
  fn interrupts_mut(&mut self) -> &mut Interrupts {
    &mut self.interrupts
  }
}

impl CpuContext for Hardware {
  fn read_cycle(&mut self, addr: u16) -> u8 {
    self.emu_time += EmuTime::from_machine_cycles(1);
    let mut ctx = (&mut self.interrupts, &mut self.emu_events);
    self.peripherals.read(&mut ctx, addr)
  }
  fn read_cycle_high(&mut self, addr: u8) -> u8 {
    self.emu_time += EmuTime::from_machine_cycles(1);
    let mut ctx = (&mut self.interrupts, &mut self.emu_events);
    self.peripherals.read_high(&mut ctx, 0xff00 | (addr as u16))
  }
  fn read_cycle_intr(&mut self, addr: u16) -> (InterruptLine, u8) {
    self.emu_time += EmuTime::from_machine_cycles(1);
    let mut ctx = InterruptCheck {
      check: self.interrupts.clone(),
      interrupts: &mut self.interrupts,
      emu_events: &mut self.emu_events,
    };
    let data = self.peripherals.read(&mut ctx, addr);
    (ctx.check.get_interrupt(), data)
  }
  fn write_cycle(&mut self, addr: u16, data: u8) {
    self.emu_time += EmuTime::from_machine_cycles(1);
    let mut ctx = (&mut self.interrupts, &mut self.emu_events);
    self.peripherals.write(&mut ctx, addr, data)
  }
  fn write_cycle_high(&mut self, addr: u8, data: u8) {
    self.emu_time += EmuTime::from_machine_cycles(1);
    let mut ctx = (&mut self.interrupts, &mut self.emu_events);
    self
      .peripherals
      .write_high(&mut ctx, 0xff00 | (addr as u16), data);
  }
  fn write_cycle_intr(&mut self, addr: u16, data: u8) -> InterruptLine {
    self.emu_time += EmuTime::from_machine_cycles(1);
    let mut ctx = InterruptCheck {
      check: self.interrupts.clone(),
      interrupts: &mut self.interrupts,
      emu_events: &mut self.emu_events,
    };
    self.peripherals.write(&mut ctx, addr, data);
    ctx.check.get_interrupt()
  }
  fn tick_cycle(&mut self) {
    self.emu_time += EmuTime::from_machine_cycles(1);
    let mut ctx = (&mut self.interrupts, &mut self.emu_events);
    self.peripherals.generic_cycle(&mut ctx);
  }
  fn has_interrupt(&self) -> bool {
    !self.interrupts.get_interrupt().is_empty()
  }
  fn ack_interrupt(&mut self, mask: InterruptLine) {
    self.interrupts.ack_interrupt(mask);
  }
}
