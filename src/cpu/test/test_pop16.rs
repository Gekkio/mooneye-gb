use cpu::registers::{Flags, Reg16};
use cpu::test::run_test;

fn test_pop16(opcode: u8, x: u16, reg: Reg16) -> bool {
  let h = (x >> 8) as u8;
  let l = x as u8;
  let cpu = run_test(
    &[opcode, 0xed, l, h],
    |cpu| {
      cpu.regs.sp = 0x0002;
    }
  );
  cpu.clock_cycles() == 12 &&
    cpu.regs.sp == 0x0004 &&
    cpu.regs.read16(reg) == x
}

#[quickcheck]
fn test_c1(x: u16) -> bool {
  test_pop16(0xc1, x, Reg16::BC)
}

#[quickcheck]
fn test_d1(x: u16) -> bool {
  test_pop16(0xd1, x, Reg16::DE)
}

#[quickcheck]
fn test_e1(x: u16) -> bool {
  test_pop16(0xe1, x, Reg16::HL)
}

#[quickcheck]
fn test_f1(a: u8, f: u8) -> bool {
  let cpu = run_test(
    &[0xf1, 0xed, f, a],
    |cpu| {
      cpu.regs.sp = 0x0002;
    }
  );
  cpu.clock_cycles() == 12 &&
    cpu.regs.a == a &&
    cpu.regs.f == Flags::from_bits_truncate(f)
}
