use cpu::test::run_test;

#[test]
fn test_d1() {
  let cpu = run_test(
    &[0xd1, 0xed, 0x80, 0x42], // POP DE
    |cpu| {
      cpu.regs.sp = 0x02;
    }
  );
  assert_eq!(cpu.hardware.cycles, 12);
  assert_eq!(cpu.regs.d, 0x42);
  assert_eq!(cpu.regs.e, 0x80);
}

#[test]
fn test_d5() {
  let cpu = run_test(
    &[0xd5, 0xed, 0x00, 0x00], // PUSH DE
    |cpu| {
      cpu.regs.d = 0x42;
      cpu.regs.e = 0x80;
      cpu.regs.sp = 0x04;
    }
  );
  assert_eq!(cpu.hardware.cycles, 16);
  assert_eq!(cpu.regs.sp, 0x02);
  assert_eq!(cpu.hardware.memory[0x03], 0x42);
  assert_eq!(cpu.hardware.memory[0x02], 0x80);
}

