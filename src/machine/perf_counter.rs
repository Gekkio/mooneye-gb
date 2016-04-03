// This file is part of Mooneye GB.
// Copyright (C) 2014-2016 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use time::SteadyTime;

use emulation::EmuDuration;

const HISTORY_SIZE: usize = 128;

/// A cycles-per-second counter
pub struct PerfCounter {
  history: VecDeque<f64>,
  last_time: SteadyTime
}

impl PerfCounter {
  pub fn new() -> PerfCounter {
    PerfCounter {
      history: VecDeque::with_capacity(HISTORY_SIZE),
      last_time: SteadyTime::now()
    }
  }
  pub fn update(&mut self, emu_duration: EmuDuration, current_time: SteadyTime) {
    let delta_ns = (current_time - self.last_time).num_nanoseconds().unwrap_or(0);
    let clock_edges_per_s =
      emu_duration.as_clock_edges() as f64 / (delta_ns as f64 / 1_000_000_000.0);

    self.make_room_for_new_element();
    self.history.push_front(clock_edges_per_s);

    self.last_time = current_time;
  }
  pub fn get_clock_edges_per_s(&self) -> f64 {
    let sum = self.history.iter().fold(0.0, |acc, &item| acc + item);
    sum / self.history.len() as f64
  }
  fn make_room_for_new_element(&mut self) {
    if self.history.len() >= HISTORY_SIZE {
      let _ = self.history.pop_back();
    }
  }
}
