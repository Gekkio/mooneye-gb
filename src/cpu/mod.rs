use std::fmt;
use std::num::Int;

use emulation::EmuTime;
use gameboy::{HiramData, HIRAM_EMPTY};
use hardware::Bus;
use cpu::disasm::{DisasmStr, ToDisasmStr};
use cpu::registers::{
  Registers, Reg8, Reg16, Flags,
  ZERO, ADD_SUBTRACT, HALF_CARRY, CARRY
};

pub use cpu::ops::CpuOps;

mod disasm;
mod ops;
mod registers;

#[cfg(test)]
mod test;

pub struct Cpu<H: Bus> {
  regs: Registers,
  ime: bool,
  ime_change: ImeChange,
  halt: bool,
  hiram: HiramData,
  hardware: H,
  time: EmuTime
}


pub trait In8: ToDisasmStr {
  fn read<H: Bus>(&self, &mut Cpu<H>) -> u8;
}
pub trait In16: ToDisasmStr {
  fn read<H: Bus>(&self, &mut Cpu<H>) -> u16;
}


pub trait Out8: ToDisasmStr {
  fn write<H: Bus>(&self, &mut Cpu<H>, u8);
}
pub trait Out16: ToDisasmStr {
  fn write<H: Bus>(&self, &mut Cpu<H>, u16);
}


#[derive(Copy, Show)]
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

#[derive(PartialEq, Eq, Show)]
enum ImeChange {
  None, Soon(bool), Now(bool)
}

pub struct Immediate8;
impl In8 for Immediate8 {
  fn read<H: Bus>(&self, cpu: &mut Cpu<H>) -> u8 { cpu.next_u8() }
}
pub struct Immediate16;
impl In16 for Immediate16 {
  fn read<H: Bus>(&self, cpu: &mut Cpu<H>) -> u16 { cpu.next_u16() }
}


pub struct Direct16;
impl In16 for Direct16 {
  fn read<H: Bus>(&self, cpu: &mut Cpu<H>) -> u16 {
    let addr = cpu.next_u16();
    cpu.read_u16(addr)
  }
}
impl Out16 for Direct16 {
  fn write<H: Bus>(&self, cpu: &mut Cpu<H>, value: u16) {
    let addr = cpu.next_u16();
    cpu.write_u16(addr, value);
  }
}


impl In16 for Reg16 {
  fn read<H: Bus>(&self, cpu: &mut Cpu<H>) -> u16 {
    cpu.regs.read16(*self)
  }
}
impl Out16 for Reg16 {
  fn write<H: Bus>(&self, cpu: &mut Cpu<H>, value: u16) {
    cpu.regs.write16(*self, value)
  }
}


#[derive(Copy)]
pub enum Addr {
  BC, DE, HL, HLD, HLI,
  Direct, ZeroPage, ZeroPageC
}
impl Addr {
  fn addr<H: Bus>(&self, cpu: &mut Cpu<H>) -> u16 {
    use self::Addr::*;
    match *self {
      BC => cpu.regs.read16(Reg16::BC),
      DE => cpu.regs.read16(Reg16::DE),
      HL => cpu.regs.read16(Reg16::HL),
      HLD => {
        let addr = cpu.regs.read16(Reg16::HL);
        cpu.regs.write16(Reg16::HL, addr - 1);
        addr
      },
      HLI => {
        let addr = cpu.regs.read16(Reg16::HL);
        cpu.regs.write16(Reg16::HL, addr + 1);
        addr
      },
      Direct => cpu.next_u16(),
      ZeroPage => 0xff00 as u16 + cpu.next_u8() as u16,
      ZeroPageC => 0xff00 as u16 + cpu.regs.c as u16,
    }
  }
}
impl In8 for Addr {
  fn read<H: Bus>(&self, cpu: &mut Cpu<H>) -> u8 {
    let addr = self.addr(cpu);
    cpu.read_u8(addr)
  }
}
impl Out8 for Addr {
  fn write<H: Bus>(&self, cpu: &mut Cpu<H>, value: u8) {
    let addr = self.addr(cpu);
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


impl<H> fmt::Show for Cpu<H> where H: Bus {
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
      time: EmuTime::zero()
    }
  }
  pub fn hardware(&mut self) -> &mut H {
    &mut self.hardware
  }
  pub fn get_pc(&self) -> u16 {
    self.regs.pc
  }
  pub fn disasm_op(&self) -> DisasmStr {
    disasm::disasm(&self.hardware, self.regs.pc, self.time)
  }
  pub fn rewind_time(&mut self) {
    self.time.rewind();
    self.hardware.rewind_time();
  }
  pub fn clock_cycles(&self) -> u32 { self.time.cycles().as_clock_cycles() }

