use cpu::registers::{
  Flags, Reg16, HALF_CARRY, CARRY
};
use cpu::test::run_test;

fn test_load16_hl_sp_e<F: Fn(Flags) -> bool>(sp: u16, e: i8, check_flags: F) -> bool {
  let cpu = run_test(
    &[0xf8, e as u8],
    |cpu| {
      cpu.regs.write16(Reg16::SP, sp);
    }
  );
  let expected = sp.wrapping_add(e as i16 as u16);
  cpu.clock_cycles() == 12 &&
    cpu.regs.read16(Reg16::HL) == expected &&
    check_flags(cpu.regs.f)
}

#[quickcheck]
fn test_f8(sp: u16, e: i8) -> bool {
  test_load16_hl_sp_e(sp, e, |_| true)
}

#[test]
fn test_f8_overflow_inc() {
  assert!(test_load16_hl_sp_e(0xffff, 1, |f| f == HALF_CARRY | CARRY));
}

#[test]
fn test_f8_overflow_dec() {
  assert!(test_load16_hl_sp_e(0x0000, -1, |f| f == Flags::empty()));
}
