use cpu::ImeChange;
use cpu::test::run_test;

#[test]
fn test_f0() {
  let cpu = run_test(
    &[0xf0, 0x80], // LDH A, (n)
    |cpu| {
      cpu.write_hiram(0x00, 0x42);
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_f1() {
  let cpu = run_test(
    &[0xf1, 0xed, 0x80, 0x42], // POP AF
    |cpu| {
      cpu.regs.sp = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.a, 0x42);
  assert_eq!(cpu.regs.f.get(), 0x80);
}

#[test]
fn test_f2() {
  let cpu = run_test(
    &[0xf2], // LDH A, (C)
    |cpu| {
      cpu.write_hiram(0x00, 0x42);
      cpu.regs.c = 0x80;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_f3() {
  let cpu = run_test(
    &[0xf3], // DI
    |cpu| {
      cpu.ime = true;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.ime_change, ImeChange::Now(false));
}

#[test]
fn test_f5() {
  let cpu = run_test(
    &[0xf5, 0xed, 0x00, 0x00], // PUSH AF
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.f.set(0x80);
      cpu.regs.sp = 0x04;
    }
  );
  assert_eq!(cpu.clock_cycles(), 16);
  assert_eq!(cpu.regs.sp, 0x02);
  assert_eq!(cpu.hardware.memory[0x03], 0x42);
  assert_eq!(cpu.hardware.memory[0x02], 0x80);
}

#[test]
fn test_fb() {
  let cpu = run_test(
    &[0xfb], // EI
    |cpu| {
      cpu.ime = false;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.ime_change, ImeChange::Soon(true));
}
