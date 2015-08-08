// This file is part of Mooneye GB.
// Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use std::borrow::Cow;
use std::fmt;

use cpu;
use cpu::{
  CpuOps, Cond,
  In8, Out8
};
use cpu::ops;
use cpu::registers::{Reg8, Reg16};

pub type DisasmStr = Cow<'static, str>;

#[derive(Clone, Copy, Debug)]
pub enum Operand8 {
  Register(Reg8),
  Immediate(u8),
  Memory(Addr)
}

#[derive(Clone, Copy, Debug)]
pub enum Addr {
  BC, DE, HL, HLD, HLI,
  ZeroPageC, Direct(u16),
  ZeroPage(u8)
}

#[derive(Clone, Copy, Debug)]
pub enum Instr {
  Load(Operand8, Operand8),
  Add(Operand8),
  Adc(Operand8),
  Sub(Operand8),
  Sbc(Operand8),
  Cp(Operand8),
  And(Operand8),
  Or(Operand8),
  Xor(Operand8),
  Inc(Operand8),
  Dec(Operand8),
  Rlca,
  Rla,
  Rrca,
  Rra,
  Rlc(Operand8),
  Rl(Operand8),
  Rrc(Operand8),
  Rr(Operand8),
  Sla(Operand8),
  Sra(Operand8),
  Srl(Operand8),
  Swap(Operand8),
  Bit(usize, Operand8),
  Set(usize, Operand8),
  Res(usize, Operand8),
  Jp(u16),
  JpHl,
  Jr(i8, u16),
  Call(u16),
  Ret,
  Reti,
  JpCc(Cond, u16),
  JrCc(Cond, i8, u16),
  CallCc(Cond, u16),
  RetCc(Cond),
  Rst(u8),
  Halt,
  Stop,
  Di,
  Ei,
  Ccf,
  Scf,
  Nop,
  Daa,
  Cpl,
  Load16(Reg16, u16),
  Load16NnSp(u16),
  Load16SpHl,
  Load16HlSpE(i8),
  Push16(Reg16),
  Pop16(Reg16),
  Add16(Reg16),
  Add16SpE(i8),
  Inc16(Reg16),
  Dec16(Reg16),
  Undefined(u8),
  UndefinedDebug
}

pub trait ToDisasmStr {
  fn to_disasm_str(&self) -> DisasmStr;
}

impl ToDisasmStr for u8 {
  fn to_disasm_str(&self) -> DisasmStr {
    (format!("${:02x}", *self)).into()
  }
}

impl ToDisasmStr for u16 {
  fn to_disasm_str(&self) -> DisasmStr {
    (format!("${:04x}", *self)).into()
  }
}

impl ToDisasmStr for &'static str {
  fn to_disasm_str(&self) -> DisasmStr { (*self).into() }
}

impl ToDisasmStr for usize {
  fn to_disasm_str(&self) -> DisasmStr {
    (format!("{}", *self)).into()
  }
}

impl ToDisasmStr for Cond {
  fn to_disasm_str(&self) -> DisasmStr {
    (format!("{:?}", *self)).into()
  }
}

impl ToDisasmStr for Reg8 {
  fn to_disasm_str(&self) -> DisasmStr {
    (format!("{:?}", *self)).into()
  }
}

impl ToDisasmStr for Reg16 {
  fn to_disasm_str(&self) -> DisasmStr {
    (format!("{:?}", *self)).into()
  }
}

impl ToDisasmStr for Operand8 {
  fn to_disasm_str(&self) -> DisasmStr {
    match *self {
      Operand8::Register(reg) => reg.to_disasm_str(),
      Operand8::Immediate(val) => (format!("${:02x}", val)).into(),
      Operand8::Memory(addr) => addr.to_disasm_str()
    }
  }
}

impl ToDisasmStr for Addr {
  fn to_disasm_str(&self) -> DisasmStr {
    use self::Addr::*;
    match *self {
      BC => "(BC)".into(), DE => "(DE)".into(),
      HL => "(HL)".into(),
      HLD => "(HL-)".into(), HLI => "(HL+)".into(),
      ZeroPageC => "($ff00+C)".into(),
      Direct(addr) => (format!("(${:04x})", addr)).into(),
      ZeroPage(addr) => (format!("($ff00+${:02x})", addr)).into()
    }
  }
}

