#[derive(Debug)]
pub struct Register {
  ax: i16,
  bx: i16,
  cx: i16,
  dx: i16,
  ip: i16,
  sp: i16,
  bp: i16,
  si: i16,
  di: i16,
  cs: i16,
  ss: i16,
  ds: i16,
  es: i16,
  flags: i16,
}

impl Register {
  pub fn new() -> Self {
    Register {
      ax: 0,
      bx: 0,
      cx: 0,
      dx: 0,
      ip: 0,
      sp: 0,
      bp: 0,
      si: 0,
      di: 0,
      cs: 0,
      ss: 0,
      ds: 0,
      es: 0,
      flags: 0,
    }
  }
}
