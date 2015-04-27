#[derive(Clone, Copy)]
pub enum WaveDuty {
  HalfQuarter = 0,
  Quarter = 1,
  Half = 2,
  ThreeQuarters = 3,
}

impl WaveDuty {
  pub fn from_u8(value: u8) -> Option<WaveDuty> {
    use self::WaveDuty::*;
    match value {
      0 => Some(HalfQuarter),
      1 => Some(Quarter),
      2 => Some(Half),
      3 => Some(ThreeQuarters),
      _ => None
    }
  }
}
