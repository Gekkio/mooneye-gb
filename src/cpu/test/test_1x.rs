// This file is part of Mooneye GB.
// Copyright (C) 2014-2015 Joonas Javanainen <joonas.javanainen@gmail.com>
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
fn test_12() {
  let cpu = run_test(
    &[0x12, 0xed, 0x00], // LD (DE), A
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.d = 0x00;
      cpu.regs.e = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.hardware.memory[0x02], 0x42);
}

#[test]
fn test_14() {
  let cpu = run_test(
    &[0x14], // INC D
    |cpu| {
      cpu.regs.d = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x43);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_14_zero() {
  let cpu = run_test(
    &[0x14], // INC D
    |cpu| {
      cpu.regs.d = 0xff;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x00);
  assert_eq!(cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_14_half_carry() {
  let cpu = run_test(
    &[0x14], // INC D
    |cpu| {
      cpu.regs.d = 0x0f;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x10);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_15() {
  let cpu = run_test(
    &[0x15], // DEC D
    |cpu| {
      cpu.regs.d = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x41);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_15_zero() {
  let cpu = run_test(
    &[0x15], // DEC D
    |cpu| {
      cpu.regs.d = 0x01;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0x00);
  assert_eq!(cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_15_half_carry() {
  let cpu = run_test(
    &[0x15], // DEC D
    |cpu| {
      cpu.regs.d = 0x00;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.d, 0xff);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_16() {
  let cpu = run_test(
    &[0x16, 0x42], // LD D, n
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.d, 0x42);
}

#[test]
fn test_17() {
  let cpu = run_test(
    &[0x17], // RLA
    |cpu| {
      cpu.regs.a = 0x55;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0xaa);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_17_carry() {
  let cpu = run_test(
    &[0x17], // RLA
    |cpu| {
      cpu.regs.a = 0xaa;
      cpu.regs.f = CARRY;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x55);
  assert_eq!(cpu.regs.f, CARRY);
}

#[test]
fn test_1a() {
  let cpu = run_test(
    &[0x1a, 0xed, 0x42], // LD A, (DE)
    |cpu| {
      cpu.regs.d = 0x00;
      cpu.regs.e = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_1c() {
  let cpu = run_test(
    &[0x1c], // INC E
    |cpu| {
      cpu.regs.e = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x43);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_1c_zero() {
  let cpu = run_test(
    &[0x1c], // INC E
    |cpu| {
      cpu.regs.e = 0xff;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x00);
  assert_eq!(cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_1c_half_carry() {
  let cpu = run_test(
    &[0x1c], // INC E
    |cpu| {
      cpu.regs.e = 0x0f;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x10);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_1d() {
  let cpu = run_test(
    &[0x1d], // DEC E
    |cpu| {
      cpu.regs.e = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x41);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_1d_zero() {
  let cpu = run_test(
    &[0x1d], // DEC E
    |cpu| {
      cpu.regs.e = 0x01;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0x00);
  assert_eq!(cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_1d_half_carry() {
  let cpu = run_test(
    &[0x1d], // DEC E
    |cpu| {
      cpu.regs.e = 0x00;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.e, 0xff);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_1e() {
  let cpu = run_test(
    &[0x1e, 0x42], // LD E, n
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.e, 0x42);
}

#[test]
fn test_1f() {
  let cpu = run_test(
    &[0x1f], // RRA
    |cpu| {
      cpu.regs.a = 0xaa;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x55);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_1f_carry() {
  let cpu = run_test(
    &[0x1f], // RRA
    |cpu| {
      cpu.regs.a = 0x55;
      cpu.regs.f = CARRY;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0xaa);
  assert_eq!(cpu.regs.f, CARRY);
}
