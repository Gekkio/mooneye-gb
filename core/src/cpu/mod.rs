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
use bitflags::bitflags;
use std::fmt;

use self::register_file::{Flags, Reg16, Reg8, RegisterFile};
use crate::hardware::Bus;
use crate::util::int::IntExt;

mod decode;
mod execute;
pub mod register_file;

#[cfg(all(test, not(feature = "acceptance_tests")))]
mod test;

bitflags!(
  pub struct InterruptLine: u8 {
    const VBLANK = 1 << 0;
    const STAT = 1 << 1;
    const TIMER = 1 << 2;
    const SERIAL = 1 << 3;
    const JOYPAD = 1 << 4;
  }
);

#[derive(Clone)]
pub struct Cpu {
  pub regs: RegisterFile,
  ime: bool,
}

pub trait In8 {
  fn read<H: Bus>(&self, cpu: &mut Cpu, bus: &mut H) -> u8;
}
pub trait Out8 {
  fn write<H: Bus>(&self, cpu: &mut Cpu, bus: &mut H, data: u8);
}

#[derive(Clone, Copy, Debug)]
pub enum Cond {
  NZ,
  Z,
  NC,
  C,
}

impl Cond {
  fn check(&self, flags: Flags) -> bool {
    use self::Cond::*;
    match *self {
      NZ => !flags.contains(Flags::ZERO),
      Z => flags.contains(Flags::ZERO),
      NC => !flags.contains(Flags::CARRY),
      C => flags.contains(Flags::CARRY),
    }
  }
}

pub struct Immediate8;
impl In8 for Immediate8 {
  fn read<H: Bus>(&self, cpu: &mut Cpu, bus: &mut H) -> u8 {
    cpu.next_u8(bus)
  }
}

#[derive(Clone, Copy, Debug)]
pub enum Addr {
  BC,
  DE,
  HL,
  HLD,
  HLI,
  Direct,
  ZeroPage,
  ZeroPageC,
}
impl In8 for Addr {
  fn read<H: Bus>(&self, cpu: &mut Cpu, bus: &mut H) -> u8 {
    let addr = cpu.indirect_addr(bus, *self);
    bus.read_cycle(addr)
  }
}
impl Out8 for Addr {
  fn write<H: Bus>(&self, cpu: &mut Cpu, bus: &mut H, value: u8) {
    let addr = cpu.indirect_addr(bus, *self);
    bus.write_cycle(addr, value)
  }
}

