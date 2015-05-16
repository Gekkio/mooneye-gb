use cpu::registers::{
  Flags,
  ZERO, ADD_SUBTRACT, HALF_CARRY, CARRY
};
use cpu::test::run_test;

#[test]
fn test_30() {
  let cpu = run_test(
    &[0x30, 0x01, 0xed, 0xed], // JR NC, e
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.pc, 0x03);
}

#[test]
fn test_30_negative() {
  let cpu = run_test(
    &[0x00, 0xed, 0x30, -3 as u8], // JR NC, e
    |cpu| {
      cpu.regs.pc = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.pc, 0x01);
}

#[test]
fn test_30_nojump() {
  let cpu = run_test(
    &[0x30, 0x01, 0xed, 0x00], // JR NC, e
    |cpu| {
      cpu.regs.f = CARRY;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.pc, 0x02);
}

#[test]
fn test_32() {
  let cpu = run_test(
    &[0x32, 0xed, 0x00], // LDD (HL), A
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.h, 0x00);
  assert_eq!(cpu.regs.l, 0x01);
  assert_eq!(cpu.hardware.memory[0x02], 0x42);
}

#[test]
fn test_34() {
  let cpu = run_test(
    &[0x34, 0xed, 0x42], // INC (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.hardware.memory[0x02], 0x43);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_34_zero() {
  let cpu = run_test(
    &[0x34, 0xed, 0xff], // INC (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.hardware.memory[0x02], 0x00);
  assert_eq!(cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_34_half_carry() {
  let cpu = run_test(
    &[0x34, 0xed, 0x0f], // INC (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.hardware.memory[0x02], 0x10);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_35() {
  let cpu = run_test(
    &[0x35, 0xed, 0x42], // DEC (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.hardware.memory[0x02], 0x41);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_35_zero() {
  let cpu = run_test(
    &[0x35, 0xed, 0x01], // DEC (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.hardware.memory[0x02], 0x00);
  assert_eq!(cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_35_half_carry() {
  let cpu = run_test(
    &[0x35, 0xed, 0x00], // DEC (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.hardware.memory[0x02], 0xff);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_36() {
  let cpu = run_test(
    &[0x36, 0x42, 0xed, 0x00], // LD (HL), n
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x03;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.hardware.memory[0x03], 0x42);
}

#[test]
fn test_37() {
  let cpu = run_test(
    &[0x37], // SCF
    |cpu| {
      cpu.regs.f = ADD_SUBTRACT | HALF_CARRY;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.f, CARRY);
}

#[test]
fn test_38() {
  let cpu = run_test(
    &[0x38, 0x01, 0xed, 0xed], // JR C, e
    |cpu| {
      cpu.regs.f = CARRY;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.pc, 0x03);
}

#[test]
fn test_38_negative() {
  let cpu = run_test(
    &[0x00, 0xed, 0x38, -3 as u8], // JR C, e
    |cpu| {
      cpu.regs.f = CARRY;
      cpu.regs.pc = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.pc, 0x01);
}

#[test]
fn test_38_nojump() {
  let cpu = run_test(
    &[0x38, 0x01, 0xed, 0x00], // JR C, e
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.pc, 0x02);
}

#[test]
fn test_3a() {
  let cpu = run_test(
    &[0x3a, 0xed, 0x42], // LD A, (HL-)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x42);
  assert_eq!(cpu.regs.h, 0x00);
  assert_eq!(cpu.regs.l, 0x01);
}

#[test]
fn test_3c() {
  let cpu = run_test(
    &[0x3c], // INC A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x43);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_3c_zero() {
  let cpu = run_test(
    &[0x3c], // INC A
    |cpu| {
      cpu.regs.a = 0xff;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_3c_half_carry() {
  let cpu = run_test(
    &[0x3c], // INC A
    |cpu| {
      cpu.regs.a = 0x0f;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x10);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_3d() {
  let cpu = run_test(
    &[0x3d], // DEC A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x41);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_3d_zero() {
  let cpu = run_test(
    &[0x3d], // DEC A
    |cpu| {
      cpu.regs.a = 0x01;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_3d_half_carry() {
  let cpu = run_test(
    &[0x3d], // DEC A
    |cpu| {
      cpu.regs.a = 0x00;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0xff);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_3e() {
  let cpu = run_test(
    &[0x3e, 0x42], // LD A, n
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_3f() {
  let cpu = run_test(
    &[0x3f], // CCF
    |cpu| {
      cpu.regs.f = ADD_SUBTRACT | HALF_CARRY;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.f, CARRY);
}

#[test]
fn test_3f_carry() {
  let cpu = run_test(
    &[0x3f], // CCF
    |cpu| {
      cpu.regs.f = ADD_SUBTRACT | HALF_CARRY | CARRY;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.f, Flags::empty());
}
