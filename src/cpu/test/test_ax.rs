use cpu::registers::{
  Flags,
  ZERO, ADD_SUBTRACT, HALF_CARRY, CARRY
};
use cpu::test::run_test;

#[test]
fn test_a8() {
  let cpu = run_test(
    &[0xa8], // XOR B
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.b = 0x38;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x7a);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_a8_zero() {
  let cpu = run_test(
    &[0xa8], // XOR B
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO);
}

#[test]
fn test_a9() {
  let cpu = run_test(
    &[0xa9], // XOR C
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.c = 0x38;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x7a);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_a9_zero() {
  let cpu = run_test(
    &[0xa9], // XOR C
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO);
}

#[test]
fn test_aa() {
  let cpu = run_test(
    &[0xaa], // XOR D
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.d = 0x38;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x7a);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_aa_zero() {
  let cpu = run_test(
    &[0xaa], // XOR D
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.d = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO);
}

#[test]
fn test_ab() {
  let cpu = run_test(
    &[0xab], // XOR E
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.e = 0x38;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x7a);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_ab_zero() {
  let cpu = run_test(
    &[0xab], // XOR E
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.e = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO);
}

#[test]
fn test_ac() {
  let cpu = run_test(
    &[0xac], // XOR H
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.h = 0x38;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x7a);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_ac_zero() {
  let cpu = run_test(
    &[0xac], // XOR H
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.h = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO);
}

#[test]
fn test_ad() {
  let cpu = run_test(
    &[0xad], // XOR L
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.l = 0x38;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x7a);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_ad_zero() {
  let cpu = run_test(
    &[0xad], // XOR L
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.l = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO);
}

#[test]
fn test_ae() {
  let cpu = run_test(
    &[0xae, 0xed, 0x38], // XOR (HL)
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x7a);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_ae_zero() {
  let cpu = run_test(
    &[0xae, 0xed, 0x42], // XOR (HL)
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO);
}

#[test]
fn test_af() {
  let cpu = run_test(
    &[0xaf], // XOR A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO);
}
