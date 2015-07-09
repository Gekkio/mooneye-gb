use cpu::{
  In8, Out8,
  In16, Out16,
  Cond,
  Immediate8, Addr,
  Immediate16, Direct16
};
use cpu::registers::Reg8::{
  A, B, C, D, E, H, L
};
use cpu::registers::Reg16;
use cpu::registers::Reg16::{
  AF, BC, DE, HL, SP
};

pub trait CpuOps<R> {
  fn next_u8(&mut self) -> u8;
  fn next_u16(&mut self) -> u16 {
    let l = self.next_u8();
    let h = self.next_u8();
    ((h as u16) << 8) | (l as u16)
  }
  // --- 8-bit operations
  // 8-bit loads
  fn load<O: Out8, I: In8>(&mut self, O, I) -> R;
  // 8-bit arithmetic
  fn add<I: In8>(&mut self, I) -> R;
  fn adc<I: In8>(&mut self, I) -> R;
  fn sub<I: In8>(&mut self, I) -> R;
  fn sbc<I: In8>(&mut self, I) -> R;
  fn  cp<I: In8>(&mut self, I) -> R;
  fn and<I: In8>(&mut self, I) -> R;
  fn  or<I: In8>(&mut self, I) -> R;
  fn xor<I: In8>(&mut self, I) -> R;
  fn inc<IO: In8+Out8>(&mut self, IO) -> R;
  fn dec<IO: In8+Out8>(&mut self, IO) -> R;
  fn rlca(&mut self) -> R;
  fn  rla(&mut self) -> R;
  fn rrca(&mut self) -> R;
  fn  rra(&mut self) -> R;
  fn  rlc<IO: In8+Out8>(&mut self, IO) -> R;
  fn   rl<IO: In8+Out8>(&mut self, IO) -> R;
  fn  rrc<IO: In8+Out8>(&mut self, IO) -> R;
  fn   rr<IO: In8+Out8>(&mut self, IO) -> R;
  fn  sla<IO: In8+Out8>(&mut self, IO) -> R;
  fn  sra<IO: In8+Out8>(&mut self, IO) -> R;
  fn  srl<IO: In8+Out8>(&mut self, IO) -> R;
  fn swap<IO: In8+Out8>(&mut self, IO) -> R;
  fn  bit<I:  In8>     (&mut self, usize, I) -> R;
  fn  set<IO: In8+Out8>(&mut self, usize, IO) -> R;
  fn  res<IO: In8+Out8>(&mut self, usize, IO) -> R;
  // --- Control
  fn      jp(&mut self) -> R;
  fn   jp_hl(&mut self) -> R;
  fn      jr(&mut self) -> R;
  fn    call(&mut self) -> R;
  fn     ret(&mut self) -> R;
  fn    reti(&mut self) -> R;
  fn   jp_cc(&mut self, Cond) -> R;
  fn   jr_cc(&mut self, Cond) -> R;
  fn call_cc(&mut self, Cond) -> R;
  fn  ret_cc(&mut self, Cond) -> R;
  fn     rst(&mut self, u8) -> R;
  // --- Miscellaneous
  fn halt(&mut self) -> R;
  fn stop(&mut self) -> R;
  fn   di(&mut self) -> R;
  fn   ei(&mut self) -> R;
  fn  ccf(&mut self) -> R;
  fn  scf(&mut self) -> R;
  fn  nop(&mut self) -> R;
  fn  daa(&mut self) -> R;
  fn  cpl(&mut self) -> R;
  // --- 16-bit operations
  // 16-bit loads
  fn load16<O: Out16, I: In16>(&mut self, O, I) -> R;
  fn load16_sp_hl(&mut self) -> R;
  fn load16_hl_sp_e(&mut self) -> R;
  fn push16(&mut self, Reg16) -> R;
  fn  pop16(&mut self, Reg16) -> R;
  // 16-bit arithmetic
  fn add16(&mut self, Reg16) -> R;
  fn add16_sp_e(&mut self) -> R;
  fn inc16(&mut self, Reg16) -> R;
  fn dec16(&mut self, Reg16) -> R;
  // --- Undefined
  fn undefined(&mut self, u8) -> R;
  fn undefined_debug(&mut self) -> R;

