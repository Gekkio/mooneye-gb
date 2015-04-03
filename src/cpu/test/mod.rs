use std::iter;
use test::Bencher;

use cpu::Cpu;
use emulation::EmuTime;
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
// mod test_8x;
// mod test_9x;
mod test_ax;
// mod test_bx;
mod test_cx;
mod test_dx;
mod test_ex;
mod test_fx;

struct TestHardware {
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
  fn write(&mut self, time: EmuTime, addr: u16, value: u8) {
    self.memory[addr as usize] = value;
  }
  fn read(&self, time: EmuTime, addr: u16) -> u8 {
    self.memory[addr as usize]
  }
  fn emulate(&mut self, time: EmuTime) {}
  fn rewind_time(&mut self) {}
  fn ack_interrupt(&mut self) -> Option<Interrupt> { None }
  fn has_interrupt(&self) -> bool { false }
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

#[bench]
fn turbo(b: &mut Bencher) {
  let bus = TestHardware::from_memory(iter::repeat(0x0A).take(8192).collect());
  let mut cpu = Cpu::new(bus);
  cpu.regs.h = 0x00;
  cpu.regs.l = 0x00;
  cpu.regs.a = 0x01;
  b.iter(|| {
    cpu.regs.pc = 0x01;
    for _ in (0..8191) {
      cpu.execute();
    }
  })
}

#[test]
fn test_disasm_all_opcodes() {
  let bus = TestHardware::from_memory(vec![0x00, 0x00, 0x00]);
  let mut cpu = Cpu::new(bus);

  for op in (0..0xff) {
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
