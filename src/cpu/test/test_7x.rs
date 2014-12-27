use cpu::test::run_test;

#[test]
fn test_70() {
  let cpu = run_test(
    &[0x70, 0xed, 0x00], // LD (HL), B
    |cpu| {
      cpu.regs.b = 0x42;
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.hardware.memory[0x02], 0x42);
}

#[test]
fn test_71() {
  let cpu = run_test(
    &[0x71, 0xed, 0x00], // LD (HL), C
    |cpu| {
      cpu.regs.c = 0x42;
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.hardware.memory[0x02], 0x42);
}

#[test]
fn test_72() {
  let cpu = run_test(
    &[0x72, 0xed, 0x00], // LD (HL), D
    |cpu| {
      cpu.regs.d = 0x42;
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.hardware.memory[0x02], 0x42);
}

#[test]
fn test_73() {
  let cpu = run_test(
    &[0x73, 0xed, 0x00], // LD (HL), E
    |cpu| {
      cpu.regs.e = 0x42;
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.hardware.memory[0x02], 0x42);
}

#[test]
fn test_74() {
  let cpu = run_test(
    &[0x74, 0xed, 0x42], // LD (HL), H
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.hardware.memory[0x02], 0x00);
}

#[test]
fn test_75() {
  let cpu = run_test(
    &[0x75, 0xed, 0x00], // LD (HL), L
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.hardware.memory[0x02], 0x02);
}

#[test]
fn test_76() {
  let cpu = run_test(
    &[0x76], // HALT
    |cpu| {
      cpu.ime = true;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert!(cpu.halt);
}

#[test]
fn test_77() {
  let cpu = run_test(
    &[0x77, 0xed, 0x00], // LD (HL), A
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.hardware.memory[0x02], 0x42);
}

#[test]
fn test_78() {
  let cpu = run_test(
    &[0x78], // LD A, B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_79() {
  let cpu = run_test(
    &[0x79], // LD A, C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_7a() {
  let cpu = run_test(
    &[0x7a], // LD A, D
    |cpu| {
      cpu.regs.d = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_7b() {
  let cpu = run_test(
    &[0x7b], // LD A, E
    |cpu| {
      cpu.regs.e = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_7c() {
  let cpu = run_test(
    &[0x7c], // LD A, H
    |cpu| {
      cpu.regs.h = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_7d() {
  let cpu = run_test(
    &[0x7d], // LD A, L
    |cpu| {
      cpu.regs.l = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_7e() {
  let cpu = run_test(
    &[0x7e, 0xed, 0x42], // LD A, (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_7f() {
  let cpu = run_test(
    &[0x7f], // LD A, A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x42);
}