  fn decode(&mut self, op: u8) -> R {
    match op {
      // --- 8-bit operations
      // 8-bit loads
      0x7f => self.load(A, A), 0x78 => self.load(A, B),
      0x79 => self.load(A, C), 0x7a => self.load(A, D),
      0x7b => self.load(A, E), 0x7c => self.load(A, H),
      0x7d => self.load(A, L), 0x7e => self.load(A, Addr::HL),
      0x47 => self.load(B, A), 0x40 => self.load(B, B),
      0x41 => self.load(B, C), 0x42 => self.load(B, D),
      0x43 => self.load(B, E), 0x44 => self.load(B, H),
      0x45 => self.load(B, L), 0x46 => self.load(B, Addr::HL),
      0x4f => self.load(C, A), 0x48 => self.load(C, B),
      0x49 => self.load(C, C), 0x4a => self.load(C, D),
      0x4b => self.load(C, E), 0x4c => self.load(C, H),
      0x4d => self.load(C, L), 0x4e => self.load(C, Addr::HL),
      0x57 => self.load(D, A), 0x50 => self.load(D, B),
      0x51 => self.load(D, C), 0x52 => self.load(D, D),
      0x53 => self.load(D, E), 0x54 => self.load(D, H),
      0x55 => self.load(D, L), 0x56 => self.load(D, Addr::HL),
      0x5f => self.load(E, A), 0x58 => self.load(E, B),
      0x59 => self.load(E, C), 0x5a => self.load(E, D),
      0x5b => self.load(E, E), 0x5c => self.load(E, H),
      0x5d => self.load(E, L), 0x5e => self.load(E, Addr::HL),
      0x67 => self.load(H, A), 0x60 => self.load(H, B),
      0x61 => self.load(H, C), 0x62 => self.load(H, D),
      0x63 => self.load(H, E), 0x64 => self.load(H, H),
      0x65 => self.load(H, L), 0x66 => self.load(H, Addr::HL),
      0x6f => self.load(L, A), 0x68 => self.load(L, B),
      0x69 => self.load(L, C), 0x6a => self.load(L, D),
      0x6b => self.load(L, E), 0x6c => self.load(L, H),
      0x6d => self.load(L, L), 0x6e => self.load(L, Addr::HL),
      0x3e => self.load(A, Immediate8), 0x06 => self.load(B, Immediate8),
      0x0e => self.load(C, Immediate8), 0x16 => self.load(D, Immediate8),
      0x1e => self.load(E, Immediate8), 0x26 => self.load(H, Immediate8),
      0x2e => self.load(L, Immediate8), 0x36 => self.load(Addr::HL, Immediate8),
      0x77 => self.load(Addr::HL, A), 0x70 => self.load(Addr::HL, B),
      0x71 => self.load(Addr::HL, C), 0x72 => self.load(Addr::HL, D),
      0x73 => self.load(Addr::HL, E), 0x74 => self.load(Addr::HL, H),
      0x75 => self.load(Addr::HL, L),
      0x0a => self.load(A, Addr::BC       ), 0x02 => self.load(Addr::BC,        A),
      0x1a => self.load(A, Addr::DE       ), 0x12 => self.load(Addr::DE,        A),
      0xfa => self.load(A, Addr::Direct   ), 0xea => self.load(Addr::Direct,    A),
      0x3a => self.load(A, Addr::HLD      ), 0x32 => self.load(Addr::HLD,       A),
      0x2a => self.load(A, Addr::HLI      ), 0x22 => self.load(Addr::HLI,       A),
      0xf2 => self.load(A, Addr::ZeroPageC), 0xe2 => self.load(Addr::ZeroPageC, A),
      0xf0 => self.load(A, Addr::ZeroPage ), 0xe0 => self.load(Addr::ZeroPage,  A),
      // 8-bit arithmetic
      0x87 => self.add(A), 0x80 => self.add(B),
      0x81 => self.add(C), 0x82 => self.add(D),
      0x83 => self.add(E), 0x84 => self.add(H),
      0x85 => self.add(L), 0x86 => self.add(Addr::HL),
      0xc6 => self.add(Immediate8),
      0x8f => self.adc(A), 0x88 => self.adc(B),
      0x89 => self.adc(C), 0x8a => self.adc(D),
      0x8b => self.adc(E), 0x8c => self.adc(H),
      0x8d => self.adc(L), 0x8e => self.adc(Addr::HL),
      0xce => self.adc(Immediate8),
      0x97 => self.sub(A), 0x90 => self.sub(B),
      0x91 => self.sub(C), 0x92 => self.sub(D),
      0x93 => self.sub(E), 0x94 => self.sub(H),
      0x95 => self.sub(L), 0x96 => self.sub(Addr::HL),
      0xd6 => self.sub(Immediate8),
      0x9f => self.sbc(A), 0x98 => self.sbc(B),
      0x99 => self.sbc(C), 0x9a => self.sbc(D),
      0x9b => self.sbc(E), 0x9c => self.sbc(H),
      0x9d => self.sbc(L), 0x9e => self.sbc(Addr::HL),
      0xde => self.sbc(Immediate8),
      0xbf => self.cp(A), 0xb8 => self.cp(B),
      0xb9 => self.cp(C), 0xba => self.cp(D),
      0xbb => self.cp(E), 0xbc => self.cp(H),
      0xbd => self.cp(L), 0xbe => self.cp(Addr::HL),
      0xfe => self.cp(Immediate8),
      0xa7 => self.and(A), 0xa0 => self.and(B),
      0xa1 => self.and(C), 0xa2 => self.and(D),
      0xa3 => self.and(E), 0xa4 => self.and(H),
      0xa5 => self.and(L), 0xa6 => self.and(Addr::HL),
      0xe6 => self.and(Immediate8),
      0xb7 => self.or(A), 0xb0 => self.or(B),
      0xb1 => self.or(C), 0xb2 => self.or(D),
      0xb3 => self.or(E), 0xb4 => self.or(H),
      0xb5 => self.or(L), 0xb6 => self.or(Addr::HL),
      0xf6 => self.or(Immediate8),
      0xaf => self.xor(A), 0xa8 => self.xor(B),
      0xa9 => self.xor(C), 0xaa => self.xor(D),
      0xab => self.xor(E), 0xac => self.xor(H),
      0xad => self.xor(L), 0xae => self.xor(Addr::HL),
      0xee => self.xor(Immediate8),
      0x3c => self.inc(A), 0x04 => self.inc(B),
      0x0c => self.inc(C), 0x14 => self.inc(D),
      0x1c => self.inc(E), 0x24 => self.inc(H),
      0x2c => self.inc(L), 0x34 => self.inc(Addr::HL),
      0x3d => self.dec(A), 0x05 => self.dec(B),
      0x0d => self.dec(C), 0x15 => self.dec(D),
      0x1d => self.dec(E), 0x25 => self.dec(H),
      0x2d => self.dec(L), 0x35 => self.dec(Addr::HL),
      0x07 => self.rlca(), 0x17 => self.rla(),
      0x0f => self.rrca(), 0x1f => self.rra(),
      // --- Control
      0xc3 => self.   jp(),
      0xe9 => self.jp_hl(),
      0x18 => self.   jr(),
      0xcd => self. call(),
      0xc9 => self.  ret(),
      0xd9 => self. reti(),
      0xc2 => self.  jp_cc(Cond::NZ), 0xca => self.  jp_cc(Cond::Z),
      0xd2 => self.  jp_cc(Cond::NC), 0xda => self.  jp_cc(Cond::C),
      0x20 => self.  jr_cc(Cond::NZ), 0x28 => self.  jr_cc(Cond::Z),
      0x30 => self.  jr_cc(Cond::NC), 0x38 => self.  jr_cc(Cond::C),
      0xc4 => self.call_cc(Cond::NZ), 0xcc => self.call_cc(Cond::Z),
      0xd4 => self.call_cc(Cond::NC), 0xdc => self.call_cc(Cond::C),
      0xc0 => self. ret_cc(Cond::NZ), 0xc8 => self. ret_cc(Cond::Z),
      0xd0 => self. ret_cc(Cond::NC), 0xd8 => self. ret_cc(Cond::C),
      0xc7 => self.rst(0x00), 0xcf => self.rst(0x08),
      0xd7 => self.rst(0x10), 0xdf => self.rst(0x18),
      0xe7 => self.rst(0x20), 0xef => self.rst(0x28),
      0xf7 => self.rst(0x30), 0xff => self.rst(0x38),
      // --- Miscellaneous
      0x76 => self.halt(), 0x10 => self.stop(),
      0xf3 => self.  di(), 0xfb => self.  ei(),
      0x3f => self. ccf(), 0x37 => self. scf(),
      0x00 => self. nop(),
      0x27 => self. daa(),
      0x2f => self. cpl(),
      // --- 16-bit operations
      // 16-bit loads
      0x01 => self.load16(BC, Immediate16), 0x11 => self.load16(DE, Immediate16),
      0x21 => self.load16(HL, Immediate16), 0x31 => self.load16(SP, Immediate16),
      0x08 => self.load16(Direct16, SP),
      0xf9 => self.load16_sp_hl(),
      0xf8 => self.load16_hl_sp_e(),
      0xc5 => self.push16(BC), 0xd5 => self.push16(DE),
      0xe5 => self.push16(HL), 0xf5 => self.push16(AF),
      0xc1 => self. pop16(BC), 0xd1 => self. pop16(DE),
      0xe1 => self. pop16(HL), 0xf1 => self. pop16(AF),
      // 16-bit arithmetic
      0x09 => self.add16(BC), 0x19 => self.add16(DE),
      0x29 => self.add16(HL), 0x39 => self.add16(SP),
      0xe8 => self.add16_sp_e(),
      0x03 => self.inc16(BC), 0x13 => self.inc16(DE),
      0x23 => self.inc16(HL), 0x33 => self.inc16(SP),
      0x0b => self.dec16(BC), 0x1b => self.dec16(DE),
      0x2b => self.dec16(HL), 0x3b => self.dec16(SP),
      0xcb => {
        let op = self.next_u8();
        match op {
          // --- 8-bit operations
          // 8-bit arithmetic
          0x07 => self. rlc(A), 0x00 => self. rlc(B),
          0x01 => self. rlc(C), 0x02 => self. rlc(D),
          0x03 => self. rlc(E), 0x04 => self. rlc(H),
          0x05 => self. rlc(L), 0x06 => self. rlc(Addr::HL),
          0x17 => self.  rl(A), 0x10 => self.  rl(B),
          0x11 => self.  rl(C), 0x12 => self.  rl(D),
          0x13 => self.  rl(E), 0x14 => self.  rl(H),
          0x15 => self.  rl(L), 0x16 => self.  rl(Addr::HL),
          0x0f => self. rrc(A), 0x08 => self. rrc(B),
          0x09 => self. rrc(C), 0x0a => self. rrc(D),
          0x0b => self. rrc(E), 0x0c => self. rrc(H),
          0x0d => self. rrc(L), 0x0e => self. rrc(Addr::HL),
          0x1f => self.  rr(A), 0x18 => self.  rr(B),
          0x19 => self.  rr(C), 0x1a => self.  rr(D),
          0x1b => self.  rr(E), 0x1c => self.  rr(H),
          0x1d => self.  rr(L), 0x1e => self.  rr(Addr::HL),
          0x27 => self. sla(A), 0x20 => self. sla(B),
          0x21 => self. sla(C), 0x22 => self. sla(D),
          0x23 => self. sla(E), 0x24 => self. sla(H),
          0x25 => self. sla(L), 0x26 => self. sla(Addr::HL),
          0x2f => self. sra(A), 0x28 => self. sra(B),
          0x29 => self. sra(C), 0x2a => self. sra(D),
          0x2b => self. sra(E), 0x2c => self. sra(H),
          0x2d => self. sra(L), 0x2e => self. sra(Addr::HL),
          0x3f => self. srl(A), 0x38 => self. srl(B),
          0x39 => self. srl(C), 0x3a => self. srl(D),
          0x3b => self. srl(E), 0x3c => self. srl(H),
          0x3d => self. srl(L), 0x3e => self. srl(Addr::HL),
          0x37 => self.swap(A), 0x30 => self.swap(B),
          0x31 => self.swap(C), 0x32 => self.swap(D),
          0x33 => self.swap(E), 0x34 => self.swap(H),
          0x35 => self.swap(L), 0x36 => self.swap(Addr::HL),
          0x47 => self.bit(0, A), 0x4f => self.bit(1, A),
          0x57 => self.bit(2, A), 0x5f => self.bit(3, A),
          0x67 => self.bit(4, A), 0x6f => self.bit(5, A),
          0x77 => self.bit(6, A), 0x7f => self.bit(7, A),
          0x40 => self.bit(0, B), 0x48 => self.bit(1, B),
          0x50 => self.bit(2, B), 0x58 => self.bit(3, B),
          0x60 => self.bit(4, B), 0x68 => self.bit(5, B),
          0x70 => self.bit(6, B), 0x78 => self.bit(7, B),
          0x41 => self.bit(0, C), 0x49 => self.bit(1, C),
          0x51 => self.bit(2, C), 0x59 => self.bit(3, C),
          0x61 => self.bit(4, C), 0x69 => self.bit(5, C),
          0x71 => self.bit(6, C), 0x79 => self.bit(7, C),
          0x42 => self.bit(0, D), 0x4a => self.bit(1, D),
          0x52 => self.bit(2, D), 0x5a => self.bit(3, D),
          0x62 => self.bit(4, D), 0x6a => self.bit(5, D),
          0x72 => self.bit(6, D), 0x7a => self.bit(7, D),
          0x43 => self.bit(0, E), 0x4b => self.bit(1, E),
          0x53 => self.bit(2, E), 0x5b => self.bit(3, E),
          0x63 => self.bit(4, E), 0x6b => self.bit(5, E),
          0x73 => self.bit(6, E), 0x7b => self.bit(7, E),
          0x44 => self.bit(0, H), 0x4c => self.bit(1, H),
          0x54 => self.bit(2, H), 0x5c => self.bit(3, H),
          0x64 => self.bit(4, H), 0x6c => self.bit(5, H),
          0x74 => self.bit(6, H), 0x7c => self.bit(7, H),
          0x45 => self.bit(0, L), 0x4d => self.bit(1, L),
          0x55 => self.bit(2, L), 0x5d => self.bit(3, L),
          0x65 => self.bit(4, L), 0x6d => self.bit(5, L),
          0x75 => self.bit(6, L), 0x7d => self.bit(7, L),
          0x46 => self.bit(0, Addr::HL), 0x4e => self.bit(1, Addr::HL),
          0x56 => self.bit(2, Addr::HL), 0x5e => self.bit(3, Addr::HL),
          0x66 => self.bit(4, Addr::HL), 0x6e => self.bit(5, Addr::HL),
          0x76 => self.bit(6, Addr::HL), 0x7e => self.bit(7, Addr::HL),
          0xc7 => self.set(0, A), 0xcf => self.set(1, A),
          0xd7 => self.set(2, A), 0xdf => self.set(3, A),
          0xe7 => self.set(4, A), 0xef => self.set(5, A),
          0xf7 => self.set(6, A), 0xff => self.set(7, A),
          0xc0 => self.set(0, B), 0xc8 => self.set(1, B),
          0xd0 => self.set(2, B), 0xd8 => self.set(3, B),
          0xe0 => self.set(4, B), 0xe8 => self.set(5, B),
          0xf0 => self.set(6, B), 0xf8 => self.set(7, B),
          0xc1 => self.set(0, C), 0xc9 => self.set(1, C),
          0xd1 => self.set(2, C), 0xd9 => self.set(3, C),
          0xe1 => self.set(4, C), 0xe9 => self.set(5, C),
          0xf1 => self.set(6, C), 0xf9 => self.set(7, C),
          0xc2 => self.set(0, D), 0xca => self.set(1, D),
          0xd2 => self.set(2, D), 0xda => self.set(3, D),
          0xe2 => self.set(4, D), 0xea => self.set(5, D),
          0xf2 => self.set(6, D), 0xfa => self.set(7, D),
          0xc3 => self.set(0, E), 0xcb => self.set(1, E),
          0xd3 => self.set(2, E), 0xdb => self.set(3, E),
          0xe3 => self.set(4, E), 0xeb => self.set(5, E),
          0xf3 => self.set(6, E), 0xfb => self.set(7, E),
          0xc4 => self.set(0, H), 0xcc => self.set(1, H),
          0xd4 => self.set(2, H), 0xdc => self.set(3, H),
          0xe4 => self.set(4, H), 0xec => self.set(5, H),
          0xf4 => self.set(6, H), 0xfc => self.set(7, H),
          0xc5 => self.set(0, L), 0xcd => self.set(1, L),
          0xd5 => self.set(2, L), 0xdd => self.set(3, L),
          0xe5 => self.set(4, L), 0xed => self.set(5, L),
          0xf5 => self.set(6, L), 0xfd => self.set(7, L),
          0xc6 => self.set(0, Addr::HL), 0xce => self.set(1, Addr::HL),
          0xd6 => self.set(2, Addr::HL), 0xde => self.set(3, Addr::HL),
          0xe6 => self.set(4, Addr::HL), 0xee => self.set(5, Addr::HL),
          0xf6 => self.set(6, Addr::HL), 0xfe => self.set(7, Addr::HL),
          0x87 => self.res(0, A), 0x8f => self.res(1, A),
          0x97 => self.res(2, A), 0x9f => self.res(3, A),
          0xa7 => self.res(4, A), 0xaf => self.res(5, A),
          0xb7 => self.res(6, A), 0xbf => self.res(7, A),
          0x80 => self.res(0, B), 0x88 => self.res(1, B),
          0x90 => self.res(2, B), 0x98 => self.res(3, B),
          0xa0 => self.res(4, B), 0xa8 => self.res(5, B),
          0xb0 => self.res(6, B), 0xb8 => self.res(7, B),
          0x81 => self.res(0, C), 0x89 => self.res(1, C),
          0x91 => self.res(2, C), 0x99 => self.res(3, C),
          0xa1 => self.res(4, C), 0xa9 => self.res(5, C),
          0xb1 => self.res(6, C), 0xb9 => self.res(7, C),
          0x82 => self.res(0, D), 0x8a => self.res(1, D),
          0x92 => self.res(2, D), 0x9a => self.res(3, D),
          0xa2 => self.res(4, D), 0xaa => self.res(5, D),
          0xb2 => self.res(6, D), 0xba => self.res(7, D),
          0x83 => self.res(0, E), 0x8b => self.res(1, E),
          0x93 => self.res(2, E), 0x9b => self.res(3, E),
          0xa3 => self.res(4, E), 0xab => self.res(5, E),
          0xb3 => self.res(6, E), 0xbb => self.res(7, E),
          0x84 => self.res(0, H), 0x8c => self.res(1, H),
          0x94 => self.res(2, H), 0x9c => self.res(3, H),
          0xa4 => self.res(4, H), 0xac => self.res(5, H),
          0xb4 => self.res(6, H), 0xbc => self.res(7, H),
          0x85 => self.res(0, L), 0x8d => self.res(1, L),
          0x95 => self.res(2, L), 0x9d => self.res(3, L),
          0xa5 => self.res(4, L), 0xad => self.res(5, L),
          0xb5 => self.res(6, L), 0xbd => self.res(7, L),
          0x86 => self.res(0, Addr::HL), 0x8e => self.res(1, Addr::HL),
          0x96 => self.res(2, Addr::HL), 0x9e => self.res(3, Addr::HL),
          0xa6 => self.res(4, Addr::HL), 0xae => self.res(5, Addr::HL),
          0xb6 => self.res(6, Addr::HL), 0xbe => self.res(7, Addr::HL),
          _ => unreachable!("Unknown opcode 0xcb 0x{:0x}", op)
        }
      },
      0xed => self.undefined_debug(),
      0xd3 | 0xdb | 0xdd | 0xe3 | 0xe4 |
      0xeb | 0xec | 0xf4 | 0xfc | 0xfd => self.undefined(op),
      _ => unreachable!("Unknown opcode 0x{:0x}", op)
    }
  }
}
