use cpu::test::run_test;

#[test]
fn test_c1() {
  let cpu = run_test(
    &[0xc1, 0xed, 0x80, 0x42], // POP BC
    |cpu| {
      cpu.regs.sp = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.b, 0x42);
  assert_eq!(cpu.regs.c, 0x80);
}

#[test]
fn test_c3() {
  let cpu = run_test(
    &[0xc3, 0x04, 0x00, 0xed], // JP nn
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 16);
  assert_eq!(cpu.regs.pc, 0x04);
}

#[test]
fn test_c5() {
  let cpu = run_test(
    &[0xc5, 0xed, 0x00, 0x00], // PUSH BC
    |cpu| {
      cpu.regs.b = 0x42;
      cpu.regs.c = 0x80;
      cpu.regs.sp = 0x04;
    }
  );
  assert_eq!(cpu.clock_cycles(), 16);
  assert_eq!(cpu.regs.sp, 0x02);
  assert_eq!(cpu.hardware.memory[0x03], 0x42);
  assert_eq!(cpu.hardware.memory[0x02], 0x80);
}

