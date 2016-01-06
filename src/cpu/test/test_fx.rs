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
use cpu::ImeChange;
use cpu::test::run_test;

#[test]
fn test_f0() {
  let cpu = run_test(
    &[0xf0, 0x80], // LDH A, (n)
    |cpu| {
      cpu.write_hiram(0x00, 0x42);
    }
  );
  assert_eq!(cpu.clock_cycles(), 12);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_f2() {
  let cpu = run_test(
    &[0xf2], // LDH A, (C)
    |cpu| {
      cpu.write_hiram(0x00, 0x42);
      cpu.regs.c = 0x80;
    }
  );
  assert_eq!(cpu.clock_cycles(), 8);
  assert_eq!(cpu.regs.a, 0x42);
}

#[test]
fn test_f3() {
  let cpu = run_test(
    &[0xf3], // DI
    |cpu| {
      cpu.ime = true;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.ime, false);
}

#[test]
fn test_fb() {
  let cpu = run_test(
    &[0xfb], // EI
    |cpu| {
      cpu.ime = false;
    }
  );
  assert_eq!(cpu.clock_cycles(), 4);
  assert_eq!(cpu.ime_change, ImeChange::Soon);
}
