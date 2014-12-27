use cpu::registers::{
  Flags,
  ZERO, ADD_SUBTRACT, HALF_CARRY, CARRY
};
use cpu::test::run_test;

#[test]
fn test_00() {
  let cpu = run_test(
    &[0x00], // NOP
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 4);
}

#[test]
fn test_01() {
  let cpu = run_test(
    &[0x01, 0x42, 0x80], // LD BC, nn
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.b, 0x80);
  assert_eq!(cpu.regs.c, 0x42);
}

#[test]
fn test_02() {
  let cpu = run_test(
    &[0x02, 0xed, 0x00], // LD (BC), A
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.b = 0x00;
      cpu.regs.c = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.hardware.memory[0x02], 0x42);
}

#[test]
fn test_03() {
  let cpu = run_test(
    &[0x03], // INC BC
    |cpu| {
      cpu.regs.b = 0x7f;
      cpu.regs.c = 0xff;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.b, 0x80);
  assert_eq!(cpu.regs.c, 0x00);
}

#[test]
fn test_04() {
  let cpu = run_test(
    &[0x04], // INC B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0x43);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_04_zero() {
  let cpu = run_test(
    &[0x04], // INC B
    |cpu| {
      cpu.regs.b = 0xff;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0x00);
  assert_eq!(cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_04_half_carry() {
  let cpu = run_test(
    &[0x04], // INC B
    |cpu| {
      cpu.regs.b = 0x0f;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0x10);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_05() {
  let cpu = run_test(
    &[0x05], // DEC B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0x41);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_05_zero() {
  let cpu = run_test(
    &[0x05], // DEC B
    |cpu| {
      cpu.regs.b = 0x01;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0x00);
  assert_eq!(cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_05_half_carry() {
  let cpu = run_test(
    &[0x05], // DEC B
    |cpu| {
      cpu.regs.b = 0x00;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0xff);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_06() {
  let cpu = run_test(
    &[0x06, 0x42], // LD B, n
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.b, 0x42);
}

#[test]
fn test_07() {
  let cpu = run_test(
    &[0x07], // RLCA
    |cpu| {
      cpu.regs.a = 0x77;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0xee);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_07_carry() {
  let cpu = run_test(
    &[0x07], // RLCA
    |cpu| {
      cpu.regs.a = 0xf7;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0xef);
  assert_eq!(cpu.regs.f, CARRY);
}

#[test]
fn test_08() {
  let cpu = run_test(
    &[0x08, 0x04, 0x00, 0xed, 0x00, 0x00], // LD (nn), SP
    |cpu| {
      cpu.regs.sp = 0x8042;
    }
  );
  assert_eq!(cpu.clock_cycles(), 20);
  assert_eq!(cpu.hardware.memory[0x04], 0x42);
  assert_eq!(cpu.hardware.memory[0x05], 0x80);
}

#[test]
fn test_09() {
  let cpu = run_test(
    &[0x09], // ADD HL, BC
    |cpu| {
      cpu.regs.b = 0x0f;
      cpu.regs.c = 0xfc;
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x03;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.h, 0x0f);
  assert_eq!(cpu.regs.l, 0xff);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_09_half_carry() {
  let cpu = run_test(
    &[0x09], // ADD HL, BC
    |cpu| {
      cpu.regs.b = 0x06;
      cpu.regs.c = 0x05;
      cpu.regs.h = 0x8a;
      cpu.regs.l = 0x23;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.h, 0x90);
  assert_eq!(cpu.regs.l, 0x28);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_09_carry() {
  let cpu = run_test(
    &[0x09], // ADD HL, BC
    |cpu| {
      cpu.regs.b = 0xb7;
      cpu.regs.c = 0xfd;
      cpu.regs.h = 0x50;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.h, 0x07);
  assert_eq!(cpu.regs.l, 0xff);
  assert_eq!(cpu.regs.f, CARRY);
}

#[test]
fn test_0a() {
  let cpu = run_test(
    &[0x0a, 0xed, 0x42], // LD A, (BC)
    |cpu| {
      cpu.regs.b = 0x00;
      cpu.regs.c = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_0b() {
  let cpu = run_test(
    &[0x0b], // DEC BC
    |cpu| {
      cpu.regs.b = 0x80;
      cpu.regs.c = 0x00;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.b, 0x7f);
  assert_eq!(cpu.regs.c, 0xff);
}

#[test]
fn test_0c() {
  let cpu = run_test(
    &[0x0c], // INC C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0x43);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_0c_zero() {
  let cpu = run_test(
    &[0x0c], // INC C
    |cpu| {
      cpu.regs.c = 0xff;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0x00);
  assert_eq!(cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_0c_half_carry() {
  let cpu = run_test(
    &[0x0c], // INC C
    |cpu| {
      cpu.regs.c = 0x0f;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0x10);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_0d() {
  let cpu = run_test(
    &[0x0d], // DEC C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0x41);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_0d_zero() {
  let cpu = run_test(
    &[0x0d], // DEC C
    |cpu| {
      cpu.regs.c = 0x01;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0x00);
  assert_eq!(cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_0d_half_carry() {
  let cpu = run_test(
    &[0x0d], // DEC C
    |cpu| {
      cpu.regs.c = 0x00;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0xff);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_0e() {
  let cpu = run_test(
    &[0x0e, 0x42], // LD C, n
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.c, 0x42);
}

#[test]
fn test_0f() {
  let cpu = run_test(
    &[0x0f], // RRCA
    |cpu| {
      cpu.regs.a = 0xee;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x77);
  assert!(!cpu.regs.f.contains(CARRY));
}

#[test]
fn test_0f_carry() {
  let cpu = run_test(
    &[0x0f], // RRCA
    |cpu| {
      cpu.regs.a = 0xef;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0xf7);
  assert!(cpu.regs.f.contains(CARRY));
}
