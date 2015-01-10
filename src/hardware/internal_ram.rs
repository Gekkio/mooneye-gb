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
