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
  ZERO, ADD_SUBTRACT, HALF_CARRY
};
use cpu::test::run_test;

#[test]
fn test_20() {
  let cpu = run_test(
    &[0x20, 0x01, 0xed, 0xed], // JR NZ, e
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.pc, 0x03);
}

#[test]
fn test_20_negative() {
  let cpu = run_test(
    &[0x00, 0xed, 0x20, -3i8 as u8], // JR NZ, e
    |cpu| {
      cpu.regs.pc = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.pc, 0x01);
}

#[test]
fn test_20_nojump() {
  let cpu = run_test(
    &[0x20, 0x01, 0xed, 0x00], // JR NZ, e
    |cpu| {
      cpu.regs.f = ZERO;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.pc, 0x02);
}

#[test]
fn test_22() {
  let cpu = run_test(
    &[0x22, 0xed, 0x00], // LDI (HL), A
    |cpu| {
      cpu.regs.a = 0x42;
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.h, 0x00);
  assert_eq!(cpu.regs.l, 0x03);
  assert_eq!(cpu.hardware.memory[0x02], 0x42);
}

#[test]
fn test_24() {
  let cpu = run_test(
    &[0x24], // INC H
    |cpu| {
      cpu.regs.h = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x43);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_24_zero() {
  let cpu = run_test(
    &[0x24], // INC H
    |cpu| {
      cpu.regs.h = 0xff;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x00);
  assert_eq!(cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_24_half_carry() {
  let cpu = run_test(
    &[0x24], // INC H
    |cpu| {
      cpu.regs.h = 0x0f;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x10);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_25() {
  let cpu = run_test(
    &[0x25], // DEC H
    |cpu| {
      cpu.regs.h = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x41);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_25_zero() {
  let cpu = run_test(
    &[0x25], // DEC H
    |cpu| {
      cpu.regs.h = 0x01;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x00);
  assert_eq!(cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_25_half_carry() {
  let cpu = run_test(
    &[0x25], // DEC H
    |cpu| {
      cpu.regs.h = 0x00;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0xff);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_26() {
  let cpu = run_test(
    &[0x26, 0x42], // LD H, n
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.h, 0x42);
}

#[test]
fn test_28() {
  let cpu = run_test(
    &[0x28, 0x01, 0xed, 0xed], // JR Z, e
    |cpu| {
      cpu.regs.f = ZERO;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.pc, 0x03);
}

#[test]
fn test_28_negative() {
  let cpu = run_test(
    &[0x00, 0xed, 0x28, -3i8 as u8], // JR Z, e
    |cpu| {
      cpu.regs.f = ZERO;
      cpu.regs.pc = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.pc, 0x01);
}

#[test]
fn test_28_nojump() {
  let cpu = run_test(
    &[0x28, 0x01, 0xed, 0x00], // JR Z, e
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.pc, 0x02);
}

#[test]
fn test_2a() {
  let cpu = run_test(
    &[0x2a, 0xed, 0x42], // LD A, (HL+)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x42);
  assert_eq!(cpu.regs.h, 0x00);
  assert_eq!(cpu.regs.l, 0x03);
}

#[test]
fn test_2c() {
  let cpu = run_test(
    &[0x2c], // INC L
    |cpu| {
      cpu.regs.l = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x43);
  assert_eq!(cpu.regs.f, Flags::empty());
}

#[test]
fn test_2c_zero() {
  let cpu = run_test(
    &[0x2c], // INC L
    |cpu| {
      cpu.regs.l = 0xff;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x00);
  assert_eq!(cpu.regs.f, ZERO | HALF_CARRY);
}

#[test]
fn test_2c_half_carry() {
  let cpu = run_test(
    &[0x2c], // INC L
    |cpu| {
      cpu.regs.l = 0x0f;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x10);
  assert_eq!(cpu.regs.f, HALF_CARRY);
}

#[test]
fn test_2d() {
  let cpu = run_test(
    &[0x2d], // DEC L
    |cpu| {
      cpu.regs.l = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x41);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT);
}

#[test]
fn test_2d_zero() {
  let cpu = run_test(
    &[0x2d], // DEC L
    |cpu| {
      cpu.regs.l = 0x01;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x00);
  assert_eq!(cpu.regs.f, ZERO | ADD_SUBTRACT);
}

#[test]
fn test_2d_half_carry() {
  let cpu = run_test(
    &[0x2d], // DEC L
    |cpu| {
      cpu.regs.l = 0x00;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0xff);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}

#[test]
fn test_2e() {
  let cpu = run_test(
    &[0x2e, 0x42], // LD L, n
    |_| {}
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.l, 0x42);
}

#[test]
fn test_2f() {
  let cpu = run_test(
    &[0x2f], // CPL
    |cpu| {
      cpu.regs.a = 0xaa;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.a, 0x55);
  assert_eq!(cpu.regs.f, ADD_SUBTRACT | HALF_CARRY);
}
