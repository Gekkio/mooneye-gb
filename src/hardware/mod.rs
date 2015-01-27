#![allow(dead_code)]

use std::fmt;
use std::iter::range_step;

use backend::{
  BackendSharedMemory,
  GbKey
};
use config::HardwareConfig;
use emulation::{EmuTime, MachineCycles};
use hardware::apu::Apu;
use hardware::bootrom::Bootrom;
use hardware::cartridge::Cartridge;
use hardware::gpu::Gpu;
use hardware::internal_ram::InternalRam;
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
mod internal_ram;
pub mod irq;
mod joypad;
mod serial;
mod timer;

pub trait Bus {
  fn write(&mut self, EmuTime, u16, u8);
  fn read(&self, EmuTime, u16) -> u8;
  fn emulate(&mut self, EmuTime);
  fn ack_interrupt(&mut self) -> Option<Interrupt>;
  fn has_interrupt(&self) -> bool;
  fn rewind_time(&mut self);
}

pub struct Hardware<'a> {
  pub bootrom: Bootrom,
  pub cartridge: Cartridge,
  internal_ram: InternalRam,
  gpu: Gpu<'a>,
  apu: Apu,
  joypad: Joypad,
  serial: Serial,
  pub timer: Timer,
  oam_dma: OamDma,
  irq: Irq
}

struct OamDma {
  active: bool,
  end_time: EmuTime
}

impl OamDma {
  fn new() -> OamDma {
    OamDma {
      active: false,
      end_time: EmuTime::zero()
    }
  }
}

