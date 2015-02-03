use std::old_io::timer::Timer;
use std::os;
use std::thread::Thread;
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
  let bootrom_path = os::homedir().unwrap().join(".mooneye-gb").join("boot.bin");
  let cartridge_path = Path::new(format!("tests/{}/test.gb", name));
  let hardware_config = config::create_hardware_config(Some(&bootrom_path), &cartridge_path).unwrap();

  let (backend_tx, backend_rx) = backend::new_channel();
  let (machine_tx, machine_rx) = machine::new_channel();

  let emulation_thread = Thread::scoped(move || {
    let shared_memory = AcceptanceSharedMemory;
    let mut mach = Machine::new(&shared_memory, hardware_config);
    let channels = machine::Channels::new(machine_tx, backend_rx);

    mach.set_mode(EmulationMode::MaxSpeed);

    mach.main_loop(channels);
  });

  let max_duration = Duration::seconds(30);
  let mut max_timer = Timer::new().unwrap();
  let deadline = max_timer.oneshot(max_duration);

  let mut registers = None;
  loop {
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
      },
      _ = deadline.recv() => break
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
