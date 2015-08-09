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
use std::collections::VecDeque;
use time::precise_time_s;

const HISTORY_SIZE: usize = 64;
const RECALC_INTERVAL_SECONDS: f64 = 0.200;

pub struct FpsCounter {
  pub fps: f64,
  history: VecDeque<f64>,
  last_time: f64,
  last_recalc: f64
}

impl FpsCounter {
  pub fn new() -> FpsCounter {
    FpsCounter {
      fps: 0.0,
      history: VecDeque::with_capacity(HISTORY_SIZE),
      last_time: precise_time_s(),
      last_recalc: 0.0
    }
  }
  pub fn update(&mut self) {
    let time = precise_time_s();

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
