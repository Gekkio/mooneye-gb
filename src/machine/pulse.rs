use snooze::Snooze;
use std::comm::sync_channel;
use std::time::Duration;

pub fn start(duration: Duration) -> Receiver<()> {
  let (tx, rx) = sync_channel(1);
  spawn(move || {
    let mut snooze = Snooze::new(duration).unwrap();
    loop {
      snooze.wait().unwrap();
      if let Err(_) = tx.send_opt(()) { break }
    }
  });
  rx
}
