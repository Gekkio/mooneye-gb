use clock_ticks::precise_time_ns;
use std::env;
use std::thread;
use std::path::PathBuf;
use std::time::duration::Duration;

use backend;
use backend::{Backend, BackendMessage, BackendSharedMemory};
use config;
use gameboy;
use machine;
use machine::{EmulationMode, Machine, MachineMessage};

struct AcceptanceSharedMemory;

impl BackendSharedMemory for AcceptanceSharedMemory {
  fn draw_screen(&self, _: &gameboy::ScreenBuffer) {
  }
}

pub fn run_acceptance_test(name: &str) {
  let bootrom_path = env::home_dir().unwrap().join(".mooneye-gb").join("boot.bin");
  let test_name = format!("tests/{}/test.gb", name);
  let cartridge_path = PathBuf::from(&test_name);
  let hardware_config = config::create_hardware_config(Some(&bootrom_path), &cartridge_path).unwrap();

  let (backend_tx, backend_rx) = backend::new_channel();
  let (machine_tx, machine_rx) = machine::new_channel();

  let emulation_thread = thread::spawn(move || {
    let shared_memory = AcceptanceSharedMemory;
    let mut mach = Machine::new(&shared_memory, hardware_config);
    let channels = machine::Channels::new(machine_tx, backend_rx);

    mach.set_mode(EmulationMode::MaxSpeed);

    mach.main_loop(channels);
  });

  let max_duration = Duration::seconds(30);
  let start_time = precise_time_ns();

  let mut registers = None;
  loop {
    let time = precise_time_ns();
    if Duration::nanoseconds((time - start_time) as i64) > max_duration {
      break;
    }
    select!(
      machine_event = machine_rx.recv() => {
        match machine_event {
          Err(_) => break,
          Ok(MachineMessage::Debug(regs)) => {
            registers = Some(regs);
            break
          },
          _ => ()
        }
      }
    )
  }
  backend_tx.send(BackendMessage::Quit).unwrap();
  emulation_thread.join();
  match registers {
    None => panic!("Test did not finish"),
    Some(regs) => {
      if regs.a != 0 {
        panic!("{} assertion failures in hardware test", regs.a);
      }
      if regs.b != 3  || regs.c != 5  ||
         regs.d != 8  || regs.e != 13 ||
         regs.h != 21 || regs.l != 34 {
        panic!("Hardware test failed");
      }
    }
  }
}
