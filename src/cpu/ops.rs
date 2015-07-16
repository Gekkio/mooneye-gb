use cpu::{
  In8, Out8,
  Cond,
  Immediate8, Addr
};
use cpu::registers::Reg8::{
  A, B, C, D, E, H, L
};
use cpu::registers::Reg16;
use cpu::registers::Reg16::{
  AF, BC, DE, HL, SP
};

pub trait CpuOps {
  type R;
  // --- 8-bit operations
  // 8-bit loads
  fn load<O: Out8, I: In8>(self, O, I) -> Self::R;
  // 8-bit arithmetic
  fn add<I: In8>(self, I) -> Self::R;
  fn adc<I: In8>(self, I) -> Self::R;
  fn sub<I: In8>(self, I) -> Self::R;
  fn sbc<I: In8>(self, I) -> Self::R;
  fn  cp<I: In8>(self, I) -> Self::R;
  fn and<I: In8>(self, I) -> Self::R;
  fn  or<I: In8>(self, I) -> Self::R;
  fn xor<I: In8>(self, I) -> Self::R;
  fn inc<IO: In8+Out8>(self, IO) -> Self::R;
  fn dec<IO: In8+Out8>(self, IO) -> Self::R;
  fn rlca(self) -> Self::R;
  fn  rla(self) -> Self::R;
  fn rrca(self) -> Self::R;
  fn  rra(self) -> Self::R;
  fn  rlc<IO: In8+Out8>(self, IO) -> Self::R;
  fn   rl<IO: In8+Out8>(self, IO) -> Self::R;
  fn  rrc<IO: In8+Out8>(self, IO) -> Self::R;
  fn   rr<IO: In8+Out8>(self, IO) -> Self::R;
  fn  sla<IO: In8+Out8>(self, IO) -> Self::R;
  fn  sra<IO: In8+Out8>(self, IO) -> Self::R;
  fn  srl<IO: In8+Out8>(self, IO) -> Self::R;
  fn swap<IO: In8+Out8>(self, IO) -> Self::R;
  fn  bit<I:  In8>     (self, usize, I) -> Self::R;
  fn  set<IO: In8+Out8>(self, usize, IO) -> Self::R;
  fn  res<IO: In8+Out8>(self, usize, IO) -> Self::R;
  // --- Control
  fn      jp(self) -> Self::R;
  fn   jp_hl(self) -> Self::R;
  fn      jr(self) -> Self::R;
  fn    call(self) -> Self::R;
  fn     ret(self) -> Self::R;
  fn    reti(self) -> Self::R;
  fn   jp_cc(self, Cond) -> Self::R;
  fn   jr_cc(self, Cond) -> Self::R;
  fn call_cc(self, Cond) -> Self::R;
  fn  ret_cc(self, Cond) -> Self::R;
  fn     rst(self, u8) -> Self::R;
  // --- Miscellaneous
  fn halt(self) -> Self::R;
  fn stop(self) -> Self::R;
  fn   di(self) -> Self::R;
  fn   ei(self) -> Self::R;
  fn  ccf(self) -> Self::R;
  fn  scf(self) -> Self::R;
  fn  nop(self) -> Self::R;
  fn  daa(self) -> Self::R;
  fn  cpl(self) -> Self::R;
  // --- 16-bit operations
  // 16-bit loads
  fn load16_imm(self, Reg16) -> Self::R;
  fn load16_nn_sp(self) -> Self::R;
  fn load16_sp_hl(self) -> Self::R;
  fn load16_hl_sp_e(self) -> Self::R;
  fn push16(self, Reg16) -> Self::R;
  fn  pop16(self, Reg16) -> Self::R;
  // 16-bit arithmetic
  fn add16(self, Reg16) -> Self::R;
  fn add16_sp_e(self) -> Self::R;
  fn inc16(self, Reg16) -> Self::R;
  fn dec16(self, Reg16) -> Self::R;
  // --- Other
  fn undefined(self, u8) -> Self::R;
  fn undefined_debug(self) -> Self::R;
  fn cb_prefix(self) -> Self::R;
}

