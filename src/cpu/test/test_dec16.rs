use cpu::registers::Reg16;
use cpu::test::run_test;

fn test_dec16(opcode: u8, x: u16, reg: Reg16) -> bool {
  let cpu = run_test(
    &[opcode],
    |cpu| {
      cpu.regs.write16(reg, x);
    }
  );
  let expected = x.wrapping_sub(1);
  cpu.clock_cycles() == 8 &&
    cpu.regs.read16(reg) == expected
}

#[quickcheck]
fn test_0b(x: u16) -> bool {
  test_dec16(0x0b, x, Reg16::BC)
}

#[test]
fn test_0b_overflow() {
  assert!(test_dec16(0x0b, 0x0000, Reg16::BC))
}

#[quickcheck]
fn test_1b(x: u16) -> bool {
  test_dec16(0x1b, x, Reg16::DE)
}

#[test]
fn test_1b_overflow() {
  assert!(test_dec16(0x1b, 0x0000, Reg16::DE))
}

#[quickcheck]
fn test_2b(x: u16) -> bool {
  test_dec16(0x2b, x, Reg16::HL)
}

#[test]
fn test_2b_overflow() {
  assert!(test_dec16(0x2b, 0x0000, Reg16::HL))
}

#[quickcheck]
fn test_3b(x: u16) -> bool {
  test_dec16(0x3b, x, Reg16::SP)
}

#[test]
fn test_3b_overflow() {
  assert!(test_dec16(0x3b, 0x0000, Reg16::SP))
}

