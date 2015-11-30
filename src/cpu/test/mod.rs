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
use cpu::Cpu;
use emulation::{EmuTime, EmuEvents};
use hardware::Bus;
use hardware::irq::Interrupt;

mod cb_test;
mod test_0x;
mod test_1x;
mod test_2x;
mod test_3x;
mod test_4x;
mod test_5x;
mod test_6x;
mod test_7x;
mod test_ax;
mod test_cx;
mod test_ex;
mod test_fx;

mod test_load16;
mod test_load16_sp_hl;
mod test_load16_hl_sp_e;
mod test_push16;
mod test_pop16;
mod test_add16;
mod test_add16_sp_e;
mod test_inc16;
mod test_dec16;

pub struct TestHardware {
  memory: Vec<u8>
}

impl TestHardware {
  fn from_memory(memory: Vec<u8>) -> TestHardware {
    TestHardware {
      memory: memory
    }
  }
}

impl<'a> Bus for TestHardware {
  fn write(&mut self, _: EmuTime, addr: u16, value: u8) {
    self.memory[addr as usize] = value;
  }
  fn read(&self, _: EmuTime, addr: u16) -> u8 {
    self.memory[addr as usize]
  }
  fn emulate(&mut self, _: EmuTime) {}
  fn rewind_time(&mut self) {}
  fn ack_interrupt(&mut self) -> Option<Interrupt> { None }
  fn has_interrupt(&self) -> bool { false }
  fn trigger_emu_events(&mut self, events: EmuEvents) { }
}

pub fn run_test<I: Fn(&mut Cpu<TestHardware>) -> ()>(instructions: &[u8], cpu_init: I) -> Cpu<TestHardware> {
  let mut memory = instructions.to_vec();
  memory.push(0xed);

  let mut cpu = Cpu::new(TestHardware::from_memory(memory));
  cpu_init(&mut cpu);

  while cpu.hardware.memory[cpu.regs.pc as usize] != 0xed {
    cpu.execute();
  }
  cpu
}

#[test]
fn test_disasm_all_opcodes() {
  let bus = TestHardware::from_memory(vec![0x00, 0x00, 0x00]);
  let mut cpu = Cpu::new(bus);

  for op in 0..0xff {
    cpu.hardware.memory[0] = op as u8;
    if op != 0xcb {
      cpu.regs.pc = 0x00;
      cpu.disasm_op();
    } else {
      for cb_op in (0..0xff) {
        cpu.hardware.memory[1] = cb_op as u8;
        cpu.regs.pc = 0x00;
        cpu.disasm_op();
      }
    }
  }
}
