use cpu::test::run_test;

#[test]
fn test_50() {
  let cpu = run_test(
    &[0x50], // LD D, B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x42);
}

#[test]
fn test_51() {
  let cpu = run_test(
    &[0x51], // LD D, C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x42);
}

#[test]
fn test_52() {
  let cpu = run_test(
    &[0x52], // LD D, D
    |cpu| {
      cpu.regs.d = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x42);
}

#[test]
fn test_53() {
  let cpu = run_test(
    &[0x53], // LD D, E
    |cpu| {
      cpu.regs.e = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x42);
}

#[test]
fn test_54() {
  let cpu = run_test(
    &[0x54], // LD D, H
    |cpu| {
      cpu.regs.h = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x42);
}

#[test]
fn test_55() {
  let cpu = run_test(
    &[0x55], // LD D, L
    |cpu| {
      cpu.regs.l = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x42);
}

#[test]
fn test_56() {
  let cpu = run_test(
    &[0x56, 0xed, 0x42], // LD D, (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.d, 0x42);
}

#[test]
fn test_57() {
  let cpu = run_test(
    &[0x57], // LD D, A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x42);
}

#[test]
fn test_58() {
  let cpu = run_test(
    &[0x58], // LD E, B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x42);
}

#[test]
fn test_59() {
  let cpu = run_test(
    &[0x59], // LD E, C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x42);
}

#[test]
fn test_5a() {
  let cpu = run_test(
    &[0x5a], // LD E, D
    |cpu| {
      cpu.regs.d = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x42);
}

#[test]
fn test_5b() {
  let cpu = run_test(
    &[0x5b], // LD E, E
    |cpu| {
      cpu.regs.e = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x42);
}

#[test]
fn test_5c() {
  let cpu = run_test(
    &[0x5c], // LD E, H
    |cpu| {
      cpu.regs.h = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x42);
}

#[test]
fn test_5d() {
  let cpu = run_test(
    &[0x5d], // LD E, L
    |cpu| {
      cpu.regs.l = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x42);
}

#[test]
fn test_5e() {
  let cpu = run_test(
    &[0x5e, 0xed, 0x42], // LD E, (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.e, 0x42);
}

#[test]
fn test_5f() {
  let cpu = run_test(
    &[0x5f], // LD E, A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x42);
}