impl fmt::Display for Instr {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.to_disasm_str())
  }
}

fn null_op(op: &'static str) -> DisasmStr { op.into() }
fn unary_op<A: ToDisasmStr>(op: &'static str, arg: A) -> DisasmStr {
  (format!("{} {}", op, arg.to_disasm_str())).into()
}
fn binary_op<A: ToDisasmStr, B: ToDisasmStr>(op: &'static str, arg1: A, arg2: B) -> DisasmStr {
  (format!("{} {}, {}", op, arg1.to_disasm_str(), arg2.to_disasm_str())).into()
}

impl ToDisasmStr for Instr {
  fn to_disasm_str(&self) -> DisasmStr {
    use self::Instr::*;
    match *self {
      Load(out8, in8) => binary_op("LD", out8, in8),
      Add(io) => unary_op("ADD", io),
      Adc(io) => unary_op("ADC", io),
      Sub(io) => unary_op("SUB", io),
      Sbc(io) => unary_op("SBC", io),
      Cp(io) => unary_op( "CP", io),
      And(io) => unary_op("AND", io),
      Or(io) => unary_op( "OR", io),
      Xor(io) => unary_op("XOR", io),
      Inc(io) => unary_op("INC", io),
      Dec(io) => unary_op("DEC", io),
      Rlca => null_op("RLCA"),
      Rla => null_op("RLA"),
      Rrca => null_op("RRCA"),
      Rra => null_op("RRA"),
      Rlc(io) => unary_op("RLC", io),
      Rl(io) => unary_op("RL", io),
      Rrc(io) => unary_op("RRC", io),
      Rr(io) => unary_op("RR", io),
      Sla(io) => unary_op("SLA", io),
      Sra(io) => unary_op("SRA", io),
      Srl(io) => unary_op("SRL", io),
      Swap(io) => unary_op("SWAP", io),
      Bit(bit, io) => binary_op("BIT", bit, io),
      Set(bit, io) => binary_op("SET", bit, io),
      Res(bit, io) => binary_op("RES", bit, io),
      Jp(addr) => unary_op("JP", addr),
      JpHl => null_op("JP HL"),
      Jr(_, addr) => unary_op("JR", addr),
      Call(addr) => unary_op("CALL", addr),
      Ret => null_op("RET"),
      Reti => null_op("RETI"),
      JpCc(cc, addr) => binary_op("JP", cc, addr),
      JrCc(cc, _, addr) => binary_op("JR", cc, addr),
      CallCc(cc, addr) => binary_op("CALL", cc, addr),
      RetCc(cc) => unary_op("RET", cc),
      Rst(addr) => unary_op("RST", addr),
      Halt => null_op("HALT"),
      Stop => null_op("STOP"),
      Di => null_op("DI"),
      Ei => null_op("EI"),
      Ccf => null_op("CCF"),
      Scf => null_op("SCF"),
      Nop => null_op("NOP"),
      Daa => null_op("DAA"),
      Cpl => null_op("CPL"),
      Load16(reg, val) => binary_op("LD", reg, val),
      Load16NnSp(addr) => (format!("LD (${:04x}), SP", addr)).into(),
      Load16SpHl => null_op("LD SP, HL"),
      Load16HlSpE(offset) => (format!("LD HL, SP{:+2x}", offset)).into(),
      Push16(reg) => unary_op("PUSH", reg),
      Pop16(reg) => unary_op("POP", reg),
      Add16(reg) => binary_op("ADD", "HL", reg),
      Add16SpE(offset) => (format!("ADD SP, ${:02x}", offset)).into(),
      Inc16(reg) => unary_op("INC", reg),
      Dec16(reg) => unary_op("DEC", reg),
      Undefined(op) => (format!("${:02x} ??", op)).into(),
      UndefinedDebug => null_op("DBG")
    }
  }
}

struct Disasm<'a> {
  pc: u16,
  reader: &'a mut FnMut(u16) -> u8
}

