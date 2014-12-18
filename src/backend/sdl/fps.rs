use std::collections::RingBuf;
use time;

const HISTORY_SIZE: uint = 64;
const RECALC_INTERVAL_SECONDS: f64 = 0.200;

pub struct FpsCounter {
  pub fps: f64,
  history: RingBuf<f64>,
  last_time: f64,
  last_recalc: f64
}

impl FpsCounter {
  pub fn new() -> FpsCounter {
    FpsCounter {
      fps: 0.0,
      history: RingBuf::with_capacity(HISTORY_SIZE),
      last_time: time::precise_time_s(),
      last_recalc: 0.0
    }
  }
  pub fn update(&mut self) {
    let time = time::precise_time_s();

    if self.history.len() >= HISTORY_SIZE {
      let _ = self.history.pop_back();
    }
    self.history.push_front(time - self.last_time);
    self.last_time = time;

    if (time - self.last_recalc) > RECALC_INTERVAL_SECONDS {
      let mut sum = 0.0;
      for history_time in self.history.iter() {
        sum += *history_time;
      }

      self.fps = self.history.len() as f64 / sum;
      self.last_recalc = time;
    }
  }
}
