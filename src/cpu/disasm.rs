use std::borrow::Cow;
use std::fmt::Debug;

use cpu::{
  CpuOps, Cond,
  In8, Out8,
  Immediate8, Addr
};
use cpu::ops;
use cpu::registers::Reg16;
use emulation::EmuTime;
use hardware::Bus;

pub type DisasmStr = Cow<'static, str>;

struct Disasm<'a> {
  pc: u16,
  bus: &'a (Bus + 'a),
  time: EmuTime
}

pub trait ToDisasmStr {
  fn to_disasm_str<'a>(&self, &mut Disasm<'a>) -> DisasmStr;
}

impl<T> ToDisasmStr for T where T: Debug {
  fn to_disasm_str<'a>(&self, _: &mut Disasm<'a>) -> DisasmStr {
    (format!("{:?}", *self)).into()
  }
}

impl ToDisasmStr for Immediate8 {
  fn to_disasm_str<'a>(&self, disasm: &mut Disasm<'a>) -> DisasmStr {
    (format!("${:02x}", disasm.next_u8())).into()
  }
}

impl ToDisasmStr for Addr {
  fn to_disasm_str<'a>(&self, disasm: &mut Disasm<'a>) -> DisasmStr {
    match *self {
      Addr::BC => "(BC)".into(), Addr::DE => "(DE)".into(),
      Addr::HL => "(HL)".into(),
      Addr::HLD => "(HL-)".into(), Addr::HLI => "(HL+)".into(),
      Addr::ZeroPageC => "($FF00+C)".into(),
      Addr::Direct => (format!("(${:04x})", disasm.next_u16())).into(),
      Addr::ZeroPage => (format!("($FF00+${:02x})", disasm.next_u8())).into()
    }
  }
}

struct Immediate16;
impl ToDisasmStr for Immediate16 {
  fn to_disasm_str<'a>(&self, disasm: &mut Disasm<'a>) -> DisasmStr {
    (format!("${:04x}", disasm.next_u16())).into()
  }
}

struct Direct16;
impl ToDisasmStr for Direct16 {
  fn to_disasm_str<'a>(&self, disasm: &mut Disasm<'a>) -> DisasmStr {
    (format!("(${:04x})", disasm.next_u16())).into()
  }
}

struct PcOffset;
impl ToDisasmStr for PcOffset {
  fn to_disasm_str<'a>(&self, disasm: &mut Disasm<'a>) -> DisasmStr {
    let offset = disasm.next_u8() as i8;
    let addr = (disasm.pc as i16 + offset as i16) as u16;
    (format!("${:04x}", addr)).into()
  }
}

