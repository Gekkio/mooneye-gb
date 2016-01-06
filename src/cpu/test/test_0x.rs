// This file is part of Mooneye GB.
// Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
//
// Mooneye GB is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Mooneye GB is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Mooneye GB.  If not, see <http://www.gnu.org/licenses/>.
use cpu::registers::{
  Flags,
  ZERO, ADD_SUBTRACT, HALF_CARRY, CARRY
};
use cpu::test::run_test;

#[test]
fn test_00() {
  let cpu = run_test(
    &[0x00], // NOP
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 4);
}

#[test]
fn test_02() {
  let cpu = run_test(
    &[0x02, 0xed, 0x00], // LD (BC), A
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.b = 0x00;
      cpu.regs.c = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.hardware.memory[0x02], 0x42);
}

#[test]
fn test_04() {
  let cpu = run_test(
    &[0x04], // INC B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0x43);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_04_zero() {
  let cpu = run_test(
    &[0x04], // INC B
    |cpu| {
      cpu.regs.b = 0xff;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0x00);
  assert_eq!(cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_04_half_carry() {
  let cpu = run_test(
    &[0x04], // INC B
    |cpu| {
      cpu.regs.b = 0x0f;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0x10);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_05() {
  let cpu = run_test(
    &[0x05], // DEC B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0x41);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_05_zero() {
  let cpu = run_test(
    &[0x05], // DEC B
    |cpu| {
      cpu.regs.b = 0x01;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0x00);
  assert_eq!(cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_05_half_carry() {
  let cpu = run_test(
    &[0x05], // DEC B
    |cpu| {
      cpu.regs.b = 0x00;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.b, 0xff);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_06() {
  let cpu = run_test(
    &[0x06, 0x42], // LD B, n
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.b, 0x42);
}

#[test]
fn test_07() {
  let cpu = run_test(
    &[0x07], // RLCA
    |cpu| {
      cpu.regs.a = 0x77;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0xee);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_07_carry() {
  let cpu = run_test(
    &[0x07], // RLCA
    |cpu| {
      cpu.regs.a = 0xf7;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0xef);
  assert_eq!(cpu.regs.f, CARRY);
}

#[test]
fn test_0a() {
  let cpu = run_test(
    &[0x0a, 0xed, 0x42], // LD A, (BC)
    |cpu| {
      cpu.regs.b = 0x00;
      cpu.regs.c = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_0c() {
  let cpu = run_test(
    &[0x0c], // INC C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0x43);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_0c_zero() {
  let cpu = run_test(
    &[0x0c], // INC C
    |cpu| {
      cpu.regs.c = 0xff;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0x00);
  assert_eq!(cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_0c_half_carry() {
  let cpu = run_test(
    &[0x0c], // INC C
    |cpu| {
      cpu.regs.c = 0x0f;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0x10);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_0d() {
  let cpu = run_test(
    &[0x0d], // DEC C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0x41);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_0d_zero() {
  let cpu = run_test(
    &[0x0d], // DEC C
    |cpu| {
      cpu.regs.c = 0x01;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0x00);
  assert_eq!(cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_0d_half_carry() {
  let cpu = run_test(
    &[0x0d], // DEC C
    |cpu| {
      cpu.regs.c = 0x00;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.c, 0xff);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_0e() {
  let cpu = run_test(
    &[0x0e, 0x42], // LD C, n
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.c, 0x42);
}

#[test]
fn test_0f() {
  let cpu = run_test(
    &[0x0f], // RRCA
    |cpu| {
      cpu.regs.a = 0xee;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x77);
  assert!(!cpu.regs.f.contains(CARRY));
}

#[test]
fn test_0f_carry() {
  let cpu = run_test(
    &[0x0f], // RRCA
    |cpu| {
      cpu.regs.a = 0xef;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0xf7);
  assert!(cpu.regs.f.contains(CARRY));
}