  pub fn read_hiram(&self, reladdr: u16) -> u8 {
    self.hiram[reladdr as uint]
  }
  pub fn write_hiram(&mut self, reladdr: u16, value: u8) {
    self.hiram[reladdr as uint] = value;
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
    ((self.read_u8(addr + 1) as u16) << 8)
  }
  fn write_u16(&mut self, addr: u16, value: u16) {
    self.write_u8(addr, value as u8);
    self.write_u8(addr + 1, (value >> 8) as u8);
  }

  fn pop_u8(&mut self) -> u8 {
    let sp = self.regs.sp;
    let value = self.read_u8(sp);
    self.regs.sp += 1;
    value
  }
  fn push_u8(&mut self, value: u8) {
    self.regs.sp -= 1;
    let sp = self.regs.sp;
    self.write_u8(sp, value);
  }

  fn pop_u16(&mut self) -> u16 {
    let l = self.pop_u8();
    let h = self.pop_u8();
    ((h as u16) << 8) + (l as u16)
  }
  fn push_u16(&mut self, value: u16) {
    self.push_u8((value >> 8) as u8);
    self.push_u8(value as u8);
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
      self.decode();

      match self.ime_change {
        ImeChange::None => (),
        ImeChange::Soon(value) => {
          self.ime_change = ImeChange::Now(value);
        },
        ImeChange::Now(value) => {
          self.ime = value;
          self.ime_change = ImeChange::None;
        }
      }

      if self.ime {
        match self.hardware.ack_interrupt() {
          None => (),
          Some(interrupt) => {
            self.halt = false;
            self.ime = false;
            let pc = self.regs.pc;
            self.push_u16(pc);
            self.regs.pc = interrupt.get_addr();
            self.time.tick();
            self.hardware.emulate(self.time);
            return;
          }
        }
      }
    }
  }

  pub fn execute_until(&mut self, time: EmuTime) {
    while self.time < time {
      self.execute();
    }
  }

  fn alu_add(&mut self, value: u8, use_carry: bool) {
    let cy = if use_carry && self.regs.f.contains(CARRY) { 1 } else { 0 };
    let result = self.regs.a + value + cy;
    self.regs.f = ZERO.test(result == 0) |
                  CARRY.test(self.regs.a as u16 + value as u16 + cy as u16 > 0xff) |
                  HALF_CARRY.test((self.regs.a & 0xf) + (value & 0xf) + cy > 0xf);
    self.regs.a = result;
  }
  fn alu_sub(&mut self, value: u8, use_carry: bool) -> u8 {
    let cy = if use_carry && self.regs.f.contains(CARRY) { 1 } else { 0 };
    let result = self.regs.a - value - cy;
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
    self.push_u16(pc);
    self.regs.pc = addr;
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  fn ctrl_ret(&mut self) {
    self.regs.pc = self.pop_u16();
    self.time.tick();
    self.hardware.emulate(self.time);
  }
}


