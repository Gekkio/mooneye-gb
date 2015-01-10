use snooze::Snooze;
use std::sync::mpsc::{Receiver, sync_channel};
use std::thread::Thread;
use std::time::Duration;

pub fn start(duration: Duration) -> Receiver<()> {
  let (tx, rx) = sync_channel(1);
  Thread::spawn(move || {
    let mut snooze = Snooze::new(duration).unwrap();
    loop {
      snooze.wait().unwrap();
      if let Err(_) = tx.send(()) { break }
    }
  });
  rx
}
