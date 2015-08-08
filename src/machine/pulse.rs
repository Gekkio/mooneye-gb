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
use snooze::Snooze;
use std::sync::mpsc::{Receiver, sync_channel};
use std::thread;
use time::Duration;

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