impl<H> CpuOps<()> for Cpu<H> where H: Bus {
  fn next_u8(&mut self) -> u8 {
    let addr = self.regs.pc;
    self.regs.pc += 1;
    self.read_u8(addr)
  }
  // --- 8-bit operations
  // 8-bit loads
  /// LD d, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load<O: Out8, I: In8>(&mut self, out8: O, in8: I) {
    let value = in8.read(self);
    out8.write(self, value);
  }
  // 8-bit arithmetic
  /// ADD s
  ///
  /// Flags: Z N H C
  ///        * 0 * *
  fn add<I: In8>(&mut self, in8: I) {
    let value = in8.read(self);
    self.alu_add(value, false);
  }
  /// ADC s
  ///
  /// Flags: Z N H C
  ///        * 0 * *
  fn adc<I: In8>(&mut self, in8: I) {
    let value = in8.read(self);
    self.alu_add(value, true);
  }
  /// SUB s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  fn sub<I: In8>(&mut self, in8: I) {
    let value = in8.read(self);
    self.regs.a = self.alu_sub(value, false);
  }
  /// SBC s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  fn sbc<I: In8>(&mut self, in8: I) {
    let value = in8.read(self);
    self.regs.a = self.alu_sub(value, true);
  }
  /// CP s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  fn cp<I: In8>(&mut self, in8: I) {
    let value = in8.read(self);
    self.alu_sub(value, false);
  }
  /// AND s
  ///
  /// Flags: Z N H C
  ///        * 0 1 0
  fn and<I: In8>(&mut self, in8: I) {
    let value = in8.read(self);
    self.regs.a = self.regs.a & value;
    self.regs.f = ZERO.test(self.regs.a == 0) |
                  HALF_CARRY;
  }
  /// OR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  fn or<I: In8>(&mut self, in8: I) {
    let value = in8.read(self);
    self.regs.a = self.regs.a | value;
    self.regs.f = ZERO.test(self.regs.a == 0);
  }
  /// XOR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  fn xor<I: In8>(&mut self, in8: I) {
    let value = in8.read(self);
    self.regs.a = self.regs.a ^ value;
    self.regs.f = ZERO.test(self.regs.a == 0)
  }
  /// INC s
  ///
  /// Flags: Z N H C
  ///        * 0 * -
  fn inc<IO: In8+Out8>(&mut self, io: IO) {
    let value = io.read(self);
    let new_value = value + 1;
    self.regs.f = ZERO.test(new_value == 0) |
                  HALF_CARRY.test(value & 0xf == 0xf) |
                  (CARRY & self.regs.f);
    io.write(self, new_value);
  }
  /// DEC s
  ///
  /// Flags: Z N H C
  ///        * 0 * -
  fn dec<IO: In8+Out8>(&mut self, io: IO) {
    let value = io.read(self);
    let new_value = value - 1;
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
  fn rlca(&mut self) {
    let value = self.regs.a;
    self.regs.a = self.alu_rlc(value, false);
  }
  /// RLA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rla(&mut self) {
    let value = self.regs.a;
    self.regs.a = self.alu_rl(value, false);
  }
  /// RRCA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rrca(&mut self) {
    let value = self.regs.a;
    self.regs.a = self.alu_rrc(value, false);
  }
  /// RRA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  fn rra(&mut self) {
    let value = self.regs.a;
    self.regs.a = self.alu_rr(value, false);
  }
  /// RLC s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rlc<IO: In8+Out8>(&mut self, io: IO) {
    let value = io.read(self);
    let new_value = self.alu_rlc(value, true);
    io.write(self, new_value);
  }
  /// RL s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rl<IO: In8+Out8>(&mut self, io: IO) {
    let value = io.read(self);
    let new_value = self.alu_rl(value, true);
    io.write(self, new_value);
  }
  /// RRC s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rrc<IO: In8+Out8>(&mut self, io: IO) {
    let value = io.read(self);
    let new_value = self.alu_rrc(value, true);
    io.write(self, new_value);
  }
  /// RR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn rr<IO: In8+Out8>(&mut self, io: IO) {
    let value = io.read(self);
    let new_value = self.alu_rr(value, true);
    io.write(self, new_value);
  }
  /// SLA s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  fn sla<IO: In8+Out8>(&mut self, io: IO) {
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
  fn sra<IO: In8+Out8>(&mut self, io: IO) {
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
  fn srl<IO: In8+Out8>(&mut self, io: IO) {
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
  fn swap<IO: In8+Out8>(&mut self, io: IO) {
    let value = io.read(self);
    let new_value = (value >> 4) | (value << 4);
    self.regs.f = ZERO.test(value == 0);
    io.write(self, new_value);
  }
  /// BIT b, s
  ///
  /// Flags: Z N H C
  ///        * 0 1 -
  fn bit<I: In8>(&mut self, bit: uint, in8: I) {
    let value = in8.read(self) & (1 << bit);
    self.regs.f = ZERO.test(value == 0) |
                  HALF_CARRY |
                  (CARRY & self.regs.f);
  }
  /// SET b, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn set<IO: In8+Out8>(&mut self, bit: uint, io: IO) {
    let value = io.read(self) | (1 << bit);
    io.write(self, value);
  }
  /// RES b, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn res<IO: In8+Out8>(&mut self, bit: uint, io: IO) {
    let value = io.read(self) & !(1 << bit);
    io.write(self, value);
  }
  // --- Control
  /// JP nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jp(&mut self) {
    let addr = self.next_u16();
    self.ctrl_jp(addr);
  }
  /// JP HL
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jp_hl(&mut self) {
    self.regs.pc = self.regs.read16(Reg16::HL);
  }
  /// JR e
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jr(&mut self) {
    let offset = self.next_u8() as i8;
    self.ctrl_jr(offset);
  }
  /// CALL nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn call(&mut self) {
    let addr = self.next_u16();
    self.ctrl_call(addr);
  }
  /// RET
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn ret(&mut self) {
    self.ctrl_ret();
  }
  /// RETI
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn reti(&mut self) {
    self.ime_change = ImeChange::Now(true);
    self.ctrl_ret();
  }
  /// JP cc, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jp_cc(&mut self, cond: Cond) {
    let addr = self.next_u16();
    if cond.check(self.regs.f) {
      self.ctrl_jp(addr);
    }
  }
  /// JR cc, e
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn jr_cc(&mut self, cond: Cond) {
    let offset = self.next_u8() as i8;
    if cond.check(self.regs.f) {
      self.ctrl_jr(offset);
    }
  }
  /// CALL cc, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn call_cc(&mut self, cond: Cond) {
    let addr = self.next_u16();
    if cond.check(self.regs.f) {
      self.ctrl_call(addr);
    }
  }
  /// RET cc
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn ret_cc(&mut self, cond: Cond) {
    if cond.check(self.regs.f) {
      self.ctrl_ret();
    }
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// RST n
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn rst(&mut self, addr: u8) {
    let pc = self.regs.pc;
    self.push_u16(pc);
    self.regs.pc = addr as u16;
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  // --- Miscellaneous
  /// HALT
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn halt(&mut self) {
    // TODO: DMG BUG
    self.halt = true;
  }
  /// STOP
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn stop(&mut self) {
    panic!("STOP")
  }
  /// DI
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn di(&mut self) {
    self.ime_change = ImeChange::Soon(false);
  }
  /// EI
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn ei(&mut self) {
    self.ime_change = ImeChange::Soon(true);
  }
  /// CCF
  ///
  /// Flags: Z N H C
  ///        - 0 0 *
  fn ccf(&mut self) {
    self.regs.f = (ZERO & self.regs.f) |
                  CARRY.test(!self.regs.f.contains(CARRY))
  }
  /// SCF
  ///
  /// Flags: Z N H C
  ///        - 0 0 1
  fn scf(&mut self) {
    self.regs.f = (ZERO & self.regs.f) |
                  CARRY
  }
  /// NOP
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn nop(&mut self) {
  }
  /// DAA
  ///
  /// Flags: Z N H C
  ///        * - 0 *
  fn daa(&mut self) {
    // DAA table in page 110 of the official "Game Boy Programming Manual"
    let mut carry = false;
    if !self.regs.f.contains(ADD_SUBTRACT) {
      if self.regs.f.contains(CARRY) || self.regs.a > 0x99 {
        self.regs.a += 0x60;
        carry = true;
      }
      if self.regs.f.contains(HALF_CARRY) || self.regs.a & 0x0f > 0x09 {
        self.regs.a += 0x06;
      }
    } else {
      if self.regs.f.contains(CARRY) {
        carry = true;
        self.regs.a +=
          if self.regs.f.contains(HALF_CARRY) { 0x9a }
          else { 0xa0 }
      } else if self.regs.f.contains(HALF_CARRY) {
        self.regs.a += 0xfa;
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
  fn cpl(&mut self) {
    self.regs.a = !self.regs.a;
    self.regs.f = (ZERO & self.regs.f) |
                  ADD_SUBTRACT |
                  HALF_CARRY |
                  (CARRY & self.regs.f);
  }
  // --- 16-bit operations
  // 16-bit loads
  /// LD dd, ss
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load16<O: Out16, I: In16>(&mut self, out8: O, in8: I) {
    let value = in8.read(self);
    out8.write(self, value);
  }
  /// LD SP, HL
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn load16_sp_hl(&mut self) {
    let value = self.regs.read16(Reg16::HL);
    self.regs.sp = value;
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// LD HL, SP+e
  ///
  /// Flags: Z N H C
  ///        0 0 * *
  fn load16_hl_sp_e(&mut self) {
    let offset = self.next_u8() as i8 as i16;
    let sp = self.regs.sp as i16;
    let value = (sp + offset) as u16;
    self.regs.write16(Reg16::HL, value);
    self.regs.f = HALF_CARRY.test((sp & 0x000f) + (offset & 0x000f) > 0x000f) |
                  CARRY.test((sp & 0x00ff) + (offset & 0x00ff) > 0x00ff);
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// PUSH rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn push16(&mut self, reg: Reg16) {
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
  fn pop16(&mut self, reg: Reg16) {
    let value = self.pop_u16();
    self.regs.write16(reg, value);
  }
  // 16-bit arithmetic
  /// ADD HL, ss
  ///
  /// Flags: Z N H C
  ///        - 0 * *
  fn add16(&mut self, reg: Reg16) {
    let hl = self.regs.read16(Reg16::HL);
    let value = self.regs.read16(reg);
    let result = hl + value;
    self.regs.f = (ZERO & self.regs.f) |
                  HALF_CARRY.test((hl & 0x07FF) + (value & 0x07FF) > 0x07FF) |
                  CARRY.test(hl > 0xffff - value);
    self.regs.write16(Reg16::HL, result);
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// ADD SP, e
  ///
  /// Flags: Z N H C
  ///        0 0 * *
  fn add16_sp_e(&mut self) {
    let val = self.next_u8() as i8 as i16 as u16;
    let sp = self.regs.sp;
    self.regs.sp = sp + val;
    self.regs.f = HALF_CARRY.test((sp & 0x000f) + (val & 0x000f) > 0x000f) |
                  CARRY.test((sp & 0x00ff) + (val & 0x00ff) > 0x00ff);
    self.time.tick();
    self.hardware.emulate(self.time);
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// INC rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn inc16(&mut self, reg: Reg16) {
    let value = self.regs.read16(reg) + 1;
    self.regs.write16(reg, value);
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  /// DEC rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  fn dec16(&mut self, reg: Reg16) {
    let value = self.regs.read16(reg) - 1;
    self.regs.write16(reg, value);
    self.time.tick();
    self.hardware.emulate(self.time);
  }
  // --- Undefined
  fn undefined(&mut self, op: u8) {
    panic!("Undefined opcode {}", op)
  }
  fn undefined_debug(&mut self) {}
}