impl<'a> Hardware<'a> {
  pub fn new(backend: &'a BackendSharedMemory, config: HardwareConfig) -> Hardware<'a> {
    Hardware {
      bootrom: Bootrom::new(config.bootrom),
      cartridge: Cartridge::new(config.cartridge),
      internal_ram: InternalRam::new(),
      gpu: Gpu::new(backend),
      apu: Apu::new(),
      joypad: Joypad::new(),
      serial: Serial::new(),
      timer: Timer::new(),
      oam_dma: OamDma::new(),
      irq: Irq::new()
    }
  }
  pub fn key_down(&mut self, key: GbKey) {
    self.joypad.key_down(key, &mut self.irq);
  }
  pub fn key_up(&mut self, key: GbKey) {
    self.joypad.key_up(key);
  }
  fn start_oam_dma(&mut self, time: EmuTime, value: u8) {
    if value < 0x80 || value > 0xdf {
      println!("Invalid DMA {:02x}", value);
      return;
    }
    self.oam_dma.active = true;
    self.oam_dma.end_time = time + MachineCycles(162);
    let addr = (value as u16) << 8;
    for i in range(0, 0xa0) {
      let value = self.read_internal(time, addr + i);
      self.gpu.write_oam(i, value);
    }
  }
  pub fn dump_mem(&self, time: EmuTime, addr: u16, chunks: usize) {
    for i in range_step(addr, addr + (chunks as u16 * 8), 8) {
      println!("${:04x}: {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x} {:02x}", i,
               self.read(time, i + 0), self.read(time, i + 1), self.read(time, i + 2), self.read(time, i + 3),
               self.read(time, i + 4), self.read(time, i + 5), self.read(time, i + 6), self.read(time, i + 7));
    }
  }
  fn write_internal(&mut self, time: EmuTime, addr: u16, value: u8) {
    match addr >> 8 {
      0x00 ... 0x7f => self.cartridge.write_control(addr, value),
      0x80 ... 0x97 => self.gpu.write_character_ram(addr - 0x8000, value),
      0x98 ... 0x9b => self.gpu.write_tile_map1(addr - 0x9800, value),
      0x9c ... 0x9f => self.gpu.write_tile_map2(addr - 0x9c00, value),
      0xa0 ... 0xbf => self.cartridge.write_ram(addr - 0xa000, value),
      0xc0 ... 0xcf => self.internal_ram.write_bank0(addr - 0xc000, value),
      0xd0 ... 0xdf => self.internal_ram.write_bank1(addr - 0xd000, value),
      // Echo RAM
      0xe0 ... 0xef => self.internal_ram.write_bank0(addr - 0xe000, value),
      0xf0 ... 0xfd => self.internal_ram.write_bank1(addr - 0xf000, value),
      0xfe => {
        match addr & 0xff {
          0x00 ... 0x9f =>
            if !self.oam_dma.active {
              self.gpu.write_oam(addr & 0xff, value)
            },
          _ => ()
        }
      },
      0xff => {
        match addr & 0xff {
          0x00 => self.joypad.set_register(value),
          0x01 => self.serial.set_data(value),
          0x02 => self.serial.set_control(value),
          0x04 => self.timer.reset_divider(time),
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
          0x46 => self.start_oam_dma(time, value),
          0x47 => self.gpu.set_bg_palette(value),
          0x48 => self.gpu.set_obj_palette0(value),
          0x49 => self.gpu.set_obj_palette1(value),
          0x4a => self.gpu.set_window_y(value),
          0x4b => self.gpu.set_window_x(value),
          0x50 => self.bootrom.deactivate(),
          0xff => self.irq.set_interrupt_enable(value),
          _ => ()
        }
      },
      _ => panic!("Unsupported write at ${:04x} = {:02x}", addr, value)
    }
  }
  fn read_internal(&self, time: EmuTime, addr: u16) -> u8 {
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
      0xc0 ... 0xcf => self.internal_ram.read_bank0(addr - 0xc000),
      0xd0 ... 0xdf => self.internal_ram.read_bank1(addr - 0xd000),
      // Echo RAM
      0xe0 ... 0xef => self.internal_ram.read_bank0(addr - 0xe000),
      0xf0 ... 0xfd => self.internal_ram.read_bank1(addr - 0xf000),
      0xfe => {
        match addr & 0xff {
          0x00 ... 0x9f =>
            if self.oam_dma.active { 0xff } else {
              self.gpu.read_oam(addr & 0xff)
            },
          // 0x00 ... 0x9f => handle_oam!(),
          // 0xa0 ... 0xff => handle_unusable!(),
          _ => panic!("Unsupported read at ${:04x}", addr)
        }
      },
      0xff => {
        match addr & 0xff {
          0x00 => self.joypad.get_register(),
          0x01 => panic!("Unsupported read at ${:04x}", addr),
          0x02 => panic!("Unsupported read at ${:04x}", addr),
          0x04 => self.timer.get_divider(time),
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
          0x46 => panic!("Unsupported read at ${:04x}", addr),
          0x47 => self.gpu.get_bg_palette(),
          0x48 => self.gpu.get_obj_palette0(),
          0x49 => self.gpu.get_obj_palette1(),
          0x4a => self.gpu.get_window_y(),
          0x4b => self.gpu.get_window_x(),
          0xff => self.irq.get_interrupt_enable(),
          _ => 0xff
        }
      },
      _ => panic!("Unsupported read at ${:04x}", addr)
    }
  }
}

impl<'a> Bus for Hardware<'a> {
  fn read(&self, time: EmuTime, addr: u16) -> u8 {
    if self.oam_dma.active {
      println!("Warning: read at ${:04x} during OAM DMA!", addr);
    }
    self.read_internal(time, addr)
  }
  fn write(&mut self, time: EmuTime, addr: u16, value: u8) {
    if self.oam_dma.active {
      println!("Warning: write at ${:04x} = {:02x} during OAM DMA!", addr, value)
    }
    self.write_internal(time, addr, value)
  }
  fn emulate(&mut self, time: EmuTime) {
    if self.oam_dma.active && self.oam_dma.end_time <= time {
      self.oam_dma.active = false;
    }
    self.timer.emulate(&mut self.irq);
    self.gpu.emulate(&mut self.irq);
    self.apu.emulate();
  }
  fn ack_interrupt(&mut self) -> Option<Interrupt> { self.irq.ack_interrupt() }
  fn has_interrupt(&self) -> bool { self.irq.has_interrupt() }
  fn rewind_time(&mut self) {
    self.timer.rewind_time();
  }
}

impl<'a> fmt::Debug for Hardware<'a> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.gpu)
  }
}
