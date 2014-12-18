use std::fmt::Show;
use std::str::CowString;

use cpu::{
  CpuOps, Cond,
  In8, Out8,
  In16, Out16,
  Immediate8, Addr,
  Immediate16, Direct16
};
use cpu::registers::Reg16;
use hardware::Bus;

pub type DisasmStr = CowString<'static>;

struct Disasm<'a> {
  pc: u16,
  bus: &'a (Bus + 'a)
}

pub trait ToDisasmStr {
  fn to_disasm_str<'a>(&self, &mut Disasm<'a>) -> DisasmStr;
}

impl<T> ToDisasmStr for T where T: Show {
  fn to_disasm_str<'a>(&self, _: &mut Disasm<'a>) -> DisasmStr {
    (format!("{}", *self)).into_cow()
  }
}

impl ToDisasmStr for Immediate8 {
  fn to_disasm_str<'a>(&self, disasm: &mut Disasm<'a>) -> DisasmStr {
    (format!("${:02x}", disasm.next_u8())).into_cow()
  }
}

impl ToDisasmStr for Addr {
  fn to_disasm_str<'a>(&self, disasm: &mut Disasm<'a>) -> DisasmStr {
    match *self {
      Addr::BC => "(BC)".into_cow(), Addr::DE => "(DE)".into_cow(),
      Addr::HL => "(HL)".into_cow(),
      Addr::HLD => "(HL-)".into_cow(), Addr::HLI => "(HL+)".into_cow(),
      Addr::ZeroPageC => "($FF00+C)".into_cow(),
      Addr::Direct => (format!("(${:04x})", disasm.next_u16())).into_cow(),
      Addr::ZeroPage => (format!("($FF00+${:02x})", disasm.next_u8())).into_cow()
    }
  }
}

impl ToDisasmStr for Immediate16 {
  fn to_disasm_str<'a>(&self, disasm: &mut Disasm<'a>) -> DisasmStr {
    (format!("${:04x}", disasm.next_u16())).into_cow()
  }
}
impl ToDisasmStr for Direct16 {
  fn to_disasm_str<'a>(&self, disasm: &mut Disasm<'a>) -> DisasmStr {
    (format!("(${:04x})", disasm.next_u16())).into_cow()
  }
}

struct PcOffset;

impl ToDisasmStr for PcOffset {
  fn to_disasm_str<'a>(&self, disasm: &mut Disasm<'a>) -> DisasmStr {
    let offset = disasm.next_u8() as i8;
    let addr = (disasm.pc as i16 + offset as i16) as u16;
    (format!("${:04x}", addr)).into_cow()
  }
}

