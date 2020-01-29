#[derive(Debug)]
pub struct Register {
  ax: u16,
  bx: u16,
  cx: u16,
  dx: u16,
  ip: u16,
  sp: u16,
  bp: u16,
  si: u16,
  di: u16,
  cs: u16,
  ss: u16,
  ds: u16,
  es: u16,
  flags: u16,
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
