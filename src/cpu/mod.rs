use std::fmt;

use emulation::EmuTime;
use gameboy::{HiramData, HIRAM_EMPTY};
use hardware::Bus;
use cpu::disasm::{DisasmStr, ToDisasmStr};
use cpu::registers::{
  Registers, Reg8, Reg16, Flags,
  ZERO, ADD_SUBTRACT, HALF_CARRY, CARRY
};
use util::int::IntExt;

pub use cpu::ops::CpuOps;

pub mod disasm;
mod ops;
pub mod registers;

#[cfg(all(test, not(feature = "acceptance_tests")))]
mod test;

pub struct Cpu<H: Bus> {
  time: EmuTime,
  regs: Registers,
  ime: bool,
  ime_change: ImeChange,
  halt: bool,
  hiram: HiramData,
  hardware: H,
  hit_debug: bool
}

pub trait In8: disasm::ResolveOp8 {
  fn read<H: Bus>(&self, &mut Cpu<H>) -> u8;
}
pub trait Out8: disasm::ResolveOp8 {
  fn write<H: Bus>(&self, &mut Cpu<H>, u8);
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
      NZ => !flags.contains(ZERO),  Z => flags.contains(ZERO),
      NC => !flags.contains(CARRY), C => flags.contains(CARRY),
    }
  }
}

#[derive(PartialEq, Eq, Debug)]
enum ImeChange {
  None, Soon, Now
}

pub struct Immediate8;
impl In8 for Immediate8 {
  fn read<H: Bus>(&self, cpu: &mut Cpu<H>) -> u8 { cpu.next_u8() }
}

#[derive(Clone, Copy, Debug)]
pub enum Addr {
  BC, DE, HL, HLD, HLI,
  Direct, ZeroPage, ZeroPageC
}
impl In8 for Addr {
  fn read<H: Bus>(&self, cpu: &mut Cpu<H>) -> u8 {
    let addr = cpu.indirect_addr(*self);
    cpu.read_u8(addr)
  }
}
impl Out8 for Addr {
  fn write<H: Bus>(&self, cpu: &mut Cpu<H>, value: u8) {
    let addr = cpu.indirect_addr(*self);
    cpu.write_u8(addr, value);
  }
}


impl In8 for Reg8 {
  fn read<H: Bus>(&self, cpu: &mut Cpu<H>) -> u8 {
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
  fn write<H: Bus>(&self, cpu: &mut Cpu<H>, value: u8) {
    use cpu::registers::Reg8::*;
    match *self {
      A => cpu.regs.a = value, B => cpu.regs.b = value,
      C => cpu.regs.c = value, D => cpu.regs.d = value,
      E => cpu.regs.e = value, H => cpu.regs.h = value,
      L => cpu.regs.l = value
    }
  }
}


impl<H> fmt::Display for Cpu<H> where H: Bus {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.regs)
  }
}
impl<H> Cpu<H> where H: Bus {
  pub fn new(hardware: H) -> Cpu<H> {
    Cpu {
      regs: Registers::new(),
      ime: true,
      ime_change: ImeChange::None,
      halt: false,
      hiram: HIRAM_EMPTY,
      hardware: hardware,
      time: EmuTime::zero(),
      hit_debug: false
    }
  }
  pub fn hardware(&mut self) -> &mut H {
    &mut self.hardware
  }
  pub fn get_pc(&self) -> u16 {
    self.regs.pc
  }
  pub fn disasm_op(&self) -> DisasmStr {
    let pc = self.regs.pc;
    let time = self.time;

    disasm::disasm(pc, &mut |addr| {
      self.hardware.read(time, addr)
    }).to_disasm_str()
  }
  pub fn rewind_time(&mut self) {
    self.time.rewind();
    self.hardware.rewind_time();
  }
  pub fn clock_cycles(&self) -> u32 { self.time.cycles().as_clock_cycles() }

  pub fn read_hiram(&self, reladdr: u16) -> u8 {
    self.hiram[reladdr as usize]
  }
  pub fn write_hiram(&mut self, reladdr: u16, value: u8) {
    self.hiram[reladdr as usize] = value;
  }

