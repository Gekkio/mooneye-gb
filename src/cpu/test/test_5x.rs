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
