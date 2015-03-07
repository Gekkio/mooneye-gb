use snooze::Snooze;
use std::sync::mpsc::{Receiver, sync_channel};
use std::thread;
use std::time::Duration;

pub fn start(duration: Duration) -> Receiver<()> {
  let (tx, rx) = sync_channel(1);
  thread::spawn(move || {
    let mut snooze = Snooze::new(duration).unwrap();
    loop {
      snooze.wait().unwrap();
      if let Err(_) = tx.send(()) { break }
    }
  });
  rx
}