  pub fn ack_debug(&mut self) -> Option<Registers> {
    if !self.hit_debug { None }
    else {
      self.hit_debug = false;
      Some(self.regs.clone())
    }
  }

  fn next_u8(&mut self) -> u8 {
    let addr = self.regs.pc;
    self.regs.pc += 1;
    self.read_u8(addr)
  }
  fn next_u16(&mut self) -> u16 {
    let l = self.next_u8();
    let h = self.next_u8();
    ((h as u16) << 8) | (l as u16)
  }
  fn read_u8(&mut self, addr: u16) -> u8 {
    self.time.tick();
    self.hardware.emulate(self.time);

    if addr < 0xff80 || addr == 0xffff {
      self.hardware.read(self.time, addr)
    } else {
      self.read_hiram(addr & 0x7f)
    }
  }
  fn write_u8(&mut self, addr: u16, value: u8) {
    self.time.tick();
    self.hardware.emulate(self.time);

    if addr < 0xff80 || addr == 0xffff {
      self.hardware.write(self.time, addr, value);
    } else {
      self.write_hiram(addr & 0x7f, value);
    }
  }

  fn read_u16(&mut self, addr: u16) -> u16 {
    (self.read_u8(addr) as u16) |
    ((self.read_u8(addr.wrapping_add_one()) as u16) << 8)
  }
  fn write_u16(&mut self, addr: u16, value: u16) {
    self.write_u8(addr, value as u8);
    self.write_u8((addr.wrapping_add_one()), (value >> 8) as u8);
  }

  fn pop_u8(&mut self) -> u8 {
    let sp = self.regs.sp;
    let value = self.read_u8(sp);
    self.regs.sp = self.regs.sp.wrapping_add_one();
    value
  }
  fn push_u8(&mut self, value: u8) {
    self.regs.sp = self.regs.sp.wrapping_sub_one();
    let sp = self.regs.sp;
    self.write_u8(sp, value);
  }

  fn pop_u16(&mut self) -> u16 {
    let l = self.pop_u8();
    let h = self.pop_u8();
    ((h as u16) << 8) | (l as u16)
  }
  fn push_u16(&mut self, value: u16) {
    self.push_u8((value >> 8) as u8);
    self.push_u8(value as u8);
  }

