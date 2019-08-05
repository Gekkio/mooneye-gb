use crate::cpu::register_file::{Flags, Reg16};
use crate::cpu::{Cond, Cpu, CpuContext, In8, Out8, Step};
use crate::util::int::IntExt;

impl Cpu {
  // --- 8-bit operations
  // 8-bit loads
  /// LD d, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn load<B: CpuContext, O: Out8, I: In8>(&mut self, ctx: &mut B, out8: O, in8: I) -> Step {
    let value = in8.read(self, ctx);
    out8.write(self, ctx, value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  // Magic breakpoint
  pub fn ld_b_b<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    ctx.debug_opcode_callback();
    self.prefetch_next(ctx, self.regs.pc)
  }
  // 8-bit arithmetic
  /// ADD s
  ///
  /// Flags: Z N H C
  ///        * 0 * *
  pub fn add<B: CpuContext, I: In8>(&mut self, ctx: &mut B, in8: I) -> Step {
    let value = in8.read(self, ctx);
    let (result, carry) = self.regs.a.overflowing_add(value);
    let half_carry = (self.regs.a & 0x0f).checked_add(value | 0xf0).is_none();
    self.regs.f =
      Flags::ZERO.test(result == 0) | Flags::CARRY.test(carry) | Flags::HALF_CARRY.test(half_carry);
    self.regs.a = result;
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// ADC s
  ///
  /// Flags: Z N H C
  ///        * 0 * *
  pub fn adc<B: CpuContext, I: In8>(&mut self, ctx: &mut B, in8: I) -> Step {
    let value = in8.read(self, ctx);
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
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// SUB s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  pub fn sub<B: CpuContext, I: In8>(&mut self, ctx: &mut B, in8: I) -> Step {
    let value = in8.read(self, ctx);
    self.regs.a = self.alu_sub(value, false);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// SBC s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  pub fn sbc<B: CpuContext, I: In8>(&mut self, ctx: &mut B, in8: I) -> Step {
    let value = in8.read(self, ctx);
    self.regs.a = self.alu_sub(value, true);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// CP s
  ///
  /// Flags: Z N H C
  ///        * 1 * *
  pub fn cp<B: CpuContext, I: In8>(&mut self, ctx: &mut B, in8: I) -> Step {
    let value = in8.read(self, ctx);
    self.alu_sub(value, false);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// AND s
  ///
  /// Flags: Z N H C
  ///        * 0 1 0
  pub fn and<B: CpuContext, I: In8>(&mut self, ctx: &mut B, in8: I) -> Step {
    let value = in8.read(self, ctx);
    self.regs.a &= value;
    self.regs.f = Flags::ZERO.test(self.regs.a == 0) | Flags::HALF_CARRY;
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// OR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  pub fn or<B: CpuContext, I: In8>(&mut self, ctx: &mut B, in8: I) -> Step {
    let value = in8.read(self, ctx);
    self.regs.a |= value;
    self.regs.f = Flags::ZERO.test(self.regs.a == 0);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// XOR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  pub fn xor<B: CpuContext, I: In8>(&mut self, ctx: &mut B, in8: I) -> Step {
    let value = in8.read(self, ctx);
    self.regs.a ^= value;
    self.regs.f = Flags::ZERO.test(self.regs.a == 0);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// INC s
  ///
  /// Flags: Z N H C
  ///        * 0 * -
  pub fn inc<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, io: IO) -> Step {
    let value = io.read(self, ctx);
    let new_value = value.wrapping_add(1);
    self.regs.f = Flags::ZERO.test(new_value == 0)
      | Flags::HALF_CARRY.test(value & 0xf == 0xf)
      | (Flags::CARRY & self.regs.f);
    io.write(self, ctx, new_value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// DEC s
  ///
  /// Flags: Z N H C
  ///        * 1 * -
  pub fn dec<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, io: IO) -> Step {
    let value = io.read(self, ctx);
    let new_value = value.wrapping_sub(1);
    self.regs.f = Flags::ZERO.test(new_value == 0)
      | Flags::ADD_SUBTRACT
      | Flags::HALF_CARRY.test(value & 0xf == 0)
      | (Flags::CARRY & self.regs.f);
    io.write(self, ctx, new_value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RLCA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  pub fn rlca<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let value = self.regs.a;
    self.regs.a = self.alu_rlc(value, false);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RLA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  pub fn rla<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let value = self.regs.a;
    self.regs.a = self.alu_rl(value, false);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RRCA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  pub fn rrca<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let value = self.regs.a;
    self.regs.a = self.alu_rrc(value, false);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RRA
  ///
  /// Flags: Z N H C
  ///        0 0 0 *
  pub fn rra<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let value = self.regs.a;
    self.regs.a = self.alu_rr(value, false);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RLC s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn rlc<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, io: IO) -> Step {
    let value = io.read(self, ctx);
    let new_value = self.alu_rlc(value, true);
    io.write(self, ctx, new_value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RL s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn rl<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, io: IO) -> Step {
    let value = io.read(self, ctx);
    let new_value = self.alu_rl(value, true);
    io.write(self, ctx, new_value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RRC s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn rrc<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, io: IO) -> Step {
    let value = io.read(self, ctx);
    let new_value = self.alu_rrc(value, true);
    io.write(self, ctx, new_value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RR s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn rr<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, io: IO) -> Step {
    let value = io.read(self, ctx);
    let new_value = self.alu_rr(value, true);
    io.write(self, ctx, new_value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// SLA s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn sla<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, io: IO) -> Step {
    let value = io.read(self, ctx);
    let co = value & 0x80;
    let new_value = value << 1;
    self.regs.f = Flags::ZERO.test(new_value == 0) | Flags::CARRY.test(co != 0);
    io.write(self, ctx, new_value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// SRA s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn sra<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, io: IO) -> Step {
    let value = io.read(self, ctx);
    let co = value & 0x01;
    let hi = value & 0x80;
    let new_value = (value >> 1) | hi;
    self.regs.f = Flags::ZERO.test(new_value == 0) | Flags::CARRY.test(co != 0);
    io.write(self, ctx, new_value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// SRL s
  ///
  /// Flags: Z N H C
  ///        * 0 0 *
  pub fn srl<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, io: IO) -> Step {
    let value = io.read(self, ctx);
    let co = value & 0x01;
    let new_value = value >> 1;
    self.regs.f = Flags::ZERO.test(new_value == 0) | Flags::CARRY.test(co != 0);
    io.write(self, ctx, new_value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// SWAP s
  ///
  /// Flags: Z N H C
  ///        * 0 0 0
  pub fn swap<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, io: IO) -> Step {
    let value = io.read(self, ctx);
    let new_value = (value >> 4) | (value << 4);
    self.regs.f = Flags::ZERO.test(value == 0);
    io.write(self, ctx, new_value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// BIT b, s
  ///
  /// Flags: Z N H C
  ///        * 0 1 -
  pub fn bit<B: CpuContext, I: In8>(&mut self, ctx: &mut B, bit: usize, in8: I) -> Step {
    let value = in8.read(self, ctx) & (1 << bit);
    self.regs.f = Flags::ZERO.test(value == 0) | Flags::HALF_CARRY | (Flags::CARRY & self.regs.f);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// SET b, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn set<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, bit: usize, io: IO) -> Step {
    let value = io.read(self, ctx) | (1 << bit);
    io.write(self, ctx, value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RES b, s
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn res<B: CpuContext, IO: In8 + Out8>(&mut self, ctx: &mut B, bit: usize, io: IO) -> Step {
    let value = io.read(self, ctx) & !(1 << bit);
    io.write(self, ctx, value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  // --- Control
  /// JP nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn jp<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let addr = self.next_u16(ctx);
    self.ctrl_jp(ctx, addr);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// JP HL
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn jp_hl<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    self.prefetch_next(ctx, self.regs.read16(Reg16::HL))
  }
  /// JR e
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn jr<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let offset = self.next_u8(ctx) as i8;
    self.ctrl_jr(ctx, offset);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// CALL nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn call<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let addr = self.next_u16(ctx);
    self.ctrl_call(ctx, addr);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RET
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn ret<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    self.ctrl_ret(ctx);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RETI
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn reti<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    self.ime = true;
    self.ctrl_ret(ctx);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// JP cc, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn jp_cc<B: CpuContext>(&mut self, ctx: &mut B, cond: Cond) -> Step {
    let addr = self.next_u16(ctx);
    if cond.check(self.regs.f) {
      self.ctrl_jp(ctx, addr);
    }
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// JR cc, e
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn jr_cc<B: CpuContext>(&mut self, ctx: &mut B, cond: Cond) -> Step {
    let offset = self.next_u8(ctx) as i8;
    if cond.check(self.regs.f) {
      self.ctrl_jr(ctx, offset);
    }
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// CALL cc, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn call_cc<B: CpuContext>(&mut self, ctx: &mut B, cond: Cond) -> Step {
    let addr = self.next_u16(ctx);
    if cond.check(self.regs.f) {
      self.ctrl_call(ctx, addr);
    }
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RET cc
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn ret_cc<B: CpuContext>(&mut self, ctx: &mut B, cond: Cond) -> Step {
    ctx.tick_cycle();
    if cond.check(self.regs.f) {
      self.ctrl_ret(ctx);
    }
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// RST n
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn rst<B: CpuContext>(&mut self, ctx: &mut B, addr: u8) -> Step {
    let pc = self.regs.pc;
    ctx.tick_cycle();
    self.push_u16(ctx, pc);
    self.prefetch_next(ctx, addr as u16)
  }
  // --- Miscellaneous
  /// HALT
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn halt<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    self.opcode = ctx.read_cycle(self.regs.pc);
    if !ctx.get_mid_interrupt().is_empty() {
      if self.ime {
        Step::InterruptDispatch
      } else {
        self.decode_exec_fetch(ctx)
      }
    } else {
      ctx.tick_cycle();
      Step::Halt
    }
  }

  /// STOP
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn stop<B: CpuContext>(&mut self, _: &mut B) -> Step {
    panic!("STOP")
  }
  /// DI
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn di<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    self.ime = false;
    self.opcode = self.next_u8(ctx);
    Step::Running
  }
  /// EI
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn ei<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let step = self.prefetch_next(ctx, self.regs.pc);
    self.ime = true;
    step
  }
  /// CCF
  ///
  /// Flags: Z N H C
  ///        - 0 0 *
  pub fn ccf<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    self.regs.f =
      (Flags::ZERO & self.regs.f) | Flags::CARRY.test(!self.regs.f.contains(Flags::CARRY));
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// SCF
  ///
  /// Flags: Z N H C
  ///        - 0 0 1
  pub fn scf<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    self.regs.f = (Flags::ZERO & self.regs.f) | Flags::CARRY;
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// NOP
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn nop<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// DAA
  ///
  /// Flags: Z N H C
  ///        * - 0 *
  pub fn daa<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
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
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// CPL
  ///
  /// Flags: Z N H C
  ///        - 1 1 -
  pub fn cpl<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    self.regs.a = !self.regs.a;
    self.regs.f = (Flags::ZERO & self.regs.f)
      | Flags::ADD_SUBTRACT
      | Flags::HALF_CARRY
      | (Flags::CARRY & self.regs.f);
    self.prefetch_next(ctx, self.regs.pc)
  }
  // --- 16-bit operations
  // 16-bit loads
  /// LD dd, nn
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn load16_imm<B: CpuContext>(&mut self, ctx: &mut B, reg: Reg16) -> Step {
    let value = self.next_u16(ctx);
    self.regs.write16(reg, value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// LD (nn), SP
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn load16_nn_sp<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let value = self.regs.sp;
    let addr = self.next_u16(ctx);
    ctx.write_cycle(addr, value as u8);
    ctx.write_cycle(addr.wrapping_add(1), (value >> 8) as u8);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// LD SP, HL
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn load16_sp_hl<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let value = self.regs.read16(Reg16::HL);
    self.regs.sp = value;
    ctx.tick_cycle();
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// LD HL, SP+e
  ///
  /// Flags: Z N H C
  ///        0 0 * *
  pub fn load16_hl_sp_e<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let offset = self.next_u8(ctx) as i8 as u16;
    let sp = self.regs.sp as u16;
    let value = sp.wrapping_add(offset);
    self.regs.write16(Reg16::HL, value);
    self.regs.f = Flags::HALF_CARRY.test(u16::test_add_carry_bit(3, sp, offset))
      | Flags::CARRY.test(u16::test_add_carry_bit(7, sp, offset));
    ctx.tick_cycle();
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// PUSH rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn push16<B: CpuContext>(&mut self, ctx: &mut B, reg: Reg16) -> Step {
    let value = self.regs.read16(reg);
    ctx.tick_cycle();
    self.push_u16(ctx, value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// POP rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  /// Note! POP AF affects all flags
  pub fn pop16<B: CpuContext>(&mut self, ctx: &mut B, reg: Reg16) -> Step {
    let value = self.pop_u16(ctx);
    self.regs.write16(reg, value);
    self.prefetch_next(ctx, self.regs.pc)
  }
  // 16-bit arithmetic
  /// ADD HL, ss
  ///
  /// Flags: Z N H C
  ///        - 0 * *
  pub fn add16<B: CpuContext>(&mut self, ctx: &mut B, reg: Reg16) -> Step {
    let hl = self.regs.read16(Reg16::HL);
    let value = self.regs.read16(reg);
    let result = hl.wrapping_add(value);
    self.regs.f = (Flags::ZERO & self.regs.f)
      | Flags::HALF_CARRY.test(u16::test_add_carry_bit(11, hl, value))
      | Flags::CARRY.test(hl > 0xffff - value);
    self.regs.write16(Reg16::HL, result);
    ctx.tick_cycle();
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// ADD SP, e
  ///
  /// Flags: Z N H C
  ///        0 0 * *
  pub fn add16_sp_e<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    let val = self.next_u8(ctx) as i8 as i16 as u16;
    let sp = self.regs.sp;
    self.regs.sp = sp.wrapping_add(val);
    self.regs.f = Flags::HALF_CARRY.test(u16::test_add_carry_bit(3, sp, val))
      | Flags::CARRY.test(u16::test_add_carry_bit(7, sp, val));
    ctx.tick_cycle();
    ctx.tick_cycle();
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// INC rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn inc16<B: CpuContext>(&mut self, ctx: &mut B, reg: Reg16) -> Step {
    let value = self.regs.read16(reg).wrapping_add(1);
    self.regs.write16(reg, value);
    ctx.tick_cycle();
    self.prefetch_next(ctx, self.regs.pc)
  }
  /// DEC rr
  ///
  /// Flags: Z N H C
  ///        - - - -
  pub fn dec16<B: CpuContext>(&mut self, ctx: &mut B, reg: Reg16) -> Step {
    let value = self.regs.read16(reg).wrapping_sub(1);
    self.regs.write16(reg, value);
    ctx.tick_cycle();
    self.prefetch_next(ctx, self.regs.pc)
  }
  // --- Undefined
  pub fn undefined<B: CpuContext>(&mut self, _: &mut B) -> Step {
    panic!("Undefined opcode {}", self.opcode)
  }
  pub fn cb_prefix<B: CpuContext>(&mut self, ctx: &mut B) -> Step {
    self.opcode = self.next_u8(ctx);
    self.cb_decode_exec_fetch(ctx)
  }
}