impl<'a> Disasm<'a> {
  fn next_u8(&mut self) -> u8 {
    let addr = self.pc;
    self.pc += 1;
    (self.reader)(addr)
  }
  fn next_u16(&mut self) -> u16 {
    let l = self.next_u8();
    let h = self.next_u8();
    ((h as u16) << 8) | (l as u16)
  }
}

pub trait ResolveOp8 {
  fn resolve<'a>(&self, disasm: &mut Disasm<'a>) -> Operand8;
}

impl ResolveOp8 for Reg8 {
  fn resolve<'a>(&self, _: &mut Disasm<'a>) -> Operand8 { Operand8::Register(*self) }
}

impl ResolveOp8 for cpu::Immediate8 {
  fn resolve<'a>(&self, disasm: &mut Disasm<'a>) -> Operand8 { Operand8::Immediate(disasm.next_u8()) }
}

impl ResolveOp8 for cpu::Addr {
  fn resolve<'a>(&self, disasm: &mut Disasm<'a>) -> Operand8 {
    Operand8::Memory(match *self {
      cpu::Addr::BC => Addr::BC,
      cpu::Addr::DE => Addr::DE,
      cpu::Addr::HL => Addr::HL,
      cpu::Addr::HLD => Addr::HLD,
      cpu::Addr::HLI => Addr::HLI,
      cpu::Addr::ZeroPageC => Addr::ZeroPageC,
      cpu::Addr::Direct => Addr::Direct(disasm.next_u16()),
      cpu::Addr::ZeroPage => Addr::ZeroPage(disasm.next_u8())
    })
  }
}

