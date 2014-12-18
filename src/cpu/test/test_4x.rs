use cpu::test::run_test;

#[test]
fn test_40() {
  let cpu = run_test(
    &[0x40], // LD B, B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.b, 0x42);
}

#[test]
fn test_41() {
  let cpu = run_test(
    &[0x41], // LD B, C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.b, 0x42);
}

#[test]
fn test_42() {
  let cpu = run_test(
    &[0x42], // LD B, D
    |cpu| {
      cpu.regs.d = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.b, 0x42);
}

#[test]
fn test_43() {
  let cpu = run_test(
    &[0x43], // LD B, E
    |cpu| {
      cpu.regs.e = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.b, 0x42);
}

#[test]
fn test_44() {
  let cpu = run_test(
    &[0x44], // LD B, H
    |cpu| {
      cpu.regs.h = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.b, 0x42);
}

#[test]
fn test_45() {
  let cpu = run_test(
    &[0x45], // LD B, L
    |cpu| {
      cpu.regs.l = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.b, 0x42);
}

#[test]
fn test_46() {
  let cpu = run_test(
    &[0x46, 0xed, 0x42], // LD B, (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.hardware.cycles, 8);
  assert_eq!(cpu.regs.b, 0x42);
}

#[test]
fn test_47() {
  let cpu = run_test(
    &[0x47], // LD B, A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.b, 0x42);
}

#[test]
fn test_48() {
  let cpu = run_test(
    &[0x48], // LD C, B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.c, 0x42);
}

#[test]
fn test_49() {
  let cpu = run_test(
    &[0x49], // LD C, C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.c, 0x42);
}

#[test]
fn test_4a() {
  let cpu = run_test(
    &[0x4a], // LD C, D
    |cpu| {
      cpu.regs.d = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.c, 0x42);
}

#[test]
fn test_4b() {
  let cpu = run_test(
    &[0x4b], // LD C, E
    |cpu| {
      cpu.regs.e = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.c, 0x42);
}

#[test]
fn test_4c() {
  let cpu = run_test(
    &[0x4c], // LD C, H
    |cpu| {
      cpu.regs.h = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.c, 0x42);
}

#[test]
fn test_4d() {
  let cpu = run_test(
    &[0x4d], // LD C, L
    |cpu| {
      cpu.regs.l = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.c, 0x42);
}

#[test]
fn test_4e() {
  let cpu = run_test(
    &[0x4e, 0xed, 0x42], // LD C, (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.hardware.cycles, 8);
  assert_eq!(cpu.regs.c, 0x42);
}

#[test]
fn test_4f() {
  let cpu = run_test(
    &[0x4f], // LD C, A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.hardware.cycles, 4);
  assert_eq!(cpu.regs.c, 0x42);
}
