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

use emulation::EmuEvents;
use hardware::{Bus};
use cpu::registers::{
  Registers, Reg8, Reg16, Flags,
};
use util::int::IntExt;

pub use cpu::ops::CpuOps;

pub mod disasm;
mod ops;
pub mod registers;

#[cfg(all(test, not(feature = "acceptance_tests")))]
mod test;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u8)]
pub enum Ime {
  Disabled,
  Enabling,
  Enabled
}

pub struct Cpu {
  pub regs: Registers,
  ime: Ime,
  halt: bool,
}

pub trait In8: disasm::ResolveOp8 {
  fn read<H: Bus>(&self, &mut Cpu, &mut H) -> u8;
}
pub trait Out8: disasm::ResolveOp8 {
  fn write<H: Bus>(&self, &mut Cpu, &mut H, u8);
}


#[derive(Clone, Copy, Debug)]
pub enum Cond {
  NZ, Z,
  NC, C
}

impl Cond {
  fn check(&self, flags: Flags) -> bool {
    use self::Cond::*;
    match *self {
      NZ => !flags.contains(Flags::ZERO),  Z => flags.contains(Flags::ZERO),
      NC => !flags.contains(Flags::CARRY), C => flags.contains(Flags::CARRY),
    }
  }
}

pub struct Immediate8;
impl In8 for Immediate8 {
  fn read<H: Bus>(&self, cpu: &mut Cpu, bus: &mut H) -> u8 { cpu.next_u8(bus) }
}

#[derive(Clone, Copy, Debug)]
pub enum Addr {
  BC, DE, HL, HLD, HLI,
  Direct, ZeroPage, ZeroPageC
}
impl In8 for Addr {
  fn read<H: Bus>(&self, cpu: &mut Cpu, bus: &mut H) -> u8 {
    let addr = cpu.indirect_addr(bus, *self);
    cpu.read_cycle(bus, addr)
  }
}
impl Out8 for Addr {
  fn write<H: Bus>(&self, cpu: &mut Cpu, bus: &mut H, value: u8) {
    let addr = cpu.indirect_addr(bus, *self);
    cpu.write_cycle(bus, addr, value);
  }
}


impl In8 for Reg8 {
  fn read<H: Bus>(&self, cpu: &mut Cpu, _: &mut H) -> u8 {
    use cpu::registers::Reg8::*;
    match *self {
      A => cpu.regs.a, B => cpu.regs.b,
      C => cpu.regs.c, D => cpu.regs.d,
      E => cpu.regs.e, H => cpu.regs.h,
      L => cpu.regs.l
    }
  }
}
impl Out8 for Reg8 {
  fn write<H: Bus>(&self, cpu: &mut Cpu, _: &mut H, value: u8) {
    use cpu::registers::Reg8::*;
    match *self {
      A => cpu.regs.a = value, B => cpu.regs.b = value,
      C => cpu.regs.c = value, D => cpu.regs.d = value,
      E => cpu.regs.e = value, H => cpu.regs.h = value,
      L => cpu.regs.l = value
    }
  }
}


impl fmt::Display for Cpu {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.regs)
  }
}
impl Cpu {
  pub fn new() -> Cpu {
    Cpu {
      regs: Registers::new(),
      ime: Ime::Enabled,
      halt: false,
    }
  }

  fn fetch_cycle<H: Bus>(&mut self, bus: &mut H) {
    let result = bus.fetch_cycle(self.regs.pc);
    let interrupt = match self.ime {
      Ime::Enabled => result.interrupt,
      Ime::Enabling => {
        self.ime = Ime::Enabled;
        false
      },
      _ => false,
    };
    if interrupt {
      self.dispatch_interrupt(bus)
    } else {
      self.regs.pc = self.regs.pc.wrapping_add(1);
      ops::decode((self, bus), result.opcode)
    }
  }
  fn read_cycle<H: Bus>(&self, bus: &mut H, addr: u16) -> u8 {
    bus.read_cycle(addr)
  }
  fn write_cycle<H: Bus>(&self, bus: &mut H, addr: u16, value: u8) {
    bus.write_cycle(addr, value)
  }
  fn halt_cycle<H: Bus>(&mut self, bus: &mut H) {
    if bus.has_interrupt() {
      self.halt = false;
    } else {
      bus.emulate();
    }
  }
  fn internal_cycle<H: Bus>(&self, bus: &mut H) {
    bus.emulate();
  }