impl<'a, 'b> CpuOps for &'a mut Disasm<'b> {
  type R = Instr;
  // --- 8-bit operations
  // 8-bit loads
  fn load<O: Out8, I: In8>(self, out8: O, in8: I) -> Instr {
    Instr::Load(out8.resolve(self), in8.resolve(self))
  }
  // 8-bit arithmetic
  fn add<I: In8>(self, in8: I) -> Instr { Instr::Add(in8.resolve(self)) }
  fn adc<I: In8>(self, in8: I) -> Instr { Instr::Adc(in8.resolve(self)) }
  fn sub<I: In8>(self, in8: I) -> Instr { Instr::Sub(in8.resolve(self)) }
  fn sbc<I: In8>(self, in8: I) -> Instr { Instr::Sbc(in8.resolve(self)) }
  fn  cp<I: In8>(self, in8: I) -> Instr {  Instr::Cp(in8.resolve(self)) }
  fn and<I: In8>(self, in8: I) -> Instr { Instr::And(in8.resolve(self)) }
  fn  or<I: In8>(self, in8: I) -> Instr {  Instr::Or(in8.resolve(self)) }
  fn xor<I: In8>(self, in8: I) -> Instr { Instr::Xor(in8.resolve(self)) }
  fn inc<IO: In8+Out8>(self, io: IO) -> Instr { Instr::Inc(io.resolve(self)) }
  fn dec<IO: In8+Out8>(self, io: IO) -> Instr { Instr::Dec(io.resolve(self)) }
  fn rlca(self) -> Instr { Instr::Rlca }
  fn  rla(self) -> Instr { Instr::Rla }
  fn rrca(self) -> Instr { Instr::Rrca }
  fn  rra(self) -> Instr { Instr::Rra }
  fn  rlc<IO: In8+Out8>(self, io: IO) -> Instr { Instr::Rlc(io.resolve(self)) }
  fn   rl<IO: In8+Out8>(self, io: IO) -> Instr { Instr::Rl(io.resolve(self)) }
  fn  rrc<IO: In8+Out8>(self, io: IO) -> Instr { Instr::Rrc(io.resolve(self)) }
  fn   rr<IO: In8+Out8>(self, io: IO) -> Instr { Instr::Rr(io.resolve(self)) }
  fn  sla<IO: In8+Out8>(self, io: IO) -> Instr { Instr::Sla(io.resolve(self)) }
  fn  sra<IO: In8+Out8>(self, io: IO) -> Instr { Instr::Sra(io.resolve(self)) }
  fn  srl<IO: In8+Out8>(self, io: IO) -> Instr { Instr::Srl(io.resolve(self)) }
  fn swap<IO: In8+Out8>(self, io: IO) -> Instr { Instr::Swap(io.resolve(self)) }
  fn  bit<I:  In8>     (self, bit: usize, in8: I) -> Instr { Instr::Bit(bit, in8.resolve(self)) }
  fn  set<IO: In8+Out8>(self, bit: usize, io: IO) -> Instr { Instr::Set(bit, io.resolve(self)) }
  fn  res<IO: In8+Out8>(self, bit: usize, io: IO) -> Instr { Instr::Res(bit, io.resolve(self)) }
  // --- Control
  fn      jp(self) -> Instr { Instr::Jp(self.next_u16()) }
  fn   jp_hl(self) -> Instr { Instr::JpHl }
  fn      jr(self) -> Instr {
    let offset = self.next_u8() as i8;
    let addr = (self.pc as i16 + offset as i16) as u16;
    Instr::Jr(offset, addr)
  }
  fn    call(self) -> Instr { Instr::Call(self.next_u16()) }
  fn     ret(self) -> Instr { Instr::Ret }
  fn    reti(self) -> Instr { Instr::Reti }
  fn   jp_cc(self, cond: Cond) -> Instr { Instr::JpCc(cond, self.next_u16()) }
  fn   jr_cc(self, cond: Cond) -> Instr {
    let offset = self.next_u8() as i8;
    let addr = (self.pc as i16 + offset as i16) as u16;
    Instr::JrCc(cond, offset, addr)
  }
  fn call_cc(self, cond: Cond) -> Instr { Instr::CallCc(cond, self.next_u16()) }
  fn  ret_cc(self, cond: Cond) -> Instr { Instr::RetCc(cond) }
  fn     rst(self, addr: u8) -> Instr { Instr::Rst(addr) }
  // --- Miscellaneous
  fn halt(self) -> Instr { Instr::Halt }
  fn stop(self) -> Instr { Instr::Stop }
  fn   di(self) -> Instr { Instr::Di }
  fn   ei(self) -> Instr { Instr::Ei }
  fn  ccf(self) -> Instr { Instr::Ccf }
  fn  scf(self) -> Instr { Instr::Scf }
  fn  nop(self) -> Instr { Instr::Nop }
  fn  daa(self) -> Instr { Instr::Daa }
  fn  cpl(self) -> Instr { Instr::Cpl }
  // --- 16-bit operations
  // 16-bit loads
  fn load16_imm(self, reg: Reg16) -> Instr { Instr::Load16(reg, self.next_u16()) }
  fn load16_nn_sp(self) -> Instr { Instr::Load16NnSp(self.next_u16()) }
  fn load16_sp_hl(self) -> Instr { Instr::Load16SpHl }
  fn load16_hl_sp_e(self) -> Instr { Instr::Load16HlSpE(self.next_u8() as i8) }
  // 16-bit arithmetic
  fn push16(self, reg: Reg16) -> Instr { Instr::Push16(reg) }
  fn  pop16(self, reg: Reg16) -> Instr { Instr::Pop16(reg) }
  fn  add16(self, reg: Reg16) -> Instr { Instr::Add16(reg) }
  fn add16_sp_e(self) -> Instr { Instr::Add16SpE(self.next_u8() as i8) }
  fn inc16(self, reg: Reg16) -> Instr { Instr::Inc16(reg) }
  fn dec16(self, reg: Reg16) -> Instr { Instr::Dec16(reg) }
  // --- Undefined
  fn undefined(self, op: u8) -> Instr { Instr::Undefined(op) }
  fn undefined_debug(self) -> Instr { Instr::UndefinedDebug }
  fn cb_prefix(self) -> Instr {
    let op = self.next_u8();
    ops::decode_cb(self, op)
  }
}

pub fn disasm<'a, F: 'a+FnMut(u16) -> u8>(pc_start: u16, reader: &'a mut F) -> Instr {
  let mut disasm = Disasm {
    pc: pc_start,
    reader: reader
  };
  let op = disasm.next_u8();
  ops::decode(&mut disasm, op)
}
