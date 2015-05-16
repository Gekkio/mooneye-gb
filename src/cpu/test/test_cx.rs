use cpu::test::run_test;

#[test]
fn test_c3() {
  let cpu = run_test(
    &[0xc3, 0x04, 0x00, 0xed], // JP nn
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 16);
  assert_eq!(cpu.regs.pc, 0x04);
}