  fn next_u8<H: Bus>(&mut self, bus: &mut H) -> u8 {
    let addr = self.regs.pc;
    self.regs.pc = self.regs.pc.wrapping_add(1);
    self.read_cycle(bus, addr)
  }
  fn next_u16<H: Bus>(&mut self, bus: &mut H) -> u16 {
    let l = self.next_u8(bus);
    let h = self.next_u8(bus);
    ((h as u16) << 8) | (l as u16)
  }

  fn pop_u8<H: Bus>(&mut self, bus: &mut H) -> u8 {
    let sp = self.regs.sp;
    let value = self.read_cycle(bus, sp);
    self.regs.sp = self.regs.sp.wrapping_add_one();
    value
  }
  fn push_u8<H: Bus>(&mut self, bus: &mut H, value: u8) {
    self.regs.sp = self.regs.sp.wrapping_sub_one();
    let sp = self.regs.sp;
    self.write_cycle(bus, sp, value);
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
        self.regs.write16(Reg16::HL, addr.wrapping_sub_one());
        addr
      },
      HLI => {
        let addr = self.regs.read16(Reg16::HL);
        self.regs.write16(Reg16::HL, addr.wrapping_add_one());
        addr
      },
      Direct => self.next_u16(bus),
      ZeroPage => 0xff00u16 | self.next_u8(bus) as u16,
      ZeroPageC => 0xff00u16 | self.regs.c as u16,
    }
  }

  pub fn execute<H: Bus>(&mut self, bus: &mut H) {
    if self.halt {
      self.halt_cycle(bus);
    } else {
      self.fetch_cycle(bus);
    }
  }

  fn dispatch_interrupt<H: Bus>(&mut self, bus: &mut H) {
    self.halt = false;
    self.ime = Ime::Disabled;
    self.internal_cycle(bus);
    self.internal_cycle(bus);
    let pc = self.regs.pc;
    self.push_u8(bus, (pc >> 8) as u8);
    let interrupt = bus.ack_interrupt();
    self.push_u8(bus, pc as u8);
    self.regs.pc = interrupt.map(|i| i.get_addr()).unwrap_or(0x0000);
  }

  fn alu_sub(&mut self, value: u8, use_carry: bool) -> u8 {
    let cy = if use_carry && self.regs.f.contains(Flags::CARRY) { 1 } else { 0 };
    let result = self.regs.a.wrapping_sub(value).wrapping_sub(cy);
    self.regs.f = Flags::ZERO.test(result == 0) |
                  Flags::ADD_SUBTRACT |
                  Flags::CARRY.test((self.regs.a as u16) < (value as u16) + (cy as u16)) |
                  Flags::HALF_CARRY.test((self.regs.a & 0xf) < (value & 0xf) + cy);
    result
  }
  fn alu_rl(&mut self, value: u8, set_zero: bool) -> u8 {
    let ci = if self.regs.f.contains(Flags::CARRY) { 1 } else { 0 };
    let co = value & 0x80;
    let new_value = (value << 1) | ci;
    self.regs.f = Flags::ZERO.test(set_zero && new_value == 0) |
                  Flags::CARRY.test(co != 0);
    new_value
  }
  fn alu_rlc(&mut self, value: u8, set_zero: bool) -> u8 {
    let co = value & 0x80;
    let new_value = value.rotate_left(1);
    self.regs.f = Flags::ZERO.test(set_zero && new_value == 0) |
                  Flags::CARRY.test(co != 0);
    new_value
  }
  fn alu_rr(&mut self, value: u8, set_zero: bool) -> u8 {
    let ci = if self.regs.f.contains(Flags::CARRY) { 1 } else { 0 };
    let co = value & 0x01;
    let new_value = (value >> 1) | (ci << 7);
    self.regs.f = Flags::ZERO.test(set_zero && new_value == 0) |
                  Flags::CARRY.test(co != 0);
    new_value
  }
  fn alu_rrc(&mut self, value: u8, set_zero: bool) -> u8 {
    let co = value & 0x01;
    let new_value = value.rotate_right(1);
    self.regs.f = Flags::ZERO.test(set_zero && new_value == 0) |
                  Flags::CARRY.test(co != 0);
    new_value
  }
  fn ctrl_jp<H: Bus>(&mut self, bus: &mut H, addr: u16) {
    self.regs.pc = addr;
    self.internal_cycle(bus);
  }
  fn ctrl_jr<H: Bus>(&mut self, bus: &mut H, offset: i8) {
    self.regs.pc = self.regs.pc.wrapping_add(offset as u16);
    self.internal_cycle(bus);
  }
  fn ctrl_call<H: Bus>(&mut self, bus: &mut H, addr: u16) {
    let pc = self.regs.pc;
    self.internal_cycle(bus);
    self.push_u16(bus, pc);
    self.regs.pc = addr;
  }
  fn ctrl_ret<H: Bus>(&mut self, bus: &mut H) {
    self.regs.pc = self.pop_u16(bus);
    self.internal_cycle(bus);
  }
}


