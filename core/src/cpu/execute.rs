use crate::cpu::register_file::{Flags, Reg16};
use crate::cpu::{Cond, Cpu, In8, Out8, Step};
use crate::emulation::EmuEvents;
use crate::hardware::Bus;
use crate::util::int::IntExt;

impl Cpu {
  // --- 8-bit operations
  // 8-bit loads
  /// LD d, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn load<B: Bus, O: Out8, I: In8>(&mut self, bus: &mut B, out8: O, in8: I) -> Step {
    let value = in8.read(self, bus);
    out8.write(self, bus, value);
    self.prefetch_next(bus, self.regs.pc)
  }
  // Magic breakpoint
  pub fn ld_b_b<B: Bus>(&mut self, bus: &mut B) -> Step {
    bus.trigger_emu_events(EmuEvents::DEBUG_OP);
    self.prefetch_next(bus, self.regs.pc)
  }
  // 8-bit arithmetic
  /// ADD s
  ///
  /// Flags: Z N H C
  ///        * 0 * *
  pub fn add<B: Bus, I: In8>(&mut self, bus: &mut B, in8: I) -> Step {
    let value = in8.read(self, bus);
    let (result, carry) = self.regs.a.overflowing_add(value);
    let half_carry = (self.regs.a & 0x0f).checked_add(value | 0xf0).is_none();
    self.regs.f =
      Flags::ZERO.test(result == 0) | Flags::CARRY.test(carry) | Flags::HALF_CARRY.test(half_carry);
    self.regs.a = result;
    self.prefetch_next(bus, self.regs.pc)
  }
  /// ADC s
  ///
  /// Flags: Z N H C
  ///        * 0 * *
  pub fn adc<B: Bus, I: In8>(&mut self, bus: &mut B, in8: I) -> Step {
    let value = in8.read(self, bus);
    let cy = if self.regs.f.contains(Flags::CARRY) {
      1
    } else {
      0
    };
    let result = self.regs.a.wrapping_add(value).wrapping_add(cy);
    self.regs.f = Flags::ZERO.test(result == 0)
      | Flags::CARRY.test(self.regs.a as u16 + value as u16 + cy as u16 > 0xff)
      | Flags::HALF_CARRY.test((self.regs.a & 0xf) + (value & 0xf) + cy > 0xf);
    self.regs.a = result;
    self.prefetch_next(bus, self.regs.pc)
  }
  /// SUB s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  pub fn sub<B: Bus, I: In8>(&mut self, bus: &mut B, in8: I) -> Step {
    let value = in8.read(self, bus);
    self.regs.a = self.alu_sub(value, false);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// SBC s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  pub fn sbc<B: Bus, I: In8>(&mut self, bus: &mut B, in8: I) -> Step {
    let value = in8.read(self, bus);
    self.regs.a = self.alu_sub(value, true);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// CP s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  pub fn cp<B: Bus, I: In8>(&mut self, bus: &mut B, in8: I) -> Step {
    let value = in8.read(self, bus);
    self.alu_sub(value, false);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// AND s
  ///
  /// Flags: Z N H C
  ///        * 0 1 0
  pub fn and<B: Bus, I: In8>(&mut self, bus: &mut B, in8: I) -> Step {
    let value = in8.read(self, bus);
    self.regs.a &= value;
    self.regs.f = Flags::ZERO.test(self.regs.a == 0) | Flags::HALF_CARRY;
    self.prefetch_next(bus, self.regs.pc)
  }
  /// OR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  pub fn or<B: Bus, I: In8>(&mut self, bus: &mut B, in8: I) -> Step {
    let value = in8.read(self, bus);
    self.regs.a |= value;
    self.regs.f = Flags::ZERO.test(self.regs.a == 0);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// XOR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  pub fn xor<B: Bus, I: In8>(&mut self, bus: &mut B, in8: I) -> Step {
    let value = in8.read(self, bus);
    self.regs.a ^= value;
    self.regs.f = Flags::ZERO.test(self.regs.a == 0);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// INC s
  ///
  /// Flags: Z N H C
  ///        * 0 * -
  pub fn inc<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, io: IO) -> Step {
    let value = io.read(self, bus);
    let new_value = value.wrapping_add(1);
    self.regs.f = Flags::ZERO.test(new_value == 0)
      | Flags::HALF_CARRY.test(value & 0xf == 0xf)
      | (Flags::CARRY & self.regs.f);
    io.write(self, bus, new_value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// DEC s
  ///
  /// Flags: Z N H C
  ///        * 1 * -
  pub fn dec<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, io: IO) -> Step {
    let value = io.read(self, bus);
    let new_value = value.wrapping_sub(1);
    self.regs.f = Flags::ZERO.test(new_value == 0)
      | Flags::ADD_SUBTRACT
      | Flags::HALF_CARRY.test(value & 0xf == 0)
      | (Flags::CARRY & self.regs.f);
    io.write(self, bus, new_value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RLCA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  pub fn rlca<B: Bus>(&mut self, bus: &mut B) -> Step {
    let value = self.regs.a;
    self.regs.a = self.alu_rlc(value, false);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RLA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  pub fn rla<B: Bus>(&mut self, bus: &mut B) -> Step {
    let value = self.regs.a;
    self.regs.a = self.alu_rl(value, false);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RRCA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  pub fn rrca<B: Bus>(&mut self, bus: &mut B) -> Step {
    let value = self.regs.a;
    self.regs.a = self.alu_rrc(value, false);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RRA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  pub fn rra<B: Bus>(&mut self, bus: &mut B) -> Step {
    let value = self.regs.a;
    self.regs.a = self.alu_rr(value, false);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RLC s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn rlc<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, io: IO) -> Step {
    let value = io.read(self, bus);
    let new_value = self.alu_rlc(value, true);
    io.write(self, bus, new_value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RL s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn rl<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, io: IO) -> Step {
    let value = io.read(self, bus);
    let new_value = self.alu_rl(value, true);
    io.write(self, bus, new_value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RRC s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn rrc<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, io: IO) -> Step {
    let value = io.read(self, bus);
    let new_value = self.alu_rrc(value, true);
    io.write(self, bus, new_value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn rr<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, io: IO) -> Step {
    let value = io.read(self, bus);
    let new_value = self.alu_rr(value, true);
    io.write(self, bus, new_value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// SLA s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn sla<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, io: IO) -> Step {
    let value = io.read(self, bus);
    let co = value & 0x80;
    let new_value = value << 1;
    self.regs.f = Flags::ZERO.test(new_value == 0) | Flags::CARRY.test(co != 0);
    io.write(self, bus, new_value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// SRA s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn sra<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, io: IO) -> Step {
    let value = io.read(self, bus);
    let co = value & 0x01;
    let hi = value & 0x80;
    let new_value = (value >> 1) | hi;
    self.regs.f = Flags::ZERO.test(new_value == 0) | Flags::CARRY.test(co != 0);
    io.write(self, bus, new_value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// SRL s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn srl<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, io: IO) -> Step {
    let value = io.read(self, bus);
    let co = value & 0x01;
    let new_value = value >> 1;
    self.regs.f = Flags::ZERO.test(new_value == 0) | Flags::CARRY.test(co != 0);
    io.write(self, bus, new_value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// SWAP s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  pub fn swap<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, io: IO) -> Step {
    let value = io.read(self, bus);
    let new_value = (value >> 4) | (value << 4);
    self.regs.f = Flags::ZERO.test(value == 0);
    io.write(self, bus, new_value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// BIT b, s
  ///
  /// Flags: Z N H C
  ///        * 0 1 -
  pub fn bit<B: Bus, I: In8>(&mut self, bus: &mut B, bit: usize, in8: I) -> Step {
    let value = in8.read(self, bus) & (1 << bit);
    self.regs.f = Flags::ZERO.test(value == 0) | Flags::HALF_CARRY | (Flags::CARRY & self.regs.f);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// SET b, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn set<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, bit: usize, io: IO) -> Step {
    let value = io.read(self, bus) | (1 << bit);
    io.write(self, bus, value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RES b, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn res<B: Bus, IO: In8 + Out8>(&mut self, bus: &mut B, bit: usize, io: IO) -> Step {
    let value = io.read(self, bus) & !(1 << bit);
    io.write(self, bus, value);
    self.prefetch_next(bus, self.regs.pc)
  }
  // --- Control
  /// JP nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn jp<B: Bus>(&mut self, bus: &mut B) -> Step {
    let addr = self.next_u16(bus);
    self.ctrl_jp(bus, addr);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// JP HL
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn jp_hl<B: Bus>(&mut self, bus: &mut B) -> Step {
    self.prefetch_next(bus, self.regs.read16(Reg16::HL))
  }
  /// JR e
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn jr<B: Bus>(&mut self, bus: &mut B) -> Step {
    let offset = self.next_u8(bus) as i8;
    self.ctrl_jr(bus, offset);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// CALL nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn call<B: Bus>(&mut self, bus: &mut B) -> Step {
    let addr = self.next_u16(bus);
    self.ctrl_call(bus, addr);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RET
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn ret<B: Bus>(&mut self, bus: &mut B) -> Step {
    self.ctrl_ret(bus);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RETI
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn reti<B: Bus>(&mut self, bus: &mut B) -> Step {
    self.ime = true;
    self.ctrl_ret(bus);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// JP cc, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn jp_cc<B: Bus>(&mut self, bus: &mut B, cond: Cond) -> Step {
    let addr = self.next_u16(bus);
    if cond.check(self.regs.f) {
      self.ctrl_jp(bus, addr);
    }
    self.prefetch_next(bus, self.regs.pc)
  }
  /// JR cc, e
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn jr_cc<B: Bus>(&mut self, bus: &mut B, cond: Cond) -> Step {
    let offset = self.next_u8(bus) as i8;
    if cond.check(self.regs.f) {
      self.ctrl_jr(bus, offset);
    }
    self.prefetch_next(bus, self.regs.pc)
  }
  /// CALL cc, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn call_cc<B: Bus>(&mut self, bus: &mut B, cond: Cond) -> Step {
    let addr = self.next_u16(bus);
    if cond.check(self.regs.f) {
      self.ctrl_call(bus, addr);
    }
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RET cc
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn ret_cc<B: Bus>(&mut self, bus: &mut B, cond: Cond) -> Step {
    bus.tick_cycle();
    if cond.check(self.regs.f) {
      self.ctrl_ret(bus);
    }
    self.prefetch_next(bus, self.regs.pc)
  }
  /// RST n
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn rst<B: Bus>(&mut self, bus: &mut B, addr: u8) -> Step {
    let pc = self.regs.pc;
    bus.tick_cycle();
    self.push_u16(bus, pc);
    self.prefetch_next(bus, addr as u16)
  }
  // --- Miscellaneous
  /// HALT
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn halt<B: Bus>(&mut self, bus: &mut B) -> Step {
    self.opcode = bus.read_cycle(self.regs.pc);
    if !bus.get_mid_interrupt().is_empty() {
      if self.ime {
        Step::InterruptDispatch
      } else {
        self.decode_exec_fetch(bus)
      }
    } else {
      bus.tick_cycle();
      Step::Halt
    }
  }

  /// STOP
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn stop<B: Bus>(&mut self, _: &mut B) -> Step {
    panic!("STOP")
  }
  /// DI
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn di<B: Bus>(&mut self, bus: &mut B) -> Step {
    self.ime = false;
    self.opcode = self.next_u8(bus);
    Step::Running
  }
  /// EI
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn ei<B: Bus>(&mut self, bus: &mut B) -> Step {
    let step = self.prefetch_next(bus, self.regs.pc);
    self.ime = true;
    step
  }
  /// CCF
  ///
  /// Flags: Z N H C
  ///        - 0 0 *
  pub fn ccf<B: Bus>(&mut self, bus: &mut B) -> Step {
    self.regs.f =
      (Flags::ZERO & self.regs.f) | Flags::CARRY.test(!self.regs.f.contains(Flags::CARRY));
    self.prefetch_next(bus, self.regs.pc)
  }
  /// SCF
  ///
  /// Flags: Z N H C
  ///        - 0 0 1
  pub fn scf<B: Bus>(&mut self, bus: &mut B) -> Step {
    self.regs.f = (Flags::ZERO & self.regs.f) | Flags::CARRY;
    self.prefetch_next(bus, self.regs.pc)
  }
  /// NOP
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn nop<B: Bus>(&mut self, bus: &mut B) -> Step {
    self.prefetch_next(bus, self.regs.pc)
  }
  /// DAA
  ///
  /// Flags: Z N H C
  ///        * - 0 *
  pub fn daa<B: Bus>(&mut self, bus: &mut B) -> Step {
    // DAA table in page 110 of the official "Game Boy Programming Manual"
    let mut carry = false;
    if !self.regs.f.contains(Flags::ADD_SUBTRACT) {
      if self.regs.f.contains(Flags::CARRY) || self.regs.a > 0x99 {
        self.regs.a = self.regs.a.wrapping_add(0x60);
        carry = true;
      }
      if self.regs.f.contains(Flags::HALF_CARRY) || self.regs.a & 0x0f > 0x09 {
        self.regs.a = self.regs.a.wrapping_add(0x06);
      }
    } else if self.regs.f.contains(Flags::CARRY) {
      carry = true;
      self.regs.a = self
        .regs
        .a
        .wrapping_add(if self.regs.f.contains(Flags::HALF_CARRY) {
          0x9a
        } else {
          0xa0
        });
    } else if self.regs.f.contains(Flags::HALF_CARRY) {
      self.regs.a = self.regs.a.wrapping_add(0xfa);
    }

    self.regs.f = Flags::ZERO.test(self.regs.a == 0)
      | (Flags::ADD_SUBTRACT & self.regs.f)
      | Flags::CARRY.test(carry);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// CPL
  ///
  /// Flags: Z N H C
  ///        - 1 1 -
  pub fn cpl<B: Bus>(&mut self, bus: &mut B) -> Step {
    self.regs.a = !self.regs.a;
    self.regs.f = (Flags::ZERO & self.regs.f)
      | Flags::ADD_SUBTRACT
      | Flags::HALF_CARRY
      | (Flags::CARRY & self.regs.f);
    self.prefetch_next(bus, self.regs.pc)
  }
  // --- 16-bit operations
  // 16-bit loads
  /// LD dd, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn load16_imm<B: Bus>(&mut self, bus: &mut B, reg: Reg16) -> Step {
    let value = self.next_u16(bus);
    self.regs.write16(reg, value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// LD (nn), SP
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn load16_nn_sp<B: Bus>(&mut self, bus: &mut B) -> Step {
    let value = self.regs.sp;
    let addr = self.next_u16(bus);
    bus.write_cycle(addr, value as u8);
    bus.write_cycle(addr.wrapping_add(1), (value >> 8) as u8);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// LD SP, HL
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn load16_sp_hl<B: Bus>(&mut self, bus: &mut B) -> Step {
    let value = self.regs.read16(Reg16::HL);
    self.regs.sp = value;
    bus.tick_cycle();
    self.prefetch_next(bus, self.regs.pc)
  }
  /// LD HL, SP+e
  ///
  /// Flags: Z N H C
  ///        0 0 * *
  pub fn load16_hl_sp_e<B: Bus>(&mut self, bus: &mut B) -> Step {
    let offset = self.next_u8(bus) as i8 as u16;
    let sp = self.regs.sp as u16;
    let value = sp.wrapping_add(offset);
    self.regs.write16(Reg16::HL, value);
    self.regs.f = Flags::HALF_CARRY.test(u16::test_add_carry_bit(3, sp, offset))
      | Flags::CARRY.test(u16::test_add_carry_bit(7, sp, offset));
    bus.tick_cycle();
    self.prefetch_next(bus, self.regs.pc)
  }
  /// PUSH rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn push16<B: Bus>(&mut self, bus: &mut B, reg: Reg16) -> Step {
    let value = self.regs.read16(reg);
    bus.tick_cycle();
    self.push_u16(bus, value);
    self.prefetch_next(bus, self.regs.pc)
  }
  /// POP rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  /// Note! POP AF affects all flags
  pub fn pop16<B: Bus>(&mut self, bus: &mut B, reg: Reg16) -> Step {
    let value = self.pop_u16(bus);
    self.regs.write16(reg, value);
    self.prefetch_next(bus, self.regs.pc)
  }
  // 16-bit arithmetic
  /// ADD HL, ss
  ///
  /// Flags: Z N H C
  ///        - 0 * *
  pub fn add16<B: Bus>(&mut self, bus: &mut B, reg: Reg16) -> Step {
    let hl = self.regs.read16(Reg16::HL);
    let value = self.regs.read16(reg);
    let result = hl.wrapping_add(value);
    self.regs.f = (Flags::ZERO & self.regs.f)
      | Flags::HALF_CARRY.test(u16::test_add_carry_bit(11, hl, value))
      | Flags::CARRY.test(hl > 0xffff - value);
    self.regs.write16(Reg16::HL, result);
    bus.tick_cycle();
    self.prefetch_next(bus, self.regs.pc)
  }
  /// ADD SP, e
  ///
  /// Flags: Z N H C
  ///        0 0 * *
  pub fn add16_sp_e<B: Bus>(&mut self, bus: &mut B) -> Step {
    let val = self.next_u8(bus) as i8 as i16 as u16;
    let sp = self.regs.sp;
    self.regs.sp = sp.wrapping_add(val);
    self.regs.f = Flags::HALF_CARRY.test(u16::test_add_carry_bit(3, sp, val))
      | Flags::CARRY.test(u16::test_add_carry_bit(7, sp, val));
    bus.tick_cycle();
    bus.tick_cycle();
    self.prefetch_next(bus, self.regs.pc)
  }
  /// INC rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn inc16<B: Bus>(&mut self, bus: &mut B, reg: Reg16) -> Step {
    let value = self.regs.read16(reg).wrapping_add(1);
    self.regs.write16(reg, value);
    bus.tick_cycle();
    self.prefetch_next(bus, self.regs.pc)
  }
  /// DEC rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn dec16<B: Bus>(&mut self, bus: &mut B, reg: Reg16) -> Step {
    let value = self.regs.read16(reg).wrapping_sub(1);
    self.regs.write16(reg, value);
    bus.tick_cycle();
    self.prefetch_next(bus, self.regs.pc)
  }
  // --- Undefined
  pub fn undefined<B: Bus>(&mut self, _: &mut B) -> Step {
    panic!("Undefined opcode {}", self.opcode)
  }
  pub fn cb_prefix<B: Bus>(&mut self, bus: &mut B) -> Step {
    self.opcode = self.next_u8(bus);
    self.cb_decode_exec_fetch(bus)
  }
}
