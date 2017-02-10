// This file is part of Mooneye GB.
// Copyright (C) 2014-2017 Joonas Javanainen <joonas.javanainen@gmail.com>
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
use std::ops::Index;

pub struct Bootrom {
  data: Box<[u8]>,
  active: bool
}

impl Bootrom {
  pub fn new(config: Option<Vec<u8>>) -> Bootrom {
    let (installed, data) = match config {
      Some(config_data) => (true, config_data),
      None => (false, vec![])
    };

    Bootrom {
      data: data.into_boxed_slice(),
      active: installed
    }
  }

  pub fn is_active(&self) -> bool { self.active }
  pub fn deactivate(&mut self) { self.active = false; }
}

impl Index<u16> for Bootrom {
  type Output = u8;
  fn index(&self, index: u16) -> &u8 {
    &self.data[index as usize]
  }
}