  fn indirect_addr(&mut self, addr: Addr) -> u16 {
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
      Direct => self.next_u16(),
      ZeroPage => 0xff00u16 | self.next_u8() as u16,
      ZeroPageC => 0xff00u16 | self.regs.c as u16,
    }
  }

  pub fn execute(&mut self) {
    if self.halt {
      if self.hardware.has_interrupt() {
        self.halt = false;
      } else {
        self.time.tick();
        self.hardware.emulate(self.time);
      }
    } else {
      match self.ime_change {
        ImeChange::None => (),
        ImeChange::Soon => {
          self.ime_change = ImeChange::Now;
        },
        ImeChange::Now => {
          self.ime = true;
          self.ime_change = ImeChange::None;
        }
      }

      if self.ime {
        match self.hardware.ack_interrupt() {
          None => (),
          Some(interrupt) => {
            self.halt = false;
            self.ime = false;
            self.time.tick();
            self.hardware.emulate(self.time);
            self.time.tick();
            self.hardware.emulate(self.time);
            self.time.tick();
            self.hardware.emulate(self.time);
            let pc = self.regs.pc;
            self.push_u16(pc);
            self.regs.pc = interrupt.get_addr();
          }
        }
      }

      let op = self.next_u8();
      ops::decode(self, op)
    }
  }

  pub fn execute_until(&mut self, time: EmuTime) {
    while self.time < time {
      self.execute();
    }
  }

  fn alu_add(&mut self, value: u8, use_carry: bool) {
    let cy = if use_carry && self.regs.f.contains(CARRY) { 1 } else { 0 };
    let result = self.regs.a.wrapping_add(value).wrapping_add(cy);
    self.regs.f = ZERO.test(result == 0) |
                  CARRY.test(self.regs.a as u16 + value as u16 + cy as u16 > 0xff) |
                  HALF_CARRY.test((self.regs.a & 0xf) + (value & 0xf) + cy > 0xf);
    self.regs.a = result;
  }
  fn alu_sub(&mut self, value: u8, use_carry: bool) -> u8 {
    let cy = if use_carry && self.regs.f.contains(CARRY) { 1 } else { 0 };
    let result = self.regs.a.wrapping_sub(value).wrapping_sub(cy);
    self.regs.f = ZERO.test(result == 0) |
                  ADD_SUBTRACT |
                  CARRY.test((self.regs.a as u16) < (value as u16) + (cy as u16)) |
                  HALF_CARRY.test((self.regs.a & 0xf) < (value & 0xf) + cy);
    result
  }
  fn alu_rl(&mut self, value: u8, set_zero: bool) -> u8 {
    let ci = if self.regs.f.contains(CARRY) { 1 } else { 0 };
    let co = value & 0x80;
    let new_value = (value << 1) | ci;
    self.regs.f = ZERO.test(set_zero && new_value == 0) |
                  CARRY.test(co != 0);
    new_value
  }
  fn alu_rlc(&mut self, value: u8, set_zero: bool) -> u8 {
    let co = value & 0x80;
    let new_value = value.rotate_left(1);
    self.regs.f = ZERO.test(set_zero && new_value == 0) |
                  CARRY.test(co != 0);
    new_value
  }
  fn alu_rr(&mut self, value: u8, set_zero: bool) -> u8 {
    let ci = if self.regs.f.contains(CARRY) { 1 } else { 0 };
    let co = value & 0x01;
    let new_value = (value >> 1) | (ci << 7);
    self.regs.f = ZERO.test(set_zero && new_value == 0) |
                  CARRY.test(co != 0);
    new_value
  }
  fn alu_rrc(&mut self, value: u8, set_zero: bool) -> u8 {
    let co = value & 0x01;
    let new_value = value.rotate_right(1);
    self.regs.f = ZERO.test(set_zero && new_value == 0) |
                  CARRY.test(co != 0);
    new_value
  }
  fn ctrl_jp(&mut self, addr: u16) {
    self.regs.pc = addr;
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  fn ctrl_jr(&mut self, offset: i8) {
    self.regs.pc = (self.regs.pc as i16 + offset as i16) as u16;
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  fn ctrl_call(&mut self, addr: u16) {
    let pc = self.regs.pc;
    self.time.tick();
    self.hardware.emulate(self.time);
    self.push_u16(pc);
    self.regs.pc = addr;
  }
  fn ctrl_ret(&mut self) {
    self.regs.pc = self.pop_u16();
    self.time.tick();
    self.hardware.emulate(self.time);
  }
}


impl<'a, H> CpuOps for &'a mut Cpu<H> where H: Bus {
  type R = ();
  // --- 8-bit operations
  // 8-bit loads
  /// LD d, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load<O: Out8, I: In8>(self, out8: O, in8: I) {
    let value = in8.read(self);
    out8.write(self, value);
  }
  // 8-bit arithmetic
  /// ADD s
  ///
  /// Flags: Z N H C
  ///        * 0 * *
  fn add<I: In8>(self, in8: I) {
    let value = in8.read(self);
    self.alu_add(value, false);
  }
  /// ADC s
  ///
  /// Flags: Z N H C
  ///        * 0 * *
  fn adc<I: In8>(self, in8: I) {
    let value = in8.read(self);
    self.alu_add(value, true);
  }
  /// SUB s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  fn sub<I: In8>(self, in8: I) {
    let value = in8.read(self);
    self.regs.a = self.alu_sub(value, false);
  }
  /// SBC s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  fn sbc<I: In8>(self, in8: I) {
    let value = in8.read(self);
    self.regs.a = self.alu_sub(value, true);
  }
  /// CP s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  fn cp<I: In8>(self, in8: I) {
    let value = in8.read(self);
    self.alu_sub(value, false);
  }
  /// AND s
  ///
  /// Flags: Z N H C
  ///        * 0 1 0
  fn and<I: In8>(self, in8: I) {
    let value = in8.read(self);
    self.regs.a = self.regs.a & value;
    self.regs.f = ZERO.test(self.regs.a == 0) |
                  HALF_CARRY;
  }
  /// OR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  fn or<I: In8>(self, in8: I) {
    let value = in8.read(self);
    self.regs.a = self.regs.a | value;
    self.regs.f = ZERO.test(self.regs.a == 0);
  }
  /// XOR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  fn xor<I: In8>(self, in8: I) {
    let value = in8.read(self);
    self.regs.a = self.regs.a ^ value;
    self.regs.f = ZERO.test(self.regs.a == 0)
  }
  /// INC s
  ///
  /// Flags: Z N H C
  ///        * 0 * -
  fn inc<IO: In8+Out8>(self, io: IO) {
    let value = io.read(self);
    let new_value = value.wrapping_add_one();
    self.regs.f = ZERO.test(new_value == 0) |
                  HALF_CARRY.test(value & 0xf == 0xf) |
                  (CARRY & self.regs.f);
    io.write(self, new_value);
  }
  /// DEC s
  ///
  /// Flags: Z N H C
  ///        * 0 * -
  fn dec<IO: In8+Out8>(self, io: IO) {
    let value = io.read(self);
    let new_value = value.wrapping_sub_one();
    self.regs.f = ZERO.test(new_value == 0) |
                  ADD_SUBTRACT |
                  HALF_CARRY.test(value & 0xf == 0) |
                  (CARRY & self.regs.f);
    io.write(self, new_value);
  }
  /// RLCA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rlca(self) {
    let value = self.regs.a;
    self.regs.a = self.alu_rlc(value, false);
  }
  /// RLA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rla(self) {
    let value = self.regs.a;
    self.regs.a = self.alu_rl(value, false);
  }
  /// RRCA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rrca(self) {
    let value = self.regs.a;
    self.regs.a = self.alu_rrc(value, false);
  }
  /// RRA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rra(self) {
    let value = self.regs.a;
    self.regs.a = self.alu_rr(value, false);
  }
  /// RLC s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rlc<IO: In8+Out8>(self, io: IO) {
    let value = io.read(self);
    let new_value = self.alu_rlc(value, true);
    io.write(self, new_value);
  }
  /// RL s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rl<IO: In8+Out8>(self, io: IO) {
    let value = io.read(self);
    let new_value = self.alu_rl(value, true);
    io.write(self, new_value);
  }
  /// RRC s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rrc<IO: In8+Out8>(self, io: IO) {
    let value = io.read(self);
    let new_value = self.alu_rrc(value, true);
    io.write(self, new_value);
  }
  /// RR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rr<IO: In8+Out8>(self, io: IO) {
    let value = io.read(self);
    let new_value = self.alu_rr(value, true);
    io.write(self, new_value);
  }
  /// SLA s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn sla<IO: In8+Out8>(self, io: IO) {
    let value = io.read(self);
    let co = value & 0x80;
    let new_value = value << 1;
    self.regs.f = ZERO.test(new_value == 0) |
                  CARRY.test(co != 0);
    io.write(self, new_value);
  }
  /// SRA s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn sra<IO: In8+Out8>(self, io: IO) {
    let value = io.read(self);
    let co = value & 0x01;
    let hi = value & 0x80;
    let new_value = (value >> 1) | hi;
    self.regs.f = ZERO.test(new_value == 0) |
                  CARRY.test(co != 0);
    io.write(self, new_value);
  }
  /// SRL s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn srl<IO: In8+Out8>(self, io: IO) {
    let value = io.read(self);
    let co = value & 0x01;
    let new_value = value >> 1;
    self.regs.f = ZERO.test(new_value == 0) |
                  CARRY.test(co != 0);
    io.write(self, new_value);
  }
  /// SWAP s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  fn swap<IO: In8+Out8>(self, io: IO) {
    let value = io.read(self);
    let new_value = (value >> 4) | (value << 4);
    self.regs.f = ZERO.test(value == 0);
    io.write(self, new_value);
  }
  /// BIT b, s
  ///
  /// Flags: Z N H C
  ///        * 0 1 -
  fn bit<I: In8>(self, bit: usize, in8: I) {
    let value = in8.read(self) & (1 << bit);
    self.regs.f = ZERO.test(value == 0) |
                  HALF_CARRY |
                  (CARRY & self.regs.f);
  }
  /// SET b, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn set<IO: In8+Out8>(self, bit: usize, io: IO) {
    let value = io.read(self) | (1 << bit);
    io.write(self, value);
  }
  /// RES b, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn res<IO: In8+Out8>(self, bit: usize, io: IO) {
    let value = io.read(self) & !(1 << bit);
    io.write(self, value);
  }
  // --- Control
  /// JP nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jp(self) {
    let addr = self.next_u16();
    self.ctrl_jp(addr);
  }
  /// JP HL
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jp_hl(self) {
    self.regs.pc = self.regs.read16(Reg16::HL);
  }
  /// JR e
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jr(self) {
    let offset = self.next_u8() as i8;
    self.ctrl_jr(offset);
  }
  /// CALL nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn call(self) {
    let addr = self.next_u16();
    self.ctrl_call(addr);
  }
  /// RET
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn ret(self) {
    self.ctrl_ret();
  }
  /// RETI
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn reti(self) {
    self.ime = true;
    self.ime_change = ImeChange::None;
    self.ctrl_ret();
  }
  /// JP cc, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jp_cc(self, cond: Cond) {
    let addr = self.next_u16();
    if cond.check(self.regs.f) {
      self.ctrl_jp(addr);
    }
  }
  /// JR cc, e
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jr_cc(self, cond: Cond) {
    let offset = self.next_u8() as i8;
    if cond.check(self.regs.f) {
      self.ctrl_jr(offset);
    }
  }
  /// CALL cc, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn call_cc(self, cond: Cond) {
    let addr = self.next_u16();
    if cond.check(self.regs.f) {
      self.ctrl_call(addr);
    }
  }
  /// RET cc
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn ret_cc(self, cond: Cond) {
    self.time.tick();
    self.hardware.emulate(self.time);
    if cond.check(self.regs.f) {
      self.ctrl_ret();
    }
  }
  /// RST n
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn rst(self, addr: u8) {
    let pc = self.regs.pc;
    self.time.tick();
    self.hardware.emulate(self.time);
    self.push_u16(pc);
    self.regs.pc = addr as u16;
  }
  // --- Miscellaneous
  /// HALT
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn halt(self) {
    // TODO: DMG BUG
    self.halt = true;
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
    self.ime = false;
    self.ime_change = ImeChange::None;
  }
  /// EI
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn ei(self) {
    self.ime_change = ImeChange::Soon;
  }
  /// CCF
  ///
  /// Flags: Z N H C
  ///        - 0 0 *
  fn ccf(self) {
    self.regs.f = (ZERO & self.regs.f) |
                  CARRY.test(!self.regs.f.contains(CARRY))
  }
  /// SCF
  ///
  /// Flags: Z N H C
  ///        - 0 0 1
  fn scf(self) {
    self.regs.f = (ZERO & self.regs.f) |
                  CARRY
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
    // DAA table in page 110 of the official "Game Boy Programming Manual"
    let mut carry = false;
    if !self.regs.f.contains(ADD_SUBTRACT) {
      if self.regs.f.contains(CARRY) || self.regs.a > 0x99 {
        self.regs.a = self.regs.a.wrapping_add(0x60);
        carry = true;
      }
      if self.regs.f.contains(HALF_CARRY) || self.regs.a & 0x0f > 0x09 {
        self.regs.a = self.regs.a.wrapping_add(0x06);
      }
    } else {
      if self.regs.f.contains(CARRY) {
        carry = true;
        self.regs.a = self.regs.a.wrapping_add(
          if self.regs.f.contains(HALF_CARRY) { 0x9a }
          else { 0xa0 }
        );
      } else if self.regs.f.contains(HALF_CARRY) {
        self.regs.a = self.regs.a.wrapping_add(0xfa);
      }
    }

    self.regs.f = ZERO.test(self.regs.a == 0) |
                  (ADD_SUBTRACT & self.regs.f) |
                  CARRY.test(carry);
  }
  /// CPL
  ///
  /// Flags: Z N H C
  ///        - 1 1 -
  fn cpl(self) {
    self.regs.a = !self.regs.a;
    self.regs.f = (ZERO & self.regs.f) |
                  ADD_SUBTRACT |
                  HALF_CARRY |
                  (CARRY & self.regs.f);
  }
  // --- 16-bit operations
  // 16-bit loads
  /// LD dd, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load16_imm(self, reg: Reg16) {
    let value = self.next_u16();
    self.regs.write16(reg, value);
  }
  /// LD (nn), SP
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load16_nn_sp(self) {
    let value = self.regs.sp;
    let addr = self.next_u16();
    self.write_u16(addr, value);
  }
  /// LD SP, HL
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load16_sp_hl(self) {
    let value = self.regs.read16(Reg16::HL);
    self.regs.sp = value;
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// LD HL, SP+e
  ///
  /// Flags: Z N H C
  ///        0 0 * *
  fn load16_hl_sp_e(self) {
    let offset = self.next_u8() as i8 as u16;
    let sp = self.regs.sp as u16;
    let value = sp.wrapping_add(offset);
    self.regs.write16(Reg16::HL, value);
    self.regs.f = HALF_CARRY.test(u16::test_add_carry_bit(3, sp, offset)) |
                  CARRY.test(u16::test_add_carry_bit(7, sp, offset));
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// PUSH rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn push16(self, reg: Reg16) {
    let value = self.regs.read16(reg);
    self.time.tick();
    self.hardware.emulate(self.time);
    self.push_u16(value);
  }
  /// POP rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  /// Note! POP AF affects all flags
  fn pop16(self, reg: Reg16) {
    let value = self.pop_u16();
    self.regs.write16(reg, value);
  }
  // 16-bit arithmetic
  /// ADD HL, ss
  ///
  /// Flags: Z N H C
  ///        - 0 * *
  fn add16(self, reg: Reg16) {
    let hl = self.regs.read16(Reg16::HL);
    let value = self.regs.read16(reg);
    let result = hl.wrapping_add(value);
    self.regs.f = (ZERO & self.regs.f) |
                  HALF_CARRY.test(u16::test_add_carry_bit(11, hl, value)) |
                  CARRY.test(hl > 0xffff - value);
    self.regs.write16(Reg16::HL, result);
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// ADD SP, e
  ///
  /// Flags: Z N H C
  ///        0 0 * *
  fn add16_sp_e(self) {
    let val = self.next_u8() as i8 as i16 as u16;
    let sp = self.regs.sp;
    self.regs.sp = sp.wrapping_add(val);
    self.regs.f = HALF_CARRY.test(u16::test_add_carry_bit(3, sp, val)) |
                  CARRY.test(u16::test_add_carry_bit(7, sp, val));
    self.time.tick();
    self.hardware.emulate(self.time);
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// INC rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn inc16(self, reg: Reg16) {
    let value = self.regs.read16(reg).wrapping_add_one();
    self.regs.write16(reg, value);
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// DEC rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn dec16(self, reg: Reg16) {
    let value = self.regs.read16(reg).wrapping_sub_one();
    self.regs.write16(reg, value);
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  // --- Undefined
  fn undefined(self, op: u8) {
    panic!("Undefined opcode {}", op)
  }
  fn undefined_debug(self) {
    self.hit_debug = true;
  }
  fn cb_prefix(self) {
    let op = self.next_u8();
    ops::decode_cb(self, op)
  }
}