pub fn decode<O: CpuOps>(ops: O, op: u8) -> O::R {
  match op {
    // --- 8-bit operations
    // 8-bit loads
    0x7f => ops.load(A, A), 0x78 => ops.load(A, B),
    0x79 => ops.load(A, C), 0x7a => ops.load(A, D),
    0x7b => ops.load(A, E), 0x7c => ops.load(A, H),
    0x7d => ops.load(A, L), 0x7e => ops.load(A, Addr::HL),
    0x47 => ops.load(B, A), 0x40 => ops.load(B, B),
    0x41 => ops.load(B, C), 0x42 => ops.load(B, D),
    0x43 => ops.load(B, E), 0x44 => ops.load(B, H),
    0x45 => ops.load(B, L), 0x46 => ops.load(B, Addr::HL),
    0x4f => ops.load(C, A), 0x48 => ops.load(C, B),
    0x49 => ops.load(C, C), 0x4a => ops.load(C, D),
    0x4b => ops.load(C, E), 0x4c => ops.load(C, H),
    0x4d => ops.load(C, L), 0x4e => ops.load(C, Addr::HL),
    0x57 => ops.load(D, A), 0x50 => ops.load(D, B),
    0x51 => ops.load(D, C), 0x52 => ops.load(D, D),
    0x53 => ops.load(D, E), 0x54 => ops.load(D, H),
    0x55 => ops.load(D, L), 0x56 => ops.load(D, Addr::HL),
    0x5f => ops.load(E, A), 0x58 => ops.load(E, B),
    0x59 => ops.load(E, C), 0x5a => ops.load(E, D),
    0x5b => ops.load(E, E), 0x5c => ops.load(E, H),
    0x5d => ops.load(E, L), 0x5e => ops.load(E, Addr::HL),
    0x67 => ops.load(H, A), 0x60 => ops.load(H, B),
    0x61 => ops.load(H, C), 0x62 => ops.load(H, D),
    0x63 => ops.load(H, E), 0x64 => ops.load(H, H),
    0x65 => ops.load(H, L), 0x66 => ops.load(H, Addr::HL),
    0x6f => ops.load(L, A), 0x68 => ops.load(L, B),
    0x69 => ops.load(L, C), 0x6a => ops.load(L, D),
    0x6b => ops.load(L, E), 0x6c => ops.load(L, H),
    0x6d => ops.load(L, L), 0x6e => ops.load(L, Addr::HL),
    0x3e => ops.load(A, Immediate8), 0x06 => ops.load(B, Immediate8),
    0x0e => ops.load(C, Immediate8), 0x16 => ops.load(D, Immediate8),
    0x1e => ops.load(E, Immediate8), 0x26 => ops.load(H, Immediate8),
    0x2e => ops.load(L, Immediate8), 0x36 => ops.load(Addr::HL, Immediate8),
    0x77 => ops.load(Addr::HL, A), 0x70 => ops.load(Addr::HL, B),
    0x71 => ops.load(Addr::HL, C), 0x72 => ops.load(Addr::HL, D),
    0x73 => ops.load(Addr::HL, E), 0x74 => ops.load(Addr::HL, H),
    0x75 => ops.load(Addr::HL, L),
    0x0a => ops.load(A, Addr::BC       ), 0x02 => ops.load(Addr::BC,        A),
    0x1a => ops.load(A, Addr::DE       ), 0x12 => ops.load(Addr::DE,        A),
    0xfa => ops.load(A, Addr::Direct   ), 0xea => ops.load(Addr::Direct,    A),
    0x3a => ops.load(A, Addr::HLD      ), 0x32 => ops.load(Addr::HLD,       A),
    0x2a => ops.load(A, Addr::HLI      ), 0x22 => ops.load(Addr::HLI,       A),
    0xf2 => ops.load(A, Addr::ZeroPageC), 0xe2 => ops.load(Addr::ZeroPageC, A),
    0xf0 => ops.load(A, Addr::ZeroPage ), 0xe0 => ops.load(Addr::ZeroPage,  A),
    // 8-bit arithmetic
    0x87 => ops.add(A), 0x80 => ops.add(B),
    0x81 => ops.add(C), 0x82 => ops.add(D),
    0x83 => ops.add(E), 0x84 => ops.add(H),
    0x85 => ops.add(L), 0x86 => ops.add(Addr::HL),
    0xc6 => ops.add(Immediate8),
    0x8f => ops.adc(A), 0x88 => ops.adc(B),
    0x89 => ops.adc(C), 0x8a => ops.adc(D),
    0x8b => ops.adc(E), 0x8c => ops.adc(H),
    0x8d => ops.adc(L), 0x8e => ops.adc(Addr::HL),
    0xce => ops.adc(Immediate8),
    0x97 => ops.sub(A), 0x90 => ops.sub(B),
    0x91 => ops.sub(C), 0x92 => ops.sub(D),
    0x93 => ops.sub(E), 0x94 => ops.sub(H),
    0x95 => ops.sub(L), 0x96 => ops.sub(Addr::HL),
    0xd6 => ops.sub(Immediate8),
    0x9f => ops.sbc(A), 0x98 => ops.sbc(B),
    0x99 => ops.sbc(C), 0x9a => ops.sbc(D),
    0x9b => ops.sbc(E), 0x9c => ops.sbc(H),
    0x9d => ops.sbc(L), 0x9e => ops.sbc(Addr::HL),
    0xde => ops.sbc(Immediate8),
    0xbf => ops.cp(A), 0xb8 => ops.cp(B),
    0xb9 => ops.cp(C), 0xba => ops.cp(D),
    0xbb => ops.cp(E), 0xbc => ops.cp(H),
    0xbd => ops.cp(L), 0xbe => ops.cp(Addr::HL),
    0xfe => ops.cp(Immediate8),
    0xa7 => ops.and(A), 0xa0 => ops.and(B),
    0xa1 => ops.and(C), 0xa2 => ops.and(D),
    0xa3 => ops.and(E), 0xa4 => ops.and(H),
    0xa5 => ops.and(L), 0xa6 => ops.and(Addr::HL),
    0xe6 => ops.and(Immediate8),
    0xb7 => ops.or(A), 0xb0 => ops.or(B),
    0xb1 => ops.or(C), 0xb2 => ops.or(D),
    0xb3 => ops.or(E), 0xb4 => ops.or(H),
    0xb5 => ops.or(L), 0xb6 => ops.or(Addr::HL),
    0xf6 => ops.or(Immediate8),
    0xaf => ops.xor(A), 0xa8 => ops.xor(B),
    0xa9 => ops.xor(C), 0xaa => ops.xor(D),
    0xab => ops.xor(E), 0xac => ops.xor(H),
    0xad => ops.xor(L), 0xae => ops.xor(Addr::HL),
    0xee => ops.xor(Immediate8),
    0x3c => ops.inc(A), 0x04 => ops.inc(B),
    0x0c => ops.inc(C), 0x14 => ops.inc(D),
    0x1c => ops.inc(E), 0x24 => ops.inc(H),
    0x2c => ops.inc(L), 0x34 => ops.inc(Addr::HL),
    0x3d => ops.dec(A), 0x05 => ops.dec(B),
    0x0d => ops.dec(C), 0x15 => ops.dec(D),
    0x1d => ops.dec(E), 0x25 => ops.dec(H),
    0x2d => ops.dec(L), 0x35 => ops.dec(Addr::HL),
    0x07 => ops.rlca(), 0x17 => ops.rla(),
    0x0f => ops.rrca(), 0x1f => ops.rra(),
    // --- Control
    0xc3 => ops.   jp(),
    0xe9 => ops.jp_hl(),
    0x18 => ops.   jr(),
    0xcd => ops. call(),
    0xc9 => ops.  ret(),
    0xd9 => ops. reti(),
    0xc2 => ops.  jp_cc(Cond::NZ), 0xca => ops.  jp_cc(Cond::Z),
    0xd2 => ops.  jp_cc(Cond::NC), 0xda => ops.  jp_cc(Cond::C),
    0x20 => ops.  jr_cc(Cond::NZ), 0x28 => ops.  jr_cc(Cond::Z),
    0x30 => ops.  jr_cc(Cond::NC), 0x38 => ops.  jr_cc(Cond::C),
    0xc4 => ops.call_cc(Cond::NZ), 0xcc => ops.call_cc(Cond::Z),
    0xd4 => ops.call_cc(Cond::NC), 0xdc => ops.call_cc(Cond::C),
    0xc0 => ops. ret_cc(Cond::NZ), 0xc8 => ops. ret_cc(Cond::Z),
    0xd0 => ops. ret_cc(Cond::NC), 0xd8 => ops. ret_cc(Cond::C),
    0xc7 => ops.rst(0x00), 0xcf => ops.rst(0x08),
    0xd7 => ops.rst(0x10), 0xdf => ops.rst(0x18),
    0xe7 => ops.rst(0x20), 0xef => ops.rst(0x28),
    0xf7 => ops.rst(0x30), 0xff => ops.rst(0x38),
    // --- Miscellaneous
    0x76 => ops.halt(), 0x10 => ops.stop(),
    0xf3 => ops.  di(), 0xfb => ops.  ei(),
    0x3f => ops. ccf(), 0x37 => ops. scf(),
    0x00 => ops. nop(),
    0x27 => ops. daa(),
    0x2f => ops. cpl(),
    // --- 16-bit operations
    // 16-bit loads
    0x01 => ops.load16_imm(BC), 0x11 => ops.load16_imm(DE),
    0x21 => ops.load16_imm(HL), 0x31 => ops.load16_imm(SP),
    0x08 => ops.load16_nn_sp(),
    0xf9 => ops.load16_sp_hl(),
    0xf8 => ops.load16_hl_sp_e(),
    0xc5 => ops.push16(BC), 0xd5 => ops.push16(DE),
    0xe5 => ops.push16(HL), 0xf5 => ops.push16(AF),
    0xc1 => ops. pop16(BC), 0xd1 => ops. pop16(DE),
    0xe1 => ops. pop16(HL), 0xf1 => ops. pop16(AF),
    // 16-bit arithmetic
    0x09 => ops.add16(BC), 0x19 => ops.add16(DE),
    0x29 => ops.add16(HL), 0x39 => ops.add16(SP),
    0xe8 => ops.add16_sp_e(),
    0x03 => ops.inc16(BC), 0x13 => ops.inc16(DE),
    0x23 => ops.inc16(HL), 0x33 => ops.inc16(SP),
    0x0b => ops.dec16(BC), 0x1b => ops.dec16(DE),
    0x2b => ops.dec16(HL), 0x3b => ops.dec16(SP),
    0xcb => ops.cb_prefix(),
    0xed => ops.undefined_debug(),
    0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 |
    0xeb | 0xec | 0xf4 | 0xfc | 0xfd => ops.undefined(op),
    _ => unreachable!("Unknown opcode 0x{:0x}", op)
  }
}