impl<'a> Disasm<'a> {
  fn null_op(&mut self, op: &'static str) -> DisasmStr { op.into_cow() }
  fn unary_op<A: ToDisasmStr>(&mut self, op: &'static str, arg: A) -> DisasmStr {
    (format!("{} {}", op, arg.to_disasm_str(self))).into_cow()
  }
  fn binary_op<A: ToDisasmStr, B: ToDisasmStr>(&mut self, op: &'static str, arg1: A, arg2: B) -> DisasmStr {
    (format!("{} {}, {}", op, arg1.to_disasm_str(self), arg2.to_disasm_str(self))).into_cow()
  }
}

impl<'a> CpuOps<DisasmStr> for Disasm<'a> {
  fn next_u8(&mut self) -> u8 {
    let addr = self.pc;
    self.pc += 1;
    self.bus.read(addr)
  }
  // --- 8-bit operations
  // 8-bit loads
  fn load<O: Out8, I: In8>(&mut self, out8: O, in8: I) -> DisasmStr { self.binary_op("LD", out8, in8) }
  // 8-bit arithmetic
  fn add<I: In8>(&mut self, in8: I) -> DisasmStr { self.unary_op("ADD", in8) }
  fn adc<I: In8>(&mut self, in8: I) -> DisasmStr { self.unary_op("ADC", in8) }
  fn sub<I: In8>(&mut self, in8: I) -> DisasmStr { self.unary_op("SUB", in8) }
  fn sbc<I: In8>(&mut self, in8: I) -> DisasmStr { self.unary_op("SBC", in8) }
  fn  cp<I: In8>(&mut self, in8: I) -> DisasmStr { self.unary_op( "CP", in8) }
  fn and<I: In8>(&mut self, in8: I) -> DisasmStr { self.unary_op("AND", in8) }
  fn  or<I: In8>(&mut self, in8: I) -> DisasmStr { self.unary_op( "OR", in8) }
  fn xor<I: In8>(&mut self, in8: I) -> DisasmStr { self.unary_op("XOR", in8) }
  fn inc<IO: In8+Out8>(&mut self, io: IO) -> DisasmStr { self.unary_op("INC", io) }
  fn dec<IO: In8+Out8>(&mut self, io: IO) -> DisasmStr { self.unary_op("DEC", io) }
  fn rlca(&mut self) -> DisasmStr { self.null_op("RLCA") }
  fn  rla(&mut self) -> DisasmStr { self.null_op( "RLA") }
  fn rrca(&mut self) -> DisasmStr { self.null_op("RRCA") }
  fn  rra(&mut self) -> DisasmStr { self.null_op( "RRA") }
  fn  rlc<IO: In8+Out8>(&mut self, io: IO) -> DisasmStr { self.unary_op( "RLC", io) }
  fn   rl<IO: In8+Out8>(&mut self, io: IO) -> DisasmStr { self.unary_op(  "RL", io) }
  fn  rrc<IO: In8+Out8>(&mut self, io: IO) -> DisasmStr { self.unary_op( "RRC", io) }
  fn   rr<IO: In8+Out8>(&mut self, io: IO) -> DisasmStr { self.unary_op(  "RR", io) }
  fn  sla<IO: In8+Out8>(&mut self, io: IO) -> DisasmStr { self.unary_op( "SLA", io) }
  fn  sra<IO: In8+Out8>(&mut self, io: IO) -> DisasmStr { self.unary_op( "SRA", io) }
  fn  srl<IO: In8+Out8>(&mut self, io: IO) -> DisasmStr { self.unary_op( "SRL", io) }
  fn swap<IO: In8+Out8>(&mut self, io: IO) -> DisasmStr { self.unary_op("SWAP", io) }
  fn  bit<I:  In8>     (&mut self, bit: uint, in8: I) -> DisasmStr { self.binary_op("BIT", bit, in8) }
  fn  set<IO: In8+Out8>(&mut self, bit: uint, io: IO) -> DisasmStr { self.binary_op("SET", bit, io) }
  fn  res<IO: In8+Out8>(&mut self, bit: uint, io: IO) -> DisasmStr { self.binary_op("RES", bit, io) }
  // --- Control
  fn      jp(&mut self) -> DisasmStr { self.unary_op(  "JP", Immediate16) }
  fn   jp_hl(&mut self) -> DisasmStr { self.null_op("JP HL") }
  fn      jr(&mut self) -> DisasmStr { self.unary_op(  "JR", PcOffset) }
  fn    call(&mut self) -> DisasmStr { self.unary_op("CALL", Immediate16) }
  fn     ret(&mut self) -> DisasmStr { self.null_op(  "RET") }
  fn    reti(&mut self) -> DisasmStr { self.null_op( "RETI") }
  fn   jp_cc(&mut self, cond: Cond) -> DisasmStr { self.binary_op(  "JP", cond, Immediate16) }
  fn   jr_cc(&mut self, cond: Cond) -> DisasmStr { self.binary_op(  "JR", cond, PcOffset) }
  fn call_cc(&mut self, cond: Cond) -> DisasmStr { self.binary_op("CALL", cond, Immediate16) }
  fn  ret_cc(&mut self, cond: Cond) -> DisasmStr { self.unary_op(  "RET", cond) }
  fn     rst(&mut self, addr: u8) -> DisasmStr {
    (format!("RST, ${:02x}", addr)).into_cow()
  }
  // --- Miscellaneous
  fn halt(&mut self) -> DisasmStr { self.null_op("HALT") }
  fn stop(&mut self) -> DisasmStr { self.null_op("STOP") }
  fn   di(&mut self) -> DisasmStr { self.null_op(  "DI") }
  fn   ei(&mut self) -> DisasmStr { self.null_op(  "EI") }
  fn  ccf(&mut self) -> DisasmStr { self.null_op( "CCF") }
  fn  scf(&mut self) -> DisasmStr { self.null_op( "SCF") }
  fn  nop(&mut self) -> DisasmStr { self.null_op( "NOP") }
  fn  daa(&mut self) -> DisasmStr { self.null_op( "DAA") }
  fn  cpl(&mut self) -> DisasmStr { self.null_op( "CPL") }
  // --- 16-bit operations
  // 16-bit loads
  fn load16<O: Out16, I: In16>(&mut self, out8: O, in8: I) -> DisasmStr { self.binary_op("LD", out8, in8) }
  fn load16_sp_hl(&mut self) -> DisasmStr { self.null_op("LD SP, HL") }
  fn load16_hl_sp_e(&mut self) -> DisasmStr {
    let offset = self.next_u8() as i8;
    (format!("LD HL, SP{:+2x}", offset)).into_cow()
  }
  // 16-bit arithmetic
  fn push16(&mut self, reg: Reg16) -> DisasmStr { self.unary_op("PUSH", reg) }
  fn  pop16(&mut self, reg: Reg16) -> DisasmStr { self.unary_op( "POP", reg) }
  fn  add16(&mut self, reg: Reg16) -> DisasmStr { self.binary_op("ADD", "HL", reg) }
  fn add16_sp_e(&mut self) -> DisasmStr {
    let offset = self.next_u8() as i8;
    (format!("ADD SP, ${:02x}", offset)).into_cow()
  }
  fn inc16(&mut self, reg: Reg16) -> DisasmStr { self.unary_op("INC", reg) }
  fn dec16(&mut self, reg: Reg16) -> DisasmStr { self.unary_op("DEC", reg) }
  // --- Undefined
  fn undefined(&mut self, op: u8) -> DisasmStr {
    (format!("${:02x} ??", op)).into_cow()
  }
  fn undefined_debug(&mut self) -> DisasmStr { self.null_op("DBG") }
}

pub fn disasm<B: Bus>(bus: &B, pc_start: u16) -> DisasmStr {
  let mut disasm = Disasm {
    pc: pc_start,
    bus: bus
  };
  disasm.decode()
}
