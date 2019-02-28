use failure::{format_err, Error};
use mooneye_gb::emulation::{EmuEvents, EmuTime};
use mooneye_gb::machine::Machine;
use mooneye_gb::{GbKey, ScreenBuffer, CPU_SPEED_HZ, SCREEN_EMPTY};
use std::mem;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::frame_times::FrameTimes;
use crate::perf_counter::PerfCounter;

#[derive(Debug)]
pub struct EmuThreadHandle {
  pub join_handle: thread::JoinHandle<Result<(Machine, PerfCounter), Error>>,
  pub sender: mpsc::Sender<Request>,
  pub receiver: mpsc::Receiver<Tick>,
}

impl EmuThreadHandle {
  pub fn key_down(&self, key: GbKey) -> Result<(), Error> {
    self.sender.send(Request::KeyDown(key))?;
    Ok(())
  }
  pub fn key_up(&self, key: GbKey) -> Result<(), Error> {
    self.sender.send(Request::KeyUp(key))?;
    Ok(())
  }
  pub fn next_tick(&self, buffer: Box<ScreenBuffer>) -> Result<(), Error> {
    self.sender.send(Request::NextTick(buffer))?;
    Ok(())
  }
  pub fn stop(self) -> Result<(Machine, PerfCounter), Error> {
    self.sender.send(Request::Stop)?;
    self
      .join_handle
      .join()
      .map_err(|_| format_err!("Emu thread panicked"))?
  }
  pub fn check_tick(&self) -> Option<Tick> {
    self.receiver.try_iter().next()
  }
}

pub enum Request {
  NextTick(Box<ScreenBuffer>),
  Stop,
  KeyDown(GbKey),
  KeyUp(GbKey),
}

pub struct Tick {
  pub screen_buffer: Box<ScreenBuffer>,
  pub screen_buffer_updated: bool,
  pub cycles_per_s: f64,
}

fn run(
  mut machine: Machine,
  mut perf_counter: PerfCounter,
  receiver: &mpsc::Receiver<Request>,
  sender: &mpsc::Sender<Tick>,
) -> Result<(Machine, PerfCounter), Error> {
  let mut emu_time = machine.emu_time();
  let mut times = FrameTimes::new(Duration::from_secs(1) / 60);
  let mut screen_buffer = Box::new(SCREEN_EMPTY);
  let mut screen_buffer_updated = false;
  loop {
    let delta = times.update();
    let delta_s = delta.as_secs() as f64 + f64::from(delta.subsec_nanos()) / 1_000_000_000.0;

    for request in receiver.try_iter() {
      match request {
        Request::NextTick(mut buffer) => {
          let cycles_per_s = perf_counter.get_machine_cycles_per_s();
          if screen_buffer_updated {
            mem::swap(&mut buffer, &mut screen_buffer);
            sender.send(Tick {
              screen_buffer: buffer,
              screen_buffer_updated: true,
              cycles_per_s,
            })?;
          } else {
            sender.send(Tick {
              screen_buffer: buffer,
              screen_buffer_updated: false,
              cycles_per_s,
            })?;
          }
        }
        Request::KeyDown(key) => machine.key_down(key),
        Request::KeyUp(key) => machine.key_up(key),
        Request::Stop => return Ok((machine, perf_counter)),
      }
    }

    let machine_cycles = EmuTime::from_machine_cycles(CPU_SPEED_HZ as u64 / 60 / 4);
    let target_time = emu_time + machine_cycles;
    loop {
      let (events, end_time) = machine.emulate(target_time);

      if events.contains(EmuEvents::VSYNC) {
        screen_buffer.copy_from_slice(machine.screen_buffer());
        screen_buffer_updated = true;
      }

      if end_time >= target_time {
        perf_counter.update(end_time - emu_time, delta_s);
        emu_time = end_time;
        break;
      }
    }
  }
}

pub fn spawn(machine: Machine, perf_counter: PerfCounter) -> EmuThreadHandle {
  let (tx_sender, tx_receiver) = mpsc::channel();
  let (rx_sender, rx_receiver) = mpsc::channel();
  let join_handle = thread::spawn(move || run(machine, perf_counter, &tx_receiver, &rx_sender));
  EmuThreadHandle {
    join_handle,
    sender: tx_sender,
    receiver: rx_receiver,
  }
}