impl In8 for Reg8 {
  fn read<H: Bus>(&self, cpu: &mut Cpu, _: &mut H) -> u8 {
    use self::register_file::Reg8::*;
    match *self {
      A => cpu.regs.a,
      B => cpu.regs.b,
      C => cpu.regs.c,
      D => cpu.regs.d,
      E => cpu.regs.e,
      H => cpu.regs.h,
      L => cpu.regs.l,
    }
  }
}
impl Out8 for Reg8 {
  fn write<H: Bus>(&self, cpu: &mut Cpu, _: &mut H, value: u8) {
    use self::register_file::Reg8::*;
    match *self {
      A => cpu.regs.a = value,
      B => cpu.regs.b = value,
      C => cpu.regs.c = value,
      D => cpu.regs.d = value,
      E => cpu.regs.e = value,
      H => cpu.regs.h = value,
      L => cpu.regs.l = value,
    }
  }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Step {
  Initial,
  Opcode(u8),
  Halt,
  InterruptDispatch,
}

impl fmt::Display for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.regs)
  }
}
impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      regs: RegisterFile::new(),
      ime: false,
    }
  }

  fn prefetch_next<H: Bus>(&mut self, bus: &mut H, addr: u16) -> Step {
    let opcode = bus.read_cycle(addr);
    if self.ime && !bus.get_mid_interrupt().is_empty() {
      Step::InterruptDispatch
    } else {
      self.regs.pc = addr.wrapping_add(1);
      Step::Opcode(opcode)
    }
  }

  fn next_u8<H: Bus>(&mut self, bus: &mut H) -> u8 {
    let addr = self.regs.pc;
    self.regs.pc = self.regs.pc.wrapping_add(1);
    bus.read_cycle(addr)
  }
  fn next_u16<H: Bus>(&mut self, bus: &mut H) -> u16 {
    let l = self.next_u8(bus);
    let h = self.next_u8(bus);
    ((h as u16) << 8) | (l as u16)
  }

  fn pop_u8<H: Bus>(&mut self, bus: &mut H) -> u8 {
    let sp = self.regs.sp;
    let value = bus.read_cycle(sp);
    self.regs.sp = self.regs.sp.wrapping_add(1);
    value
  }
  fn push_u8<H: Bus>(&mut self, bus: &mut H, value: u8) {
    self.regs.sp = self.regs.sp.wrapping_sub(1);
    let sp = self.regs.sp;
    bus.write_cycle(sp, value);
  }

  fn pop_u16<H: Bus>(&mut self, bus: &mut H) -> u16 {
    let l = self.pop_u8(bus);
    let h = self.pop_u8(bus);
    ((h as u16) << 8) | (l as u16)
  }
  fn push_u16<H: Bus>(&mut self, bus: &mut H, value: u16) {
    self.push_u8(bus, (value >> 8) as u8);
    self.push_u8(bus, value as u8);
  }

  fn indirect_addr<H: Bus>(&mut self, bus: &mut H, addr: Addr) -> u16 {
    use self::Addr::*;
    match addr {
      BC => self.regs.read16(Reg16::BC),
      DE => self.regs.read16(Reg16::DE),
      HL => self.regs.read16(Reg16::HL),
      HLD => {
        let addr = self.regs.read16(Reg16::HL);
        self.regs.write16(Reg16::HL, addr.wrapping_sub(1));
        addr
      }
      HLI => {
        let addr = self.regs.read16(Reg16::HL);
        self.regs.write16(Reg16::HL, addr.wrapping_add(1));
        addr
      }
      Direct => self.next_u16(bus),
      ZeroPage => 0xff00u16 | self.next_u8(bus) as u16,
      ZeroPageC => 0xff00u16 | self.regs.c as u16,
    }
  }

  pub fn execute_step<H: Bus>(&mut self, bus: &mut H, step: Step) -> Step {
    match step {
      Step::Initial => self.prefetch_next(bus, self.regs.pc),
      Step::Opcode(opcode) => self.decode_exec_fetch(bus, opcode),
      Step::InterruptDispatch => {
        self.ime = false;
        bus.tick_cycle();
        bus.tick_cycle();
        let pc = self.regs.pc;
        self.push_u8(bus, (pc >> 8) as u8);
        self.push_u8(bus, pc as u8);
        let interrupt =
          InterruptLine::from_bits_truncate(bus.get_mid_interrupt().bits().isolate_rightmost_one());
        bus.ack_interrupt(interrupt);
        self.regs.pc = match interrupt {
          InterruptLine::VBLANK => 0x0040,
          InterruptLine::STAT => 0x0048,
          InterruptLine::TIMER => 0x0050,
          InterruptLine::SERIAL => 0x0058,
          InterruptLine::JOYPAD => 0x0060,
          _ => 0x0000,
        };
        let opcode = self.next_u8(bus);
        Step::Opcode(opcode)
      }
      Step::Halt => {
        if !bus.get_end_interrupt().is_empty() {
          self.prefetch_next(bus, self.regs.pc)
        } else {
          bus.tick_cycle();
          Step::Halt
        }
      }
    }
  }

  fn alu_sub(&mut self, value: u8, use_carry: bool) -> u8 {
    let cy = if use_carry && self.regs.f.contains(Flags::CARRY) {
      1
    } else {
      0
    };
    let result = self.regs.a.wrapping_sub(value).wrapping_sub(cy);
    self.regs.f = Flags::ZERO.test(result == 0)
      | Flags::ADD_SUBTRACT
      | Flags::CARRY.test((self.regs.a as u16) < (value as u16) + (cy as u16))
      | Flags::HALF_CARRY.test(
        (self.regs.a & 0xf)
          .wrapping_sub(value & 0xf)
          .wrapping_sub(cy)
          & (0xf + 1)
          != 0,
      );
    result
  }
  fn alu_rl(&mut self, value: u8, set_zero: bool) -> u8 {
    let ci = if self.regs.f.contains(Flags::CARRY) {
      1
    } else {
      0
    };
    let co = value & 0x80;
    let new_value = (value << 1) | ci;
    self.regs.f = Flags::ZERO.test(set_zero && new_value == 0) | Flags::CARRY.test(co != 0);
    new_value
  }
  fn alu_rlc(&mut self, value: u8, set_zero: bool) -> u8 {
    let co = value & 0x80;
    let new_value = value.rotate_left(1);
    self.regs.f = Flags::ZERO.test(set_zero && new_value == 0) | Flags::CARRY.test(co != 0);
    new_value
  }
  fn alu_rr(&mut self, value: u8, set_zero: bool) -> u8 {
    let ci = if self.regs.f.contains(Flags::CARRY) {
      1
    } else {
      0
    };
    let co = value & 0x01;
    let new_value = (value >> 1) | (ci << 7);
    self.regs.f = Flags::ZERO.test(set_zero && new_value == 0) | Flags::CARRY.test(co != 0);
    new_value
  }
  fn alu_rrc(&mut self, value: u8, set_zero: bool) -> u8 {
    let co = value & 0x01;
    let new_value = value.rotate_right(1);
    self.regs.f = Flags::ZERO.test(set_zero && new_value == 0) | Flags::CARRY.test(co != 0);
    new_value
  }
  fn ctrl_jp<H: Bus>(&mut self, bus: &mut H, addr: u16) {
    self.regs.pc = addr;
    bus.tick_cycle();
  }
  fn ctrl_jr<H: Bus>(&mut self, bus: &mut H, offset: i8) {
    self.regs.pc = self.regs.pc.wrapping_add(offset as u16);
    bus.tick_cycle();
  }
  fn ctrl_call<H: Bus>(&mut self, bus: &mut H, addr: u16) {
    let pc = self.regs.pc;
    bus.tick_cycle();
    self.push_u16(bus, pc);
    self.regs.pc = addr;
  }
  fn ctrl_ret<H: Bus>(&mut self, bus: &mut H) {
    self.regs.pc = self.pop_u16(bus);
    bus.tick_cycle();
  }
}