pub fn decode_cb<O: CpuOps>(ops: O, op: u8) -> O::R {
  match op {
    // --- 8-bit operations
    // 8-bit arithmetic
    0x07 => ops. rlc(A), 0x00 => ops. rlc(B),
    0x01 => ops. rlc(C), 0x02 => ops. rlc(D),
    0x03 => ops. rlc(E), 0x04 => ops. rlc(H),
    0x05 => ops. rlc(L), 0x06 => ops. rlc(Addr::HL),
    0x17 => ops.  rl(A), 0x10 => ops.  rl(B),
    0x11 => ops.  rl(C), 0x12 => ops.  rl(D),
    0x13 => ops.  rl(E), 0x14 => ops.  rl(H),
    0x15 => ops.  rl(L), 0x16 => ops.  rl(Addr::HL),
    0x0f => ops. rrc(A), 0x08 => ops. rrc(B),
    0x09 => ops. rrc(C), 0x0a => ops. rrc(D),
    0x0b => ops. rrc(E), 0x0c => ops. rrc(H),
    0x0d => ops. rrc(L), 0x0e => ops. rrc(Addr::HL),
    0x1f => ops.  rr(A), 0x18 => ops.  rr(B),
    0x19 => ops.  rr(C), 0x1a => ops.  rr(D),
    0x1b => ops.  rr(E), 0x1c => ops.  rr(H),
    0x1d => ops.  rr(L), 0x1e => ops.  rr(Addr::HL),
    0x27 => ops. sla(A), 0x20 => ops. sla(B),
    0x21 => ops. sla(C), 0x22 => ops. sla(D),
    0x23 => ops. sla(E), 0x24 => ops. sla(H),
    0x25 => ops. sla(L), 0x26 => ops. sla(Addr::HL),
    0x2f => ops. sra(A), 0x28 => ops. sra(B),
    0x29 => ops. sra(C), 0x2a => ops. sra(D),
    0x2b => ops. sra(E), 0x2c => ops. sra(H),
    0x2d => ops. sra(L), 0x2e => ops. sra(Addr::HL),
    0x3f => ops. srl(A), 0x38 => ops. srl(B),
    0x39 => ops. srl(C), 0x3a => ops. srl(D),
    0x3b => ops. srl(E), 0x3c => ops. srl(H),
    0x3d => ops. srl(L), 0x3e => ops. srl(Addr::HL),
    0x37 => ops.swap(A), 0x30 => ops.swap(B),
    0x31 => ops.swap(C), 0x32 => ops.swap(D),
    0x33 => ops.swap(E), 0x34 => ops.swap(H),
    0x35 => ops.swap(L), 0x36 => ops.swap(Addr::HL),
    0x47 => ops.bit(0, A), 0x4f => ops.bit(1, A),
    0x57 => ops.bit(2, A), 0x5f => ops.bit(3, A),
    0x67 => ops.bit(4, A), 0x6f => ops.bit(5, A),
    0x77 => ops.bit(6, A), 0x7f => ops.bit(7, A),
    0x40 => ops.bit(0, B), 0x48 => ops.bit(1, B),
    0x50 => ops.bit(2, B), 0x58 => ops.bit(3, B),
    0x60 => ops.bit(4, B), 0x68 => ops.bit(5, B),
    0x70 => ops.bit(6, B), 0x78 => ops.bit(7, B),
    0x41 => ops.bit(0, C), 0x49 => ops.bit(1, C),
    0x51 => ops.bit(2, C), 0x59 => ops.bit(3, C),
    0x61 => ops.bit(4, C), 0x69 => ops.bit(5, C),
    0x71 => ops.bit(6, C), 0x79 => ops.bit(7, C),
    0x42 => ops.bit(0, D), 0x4a => ops.bit(1, D),
    0x52 => ops.bit(2, D), 0x5a => ops.bit(3, D),
    0x62 => ops.bit(4, D), 0x6a => ops.bit(5, D),
    0x72 => ops.bit(6, D), 0x7a => ops.bit(7, D),
    0x43 => ops.bit(0, E), 0x4b => ops.bit(1, E),
    0x53 => ops.bit(2, E), 0x5b => ops.bit(3, E),
    0x63 => ops.bit(4, E), 0x6b => ops.bit(5, E),
    0x73 => ops.bit(6, E), 0x7b => ops.bit(7, E),
    0x44 => ops.bit(0, H), 0x4c => ops.bit(1, H),
    0x54 => ops.bit(2, H), 0x5c => ops.bit(3, H),
    0x64 => ops.bit(4, H), 0x6c => ops.bit(5, H),
    0x74 => ops.bit(6, H), 0x7c => ops.bit(7, H),
    0x45 => ops.bit(0, L), 0x4d => ops.bit(1, L),
    0x55 => ops.bit(2, L), 0x5d => ops.bit(3, L),
    0x65 => ops.bit(4, L), 0x6d => ops.bit(5, L),
    0x75 => ops.bit(6, L), 0x7d => ops.bit(7, L),
    0x46 => ops.bit(0, Addr::HL), 0x4e => ops.bit(1, Addr::HL),
    0x56 => ops.bit(2, Addr::HL), 0x5e => ops.bit(3, Addr::HL),
    0x66 => ops.bit(4, Addr::HL), 0x6e => ops.bit(5, Addr::HL),
    0x76 => ops.bit(6, Addr::HL), 0x7e => ops.bit(7, Addr::HL),
    0xc7 => ops.set(0, A), 0xcf => ops.set(1, A),
    0xd7 => ops.set(2, A), 0xdf => ops.set(3, A),
    0xe7 => ops.set(4, A), 0xef => ops.set(5, A),
    0xf7 => ops.set(6, A), 0xff => ops.set(7, A),
    0xc0 => ops.set(0, B), 0xc8 => ops.set(1, B),
    0xd0 => ops.set(2, B), 0xd8 => ops.set(3, B),
    0xe0 => ops.set(4, B), 0xe8 => ops.set(5, B),
    0xf0 => ops.set(6, B), 0xf8 => ops.set(7, B),
    0xc1 => ops.set(0, C), 0xc9 => ops.set(1, C),
    0xd1 => ops.set(2, C), 0xd9 => ops.set(3, C),
    0xe1 => ops.set(4, C), 0xe9 => ops.set(5, C),
    0xf1 => ops.set(6, C), 0xf9 => ops.set(7, C),
    0xc2 => ops.set(0, D), 0xca => ops.set(1, D),
    0xd2 => ops.set(2, D), 0xda => ops.set(3, D),
    0xe2 => ops.set(4, D), 0xea => ops.set(5, D),
    0xf2 => ops.set(6, D), 0xfa => ops.set(7, D),
    0xc3 => ops.set(0, E), 0xcb => ops.set(1, E),
    0xd3 => ops.set(2, E), 0xdb => ops.set(3, E),
    0xe3 => ops.set(4, E), 0xeb => ops.set(5, E),
    0xf3 => ops.set(6, E), 0xfb => ops.set(7, E),
    0xc4 => ops.set(0, H), 0xcc => ops.set(1, H),
    0xd4 => ops.set(2, H), 0xdc => ops.set(3, H),
    0xe4 => ops.set(4, H), 0xec => ops.set(5, H),
    0xf4 => ops.set(6, H), 0xfc => ops.set(7, H),
    0xc5 => ops.set(0, L), 0xcd => ops.set(1, L),
    0xd5 => ops.set(2, L), 0xdd => ops.set(3, L),
    0xe5 => ops.set(4, L), 0xed => ops.set(5, L),
    0xf5 => ops.set(6, L), 0xfd => ops.set(7, L),
    0xc6 => ops.set(0, Addr::HL), 0xce => ops.set(1, Addr::HL),
    0xd6 => ops.set(2, Addr::HL), 0xde => ops.set(3, Addr::HL),
    0xe6 => ops.set(4, Addr::HL), 0xee => ops.set(5, Addr::HL),
    0xf6 => ops.set(6, Addr::HL), 0xfe => ops.set(7, Addr::HL),
    0x87 => ops.res(0, A), 0x8f => ops.res(1, A),
    0x97 => ops.res(2, A), 0x9f => ops.res(3, A),
    0xa7 => ops.res(4, A), 0xaf => ops.res(5, A),
    0xb7 => ops.res(6, A), 0xbf => ops.res(7, A),
    0x80 => ops.res(0, B), 0x88 => ops.res(1, B),
    0x90 => ops.res(2, B), 0x98 => ops.res(3, B),
    0xa0 => ops.res(4, B), 0xa8 => ops.res(5, B),
    0xb0 => ops.res(6, B), 0xb8 => ops.res(7, B),
    0x81 => ops.res(0, C), 0x89 => ops.res(1, C),
    0x91 => ops.res(2, C), 0x99 => ops.res(3, C),
    0xa1 => ops.res(4, C), 0xa9 => ops.res(5, C),
    0xb1 => ops.res(6, C), 0xb9 => ops.res(7, C),
    0x82 => ops.res(0, D), 0x8a => ops.res(1, D),
    0x92 => ops.res(2, D), 0x9a => ops.res(3, D),
    0xa2 => ops.res(4, D), 0xaa => ops.res(5, D),
    0xb2 => ops.res(6, D), 0xba => ops.res(7, D),
    0x83 => ops.res(0, E), 0x8b => ops.res(1, E),
    0x93 => ops.res(2, E), 0x9b => ops.res(3, E),
    0xa3 => ops.res(4, E), 0xab => ops.res(5, E),
    0xb3 => ops.res(6, E), 0xbb => ops.res(7, E),
    0x84 => ops.res(0, H), 0x8c => ops.res(1, H),
    0x94 => ops.res(2, H), 0x9c => ops.res(3, H),
    0xa4 => ops.res(4, H), 0xac => ops.res(5, H),
    0xb4 => ops.res(6, H), 0xbc => ops.res(7, H),
    0x85 => ops.res(0, L), 0x8d => ops.res(1, L),
    0x95 => ops.res(2, L), 0x9d => ops.res(3, L),
    0xa5 => ops.res(4, L), 0xad => ops.res(5, L),
    0xb5 => ops.res(6, L), 0xbd => ops.res(7, L),
    0x86 => ops.res(0, Addr::HL), 0x8e => ops.res(1, Addr::HL),
    0x96 => ops.res(2, Addr::HL), 0x9e => ops.res(3, Addr::HL),
    0xa6 => ops.res(4, Addr::HL), 0xae => ops.res(5, Addr::HL),
    0xb6 => ops.res(6, Addr::HL), 0xbe => ops.res(7, Addr::HL),
    _ => unreachable!("Unknown opcode 0xcb 0x{:0x}", op)
  }
}