impl<'a, H> CpuOps for (&'a mut Cpu, &'a mut H) where H: Bus {
  type R = ();
  // --- 8-bit operations
  // 8-bit loads
  /// LD d, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load<O: Out8, I: In8>(self, out8: O, in8: I) {
    let (cpu, bus) = self;
    let value = in8.read(cpu, bus);
    out8.write(cpu, bus, value);
  }
  // 8-bit arithmetic
  /// ADD s
  ///
  /// Flags: Z N H C
  ///        * 0 * *
  fn add<I: In8>(self, in8: I) {
    let (cpu, bus) = self;
    let value = in8.read(cpu, bus);
    let (result, carry) = cpu.regs.a.overflowing_add(value);
    let half_carry = (cpu.regs.a & 0x0f).checked_add(value | 0xf0).is_none();
    cpu.regs.f = Flags::ZERO.test(result == 0) |
                  Flags::CARRY.test(carry) |
                  Flags::HALF_CARRY.test(half_carry);
    cpu.regs.a = result;
  }
  /// ADC s
  ///
  /// Flags: Z N H C
  ///        * 0 * *
  fn adc<I: In8>(self, in8: I) {
    let (cpu, bus) = self;
    let value = in8.read(cpu, bus);
    let cy = if cpu.regs.f.contains(Flags::CARRY) { 1 } else { 0 };
    let result = cpu.regs.a.wrapping_add(value).wrapping_add(cy);
    cpu.regs.f = Flags::ZERO.test(result == 0) |
                  Flags::CARRY.test(cpu.regs.a as u16 + value as u16 + cy as u16 > 0xff) |
                  Flags::HALF_CARRY.test((cpu.regs.a & 0xf) + (value & 0xf) + cy > 0xf);
    cpu.regs.a = result;
  }
  /// SUB s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  fn sub<I: In8>(self, in8: I) {
    let (cpu, bus) = self;
    let value = in8.read(cpu, bus);
    cpu.regs.a = cpu.alu_sub(value, false);
  }
  /// SBC s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  fn sbc<I: In8>(self, in8: I) {
    let (cpu, bus) = self;
    let value = in8.read(cpu, bus);
    cpu.regs.a = cpu.alu_sub(value, true);
  }
  /// CP s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  fn cp<I: In8>(self, in8: I) {
    let (cpu, bus) = self;
    let value = in8.read(cpu, bus);
    cpu.alu_sub(value, false);
  }
  /// AND s
  ///
  /// Flags: Z N H C
  ///        * 0 1 0
  fn and<I: In8>(self, in8: I) {
    let (cpu, bus) = self;
    let value = in8.read(cpu, bus);
    cpu.regs.a &= value;
    cpu.regs.f = Flags::ZERO.test(cpu.regs.a == 0) |
                  Flags::HALF_CARRY;
  }
  /// OR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  fn or<I: In8>(self, in8: I) {
    let (cpu, bus) = self;
    let value = in8.read(cpu, bus);
    cpu.regs.a |= value;
    cpu.regs.f = Flags::ZERO.test(cpu.regs.a == 0);
  }
  /// XOR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  fn xor<I: In8>(self, in8: I) {
    let (cpu, bus) = self;
    let value = in8.read(cpu, bus);
    cpu.regs.a ^= value;
    cpu.regs.f = Flags::ZERO.test(cpu.regs.a == 0)
  }
  /// INC s
  ///
  /// Flags: Z N H C
  ///        * 0 * -
  fn inc<IO: In8+Out8>(self, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus);
    let new_value = value.wrapping_add_one();
    cpu.regs.f = Flags::ZERO.test(new_value == 0) |
                  Flags::HALF_CARRY.test(value & 0xf == 0xf) |
                  (Flags::CARRY & cpu.regs.f);
    io.write(cpu, bus, new_value);
  }
  /// DEC s
  ///
  /// Flags: Z N H C
  ///        * 1 * -
  fn dec<IO: In8+Out8>(self, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus);
    let new_value = value.wrapping_sub_one();
    cpu.regs.f = Flags::ZERO.test(new_value == 0) |
                 Flags::ADD_SUBTRACT |
                 Flags::HALF_CARRY.test(value & 0xf == 0) |
                 (Flags::CARRY & cpu.regs.f);
    io.write(cpu, bus, new_value);
  }
  /// RLCA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rlca(self) {
    let (cpu, _) = self;
    let value = cpu.regs.a;
    cpu.regs.a = cpu.alu_rlc(value, false);
  }
  /// RLA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rla(self) {
    let (cpu, _) = self;
    let value = cpu.regs.a;
    cpu.regs.a = cpu.alu_rl(value, false);
  }
  /// RRCA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rrca(self) {
    let (cpu, _) = self;
    let value = cpu.regs.a;
    cpu.regs.a = cpu.alu_rrc(value, false);
  }
  /// RRA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rra(self) {
    let (cpu, _) = self;
    let value = cpu.regs.a;
    cpu.regs.a = cpu.alu_rr(value, false);
  }
  /// RLC s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rlc<IO: In8+Out8>(self, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus);
    let new_value = cpu.alu_rlc(value, true);
    io.write(cpu, bus, new_value);
  }
  /// RL s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rl<IO: In8+Out8>(self, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus);
    let new_value = cpu.alu_rl(value, true);
    io.write(cpu, bus, new_value);
  }
  /// RRC s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rrc<IO: In8+Out8>(self, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus);
    let new_value = cpu.alu_rrc(value, true);
    io.write(cpu, bus, new_value);
  }
  /// RR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rr<IO: In8+Out8>(self, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus);
    let new_value = cpu.alu_rr(value, true);
    io.write(cpu, bus, new_value);
  }
  /// SLA s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn sla<IO: In8+Out8>(self, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus);
    let co = value & 0x80;
    let new_value = value << 1;
    cpu.regs.f = Flags::ZERO.test(new_value == 0) |
                  Flags::CARRY.test(co != 0);
    io.write(cpu, bus, new_value);
  }
  /// SRA s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn sra<IO: In8+Out8>(self, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus);
    let co = value & 0x01;
    let hi = value & 0x80;
    let new_value = (value >> 1) | hi;
    cpu.regs.f = Flags::ZERO.test(new_value == 0) |
                  Flags::CARRY.test(co != 0);
    io.write(cpu, bus, new_value);
  }
  /// SRL s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn srl<IO: In8+Out8>(self, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus);
    let co = value & 0x01;
    let new_value = value >> 1;
    cpu.regs.f = Flags::ZERO.test(new_value == 0) |
                  Flags::CARRY.test(co != 0);
    io.write(cpu, bus, new_value);
  }
  /// SWAP s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  fn swap<IO: In8+Out8>(self, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus);
    let new_value = (value >> 4) | (value << 4);
    cpu.regs.f = Flags::ZERO.test(value == 0);
    io.write(cpu, bus, new_value);
  }
  /// BIT b, s
  ///
  /// Flags: Z N H C
  ///        * 0 1 -
  fn bit<I: In8>(self, bit: usize, in8: I) {
    let (cpu, bus) = self;
    let value = in8.read(cpu, bus) & (1 << bit);
    cpu.regs.f = Flags::ZERO.test(value == 0) |
                  Flags::HALF_CARRY |
                  (Flags::CARRY & cpu.regs.f);
  }
  /// SET b, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn set<IO: In8+Out8>(self, bit: usize, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus) | (1 << bit);
    io.write(cpu, bus, value);
  }
  /// RES b, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn res<IO: In8+Out8>(self, bit: usize, io: IO) {
    let (cpu, bus) = self;
    let value = io.read(cpu, bus) & !(1 << bit);
    io.write(cpu, bus, value);
  }
  // --- Control
  /// JP nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jp(self) {
    let (cpu, bus) = self;
    let addr = cpu.next_u16(bus);
    cpu.ctrl_jp(bus, addr);
  }
  /// JP HL
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jp_hl(self) {
    let (cpu, _) = self;
    cpu.regs.pc = cpu.regs.read16(Reg16::HL);
  }
  /// JR e
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jr(self) {
    let (cpu, bus) = self;
    let offset = cpu.next_u8(bus) as i8;
    cpu.ctrl_jr(bus, offset);
  }
  /// CALL nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn call(self) {
    let (cpu, bus) = self;
    let addr = cpu.next_u16(bus);
    cpu.ctrl_call(bus, addr);
  }
  /// RET
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn ret(self) {
    let (cpu, bus) = self;
    cpu.ctrl_ret(bus);
  }
  /// RETI
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn reti(self) {
    let (cpu, bus) = self;
    cpu.ime = Ime::Enabled;
    cpu.ctrl_ret(bus);
  }
  /// JP cc, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jp_cc(self, cond: Cond) {
    let (cpu, bus) = self;
    let addr = cpu.next_u16(bus);
    if cond.check(cpu.regs.f) {
      cpu.ctrl_jp(bus, addr);
    }
  }
  /// JR cc, e
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jr_cc(self, cond: Cond) {
    let (cpu, bus) = self;
    let offset = cpu.next_u8(bus) as i8;
    if cond.check(cpu.regs.f) {
      cpu.ctrl_jr(bus, offset);
    }
  }
  /// CALL cc, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn call_cc(self, cond: Cond) {
    let (cpu, bus) = self;
    let addr = cpu.next_u16(bus);
    if cond.check(cpu.regs.f) {
      cpu.ctrl_call(bus, addr);
    }
  }
  /// RET cc
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn ret_cc(self, cond: Cond) {
    let (cpu, bus) = self;
    cpu.internal_cycle(bus);
    if cond.check(cpu.regs.f) {
      cpu.ctrl_ret(bus);
    }
  }
  /// RST n
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn rst(self, addr: u8) {
    let (cpu, bus) = self;
    let pc = cpu.regs.pc;
    cpu.internal_cycle(bus);
    cpu.push_u16(bus, pc);
    cpu.regs.pc = addr as u16;
  }
  // --- Miscellaneous
  /// HALT
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn halt(self) {
    let (cpu, _) = self;
    // TODO: DMG BUG
    cpu.halt = true;
  }
  /// STOP
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn stop(self) {
    panic!("STOP")
  }
  /// DI
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn di(self) {
    let (cpu, _) = self;
    cpu.ime = Ime::Disabled;
  }
  /// EI
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn ei(self) {
    let (cpu, _) = self;
    cpu.ime = match cpu.ime {
      Ime::Disabled => Ime::Enabling,
      _ => cpu.ime,
    }
  }
  /// CCF
  ///
  /// Flags: Z N H C
  ///        - 0 0 *
  fn ccf(self) {
    let (cpu, _) = self;
    cpu.regs.f = (Flags::ZERO & cpu.regs.f) |
                  Flags::CARRY.test(!cpu.regs.f.contains(Flags::CARRY))
  }
  /// SCF
  ///
  /// Flags: Z N H C
  ///        - 0 0 1
  fn scf(self) {
    let (cpu, _) = self;
    cpu.regs.f = (Flags::ZERO & cpu.regs.f) |
                  Flags::CARRY
  }
  /// NOP
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn nop(self) {
  }
  /// DAA
  ///
  /// Flags: Z N H C
  ///        * - 0 *
  fn daa(self) {
    let (cpu, _) = self;
    // DAA table in page 110 of the official "Game Boy Programming Manual"
    let mut carry = false;
    if !cpu.regs.f.contains(Flags::ADD_SUBTRACT) {
      if cpu.regs.f.contains(Flags::CARRY) || cpu.regs.a > 0x99 {
        cpu.regs.a = cpu.regs.a.wrapping_add(0x60);
        carry = true;
      }
      if cpu.regs.f.contains(Flags::HALF_CARRY) || cpu.regs.a & 0x0f > 0x09 {
        cpu.regs.a = cpu.regs.a.wrapping_add(0x06);
      }
    } else if cpu.regs.f.contains(Flags::CARRY) {
      carry = true;
      cpu.regs.a = cpu.regs.a.wrapping_add(
        if cpu.regs.f.contains(Flags::HALF_CARRY) { 0x9a }
        else { 0xa0 }
        );
    } else if cpu.regs.f.contains(Flags::HALF_CARRY) {
      cpu.regs.a = cpu.regs.a.wrapping_add(0xfa);
    }

    cpu.regs.f = Flags::ZERO.test(cpu.regs.a == 0) |
                  (Flags::ADD_SUBTRACT & cpu.regs.f) |
                  Flags::CARRY.test(carry);
  }
  /// CPL
  ///
  /// Flags: Z N H C
  ///        - 1 1 -
  fn cpl(self) {
    let (cpu, _) = self;
    cpu.regs.a = !cpu.regs.a;
    cpu.regs.f = (Flags::ZERO & cpu.regs.f) |
                  Flags::ADD_SUBTRACT |
                  Flags::HALF_CARRY |
                  (Flags::CARRY & cpu.regs.f);
  }
  // --- 16-bit operations
  // 16-bit loads
  /// LD dd, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load16_imm(self, reg: Reg16) {
    let (cpu, bus) = self;
    let value = cpu.next_u16(bus);
    cpu.regs.write16(reg, value);
  }
  /// LD (nn), SP
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load16_nn_sp(self) {
    let (cpu, bus) = self;
    let value = cpu.regs.sp;
    let addr = cpu.next_u16(bus);
    cpu.write_cycle(bus, addr, value as u8);
    cpu.write_cycle(bus, (addr.wrapping_add_one()), (value >> 8) as u8);
  }
  /// LD SP, HL
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load16_sp_hl(self) {
    let (cpu, bus) = self;
    let value = cpu.regs.read16(Reg16::HL);
    cpu.regs.sp = value;
    cpu.internal_cycle(bus);
  }
  /// LD HL, SP+e
  ///
  /// Flags: Z N H C
  ///        0 0 * *
  fn load16_hl_sp_e(self) {
    let (cpu, bus) = self;
    let offset = cpu.next_u8(bus) as i8 as u16;
    let sp = cpu.regs.sp as u16;
    let value = sp.wrapping_add(offset);
    cpu.regs.write16(Reg16::HL, value);
    cpu.regs.f = Flags::HALF_CARRY.test(u16::test_add_carry_bit(3, sp, offset)) |
                  Flags::CARRY.test(u16::test_add_carry_bit(7, sp, offset));
    cpu.internal_cycle(bus);
  }
  /// PUSH rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn push16(self, reg: Reg16) {
    let (cpu, bus) = self;
    let value = cpu.regs.read16(reg);
    cpu.internal_cycle(bus);
    cpu.push_u16(bus, value);
  }
  /// POP rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  /// Note! POP AF affects all flags
  fn pop16(self, reg: Reg16) {
    let (cpu, bus) = self;
    let value = cpu.pop_u16(bus);
    cpu.regs.write16(reg, value);
  }
  // 16-bit arithmetic
  /// ADD HL, ss
  ///
  /// Flags: Z N H C
  ///        - 0 * *
  fn add16(self, reg: Reg16) {
    let (cpu, bus) = self;
    let hl = cpu.regs.read16(Reg16::HL);
    let value = cpu.regs.read16(reg);
    let result = hl.wrapping_add(value);
    cpu.regs.f = (Flags::ZERO & cpu.regs.f) |
                  Flags::HALF_CARRY.test(u16::test_add_carry_bit(11, hl, value)) |
                  Flags::CARRY.test(hl > 0xffff - value);
    cpu.regs.write16(Reg16::HL, result);
    cpu.internal_cycle(bus);
  }
  /// ADD SP, e
  ///
  /// Flags: Z N H C
  ///        0 0 * *
  fn add16_sp_e(self) {
    let (cpu, bus) = self;
    let val = cpu.next_u8(bus) as i8 as i16 as u16;
    let sp = cpu.regs.sp;
    cpu.regs.sp = sp.wrapping_add(val);
    cpu.regs.f = Flags::HALF_CARRY.test(u16::test_add_carry_bit(3, sp, val)) |
      Flags::CARRY.test(u16::test_add_carry_bit(7, sp, val));
    cpu.internal_cycle(bus);
    cpu.internal_cycle(bus);
  }
  /// INC rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn inc16(self, reg: Reg16) {
    let (cpu, bus) = self;
    let value = cpu.regs.read16(reg).wrapping_add_one();
    cpu.regs.write16(reg, value);
    cpu.internal_cycle(bus);
  }
  /// DEC rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn dec16(self, reg: Reg16) {
    let (cpu, bus) = self;
    let value = cpu.regs.read16(reg).wrapping_sub_one();
    cpu.regs.write16(reg, value);
    cpu.internal_cycle(bus);
  }
  // --- Undefined
  fn undefined(self, op: u8) {
    panic!("Undefined opcode {}", op)
  }
  fn undefined_debug(self) {
    let (_, bus) = self;
    bus.trigger_emu_events(EmuEvents::DEBUG_OP);
  }
  fn cb_prefix(self) {
    let (cpu, bus) = self;
    let op = cpu.next_u8(bus);
    ops::decode_cb((cpu, bus), op)
  }
}
