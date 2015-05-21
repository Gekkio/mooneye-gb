use quickcheck::quickcheck;

use cpu::registers::Reg16;
use cpu::test::run_test;

#[test]
fn test_f9() {
  fn prop(hl: u16) -> bool {
    let cpu = run_test(
      &[0xf9],
      |cpu| {
        cpu.regs.write16(Reg16::HL, hl);
      }
    );
    cpu.clock_cycles() == 8 &&
      cpu.regs.read16(Reg16::SP) == hl
  }
  quickcheck(prop as fn(u16) -> bool);
}