impl<'a> Disasm<'a> {
  fn next_u8(&mut self) -> u8 {
    let addr = self.pc;
    self.pc += 1;
    self.bus.read(self.time, addr)
  }
  fn next_u16(&mut self) -> u16 {
    let l = self.next_u8();
    let h = self.next_u8();
    ((h as u16) << 8) | (l as u16)
  }
  fn null_op(&mut self, op: &'static str) -> DisasmStr { op.into() }
  fn unary_op<A: ToDisasmStr>(&mut self, op: &'static str, arg: A) -> DisasmStr {
    (format!("{} {}", op, arg.to_disasm_str(self))).into()
  }
  fn binary_op<A: ToDisasmStr, B: ToDisasmStr>(&mut self, op: &'static str, arg1: A, arg2: B) -> DisasmStr {
    (format!("{} {}, {}", op, arg1.to_disasm_str(self), arg2.to_disasm_str(self))).into()
  }
}

impl<'a, 'b> CpuOps for &'a mut Disasm<'b> {
  type R = DisasmStr;
  // --- 8-bit operations
  // 8-bit loads
  fn load<O: Out8, I: In8>(self, out8: O, in8: I) -> DisasmStr { self.binary_op("LD", out8, in8) }
  // 8-bit arithmetic
  fn add<I: In8>(self, in8: I) -> DisasmStr { self.unary_op("ADD", in8) }
  fn adc<I: In8>(self, in8: I) -> DisasmStr { self.unary_op("ADC", in8) }
  fn sub<I: In8>(self, in8: I) -> DisasmStr { self.unary_op("SUB", in8) }
  fn sbc<I: In8>(self, in8: I) -> DisasmStr { self.unary_op("SBC", in8) }
  fn  cp<I: In8>(self, in8: I) -> DisasmStr { self.unary_op( "CP", in8) }
  fn and<I: In8>(self, in8: I) -> DisasmStr { self.unary_op("AND", in8) }
  fn  or<I: In8>(self, in8: I) -> DisasmStr { self.unary_op( "OR", in8) }
  fn xor<I: In8>(self, in8: I) -> DisasmStr { self.unary_op("XOR", in8) }
  fn inc<IO: In8+Out8>(self, io: IO) -> DisasmStr { self.unary_op("INC", io) }
  fn dec<IO: In8+Out8>(self, io: IO) -> DisasmStr { self.unary_op("DEC", io) }
  fn rlca(self) -> DisasmStr { self.null_op("RLCA") }
  fn  rla(self) -> DisasmStr { self.null_op( "RLA") }
  fn rrca(self) -> DisasmStr { self.null_op("RRCA") }
  fn  rra(self) -> DisasmStr { self.null_op( "RRA") }
  fn  rlc<IO: In8+Out8>(self, io: IO) -> DisasmStr { self.unary_op( "RLC", io) }
  fn   rl<IO: In8+Out8>(self, io: IO) -> DisasmStr { self.unary_op(  "RL", io) }
  fn  rrc<IO: In8+Out8>(self, io: IO) -> DisasmStr { self.unary_op( "RRC", io) }
  fn   rr<IO: In8+Out8>(self, io: IO) -> DisasmStr { self.unary_op(  "RR", io) }
  fn  sla<IO: In8+Out8>(self, io: IO) -> DisasmStr { self.unary_op( "SLA", io) }
  fn  sra<IO: In8+Out8>(self, io: IO) -> DisasmStr { self.unary_op( "SRA", io) }
  fn  srl<IO: In8+Out8>(self, io: IO) -> DisasmStr { self.unary_op( "SRL", io) }
  fn swap<IO: In8+Out8>(self, io: IO) -> DisasmStr { self.unary_op("SWAP", io) }
  fn  bit<I:  In8>     (self, bit: usize, in8: I) -> DisasmStr { self.binary_op("BIT", bit, in8) }
  fn  set<IO: In8+Out8>(self, bit: usize, io: IO) -> DisasmStr { self.binary_op("SET", bit, io) }
  fn  res<IO: In8+Out8>(self, bit: usize, io: IO) -> DisasmStr { self.binary_op("RES", bit, io) }
  // --- Control
  fn      jp(self) -> DisasmStr { self.unary_op(  "JP", Immediate16) }
  fn   jp_hl(self) -> DisasmStr { self.null_op("JP HL") }
  fn      jr(self) -> DisasmStr { self.unary_op(  "JR", PcOffset) }
  fn    call(self) -> DisasmStr { self.unary_op("CALL", Immediate16) }
  fn     ret(self) -> DisasmStr { self.null_op(  "RET") }
  fn    reti(self) -> DisasmStr { self.null_op( "RETI") }
  fn   jp_cc(self, cond: Cond) -> DisasmStr { self.binary_op(  "JP", cond, Immediate16) }
  fn   jr_cc(self, cond: Cond) -> DisasmStr { self.binary_op(  "JR", cond, PcOffset) }
  fn call_cc(self, cond: Cond) -> DisasmStr { self.binary_op("CALL", cond, Immediate16) }
  fn  ret_cc(self, cond: Cond) -> DisasmStr { self.unary_op(  "RET", cond) }
  fn     rst(self, addr: u8) -> DisasmStr {
    (format!("RST, ${:02x}", addr)).into()
  }
  // --- Miscellaneous
  fn halt(self) -> DisasmStr { self.null_op("HALT") }
  fn stop(self) -> DisasmStr { self.null_op("STOP") }
  fn   di(self) -> DisasmStr { self.null_op(  "DI") }
  fn   ei(self) -> DisasmStr { self.null_op(  "EI") }
  fn  ccf(self) -> DisasmStr { self.null_op( "CCF") }
  fn  scf(self) -> DisasmStr { self.null_op( "SCF") }
  fn  nop(self) -> DisasmStr { self.null_op( "NOP") }
  fn  daa(self) -> DisasmStr { self.null_op( "DAA") }
  fn  cpl(self) -> DisasmStr { self.null_op( "CPL") }
  // --- 16-bit operations
  // 16-bit loads
  fn load16_imm(self, reg: Reg16) -> DisasmStr { self.binary_op("LD", reg, Immediate16) }
  fn load16_nn_sp(self) -> DisasmStr { self.binary_op("LD", Direct16, Reg16::SP)}
  fn load16_sp_hl(self) -> DisasmStr { self.null_op("LD SP, HL") }
  fn load16_hl_sp_e(self) -> DisasmStr {
    let offset = self.next_u8() as i8;
    (format!("LD HL, SP{:+2x}", offset)).into()
  }
  // 16-bit arithmetic
  fn push16(self, reg: Reg16) -> DisasmStr { self.unary_op("PUSH", reg) }
  fn  pop16(self, reg: Reg16) -> DisasmStr { self.unary_op( "POP", reg) }
  fn  add16(self, reg: Reg16) -> DisasmStr { self.binary_op("ADD", "HL", reg) }
  fn add16_sp_e(self) -> DisasmStr {
    let offset = self.next_u8() as i8;
    (format!("ADD SP, ${:02x}", offset)).into()
  }
  fn inc16(self, reg: Reg16) -> DisasmStr { self.unary_op("INC", reg) }
  fn dec16(self, reg: Reg16) -> DisasmStr { self.unary_op("DEC", reg) }
  // --- Undefined
  fn undefined(self, op: u8) -> DisasmStr {
    (format!("${:02x} ??", op)).into()
  }
  fn undefined_debug(self) -> DisasmStr { self.null_op("DBG") }
  fn cb_prefix(self) -> DisasmStr {
    let op = self.next_u8();
    ops::decode_cb(self, op)
  }
}

pub fn disasm<B: Bus>(bus: &B, pc_start: u16, time: EmuTime) -> DisasmStr {
  let mut disasm = Disasm {
    pc: pc_start,
    bus: bus,
    time: time
  };
  let op = disasm.next_u8();
  ops::decode(&mut disasm, op)
}
