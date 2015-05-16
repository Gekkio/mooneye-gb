use cpu::registers::{
  Flags, ZERO
};
use cpu::test::run_test;

#[test]
fn test_e0() {
  let cpu = run_test(
    &[0xe0, 0x80], // LDH (n), A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.read_hiram(0x00), 0x42);
}

#[test]
fn test_e2() {
  let cpu = run_test(
    &[0xe2], // LD (C), A
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.c = 0x80;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.read_hiram(0x00), 0x42);
}

#[test]
fn test_ea() {
  let cpu = run_test(
    &[0xea, 0x04, 0x00, 0xed, 0x00], // LD (nn), A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 16);
  assert_eq!(cpu.hardware.memory[0x04], 0x42);
}

#[test]
fn test_ee() {
  let cpu = run_test(
    &[0xee, 0x38], // XOR (n)
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x7a);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_ee_zero() {
  let cpu = run_test(
    &[0xee, 0x42], // XOR (n)
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x00);
  assert_eq!(cpu.regs.f, ZERO);
}
