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
use gameboy::{
  WramBank,
  WRAM_BANK_EMPTY
};

pub struct InternalRam {
  wram_bank0: WramBank,
  wram_bank1: WramBank
}

impl InternalRam {
  pub fn new() -> InternalRam {
    InternalRam {
      wram_bank0: WRAM_BANK_EMPTY,
      wram_bank1: WRAM_BANK_EMPTY
    }
  }

  pub fn read_bank0(&self, reladdr: u16) -> u8 {
    self.wram_bank0[reladdr as usize]
  }
  pub fn write_bank0(&mut self, reladdr: u16, value: u8) {
    self.wram_bank0[reladdr as usize] = value;
  }

  pub fn read_bank1(&self, reladdr: u16) -> u8 {
    self.wram_bank1[reladdr as usize]
  }
  pub fn write_bank1(&mut self, reladdr: u16, value: u8) {
    self.wram_bank1[reladdr as usize] = value;
  }
}
