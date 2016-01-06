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
fn test_60() {
  let cpu = run_test(
    &[0x60], // LD H, B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x42);
}

#[test]
fn test_61() {
  let cpu = run_test(
    &[0x61], // LD H, C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x42);
}

#[test]
fn test_62() {
  let cpu = run_test(
    &[0x62], // LD H, D
    |cpu| {
      cpu.regs.d = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x42);
}

#[test]
fn test_63() {
  let cpu = run_test(
    &[0x63], // LD H, E
    |cpu| {
      cpu.regs.e = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x42);
}

#[test]
fn test_64() {
  let cpu = run_test(
    &[0x64], // LD H, H
    |cpu| {
      cpu.regs.h = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x42);
}

#[test]
fn test_65() {
  let cpu = run_test(
    &[0x65], // LD H, L
    |cpu| {
      cpu.regs.l = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x42);
}

#[test]
fn test_66() {
  let cpu = run_test(
    &[0x66, 0xed, 0x42], // LD H, (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.h, 0x42);
}

#[test]
fn test_67() {
  let cpu = run_test(
    &[0x67], // LD H, A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.h, 0x42);
}

#[test]
fn test_68() {
  let cpu = run_test(
    &[0x68], // LD L, B
    |cpu| {
      cpu.regs.b = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x42);
}

#[test]
fn test_69() {
  let cpu = run_test(
    &[0x69], // LD L, C
    |cpu| {
      cpu.regs.c = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x42);
}

#[test]
fn test_6a() {
  let cpu = run_test(
    &[0x6a], // LD L, D
    |cpu| {
      cpu.regs.d = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x42);
}

#[test]
fn test_6b() {
  let cpu = run_test(
    &[0x6b], // LD L, E
    |cpu| {
      cpu.regs.e = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x42);
}

#[test]
fn test_6c() {
  let cpu = run_test(
    &[0x6c], // LD L, H
    |cpu| {
      cpu.regs.h = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x42);
}

#[test]
fn test_6d() {
  let cpu = run_test(
    &[0x6d], // LD L, L
    |cpu| {
      cpu.regs.l = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x42);
}

#[test]
fn test_6e() {
  let cpu = run_test(
    &[0x6e, 0xed, 0x42], // LD L, (HL)
    |cpu| {
      cpu.regs.h = 0x00;
      cpu.regs.l = 0x02;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.l, 0x42);
}

#[test]
fn test_6f() {
  let cpu = run_test(
    &[0x6f], // LD L, A
    |cpu| {
      cpu.regs.a = 0x42;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.regs.l, 0x42);
}
