#[derive(Debug)]
pub struct Register {
  pub ax: u16,
  pub bx: u16,
  pub cx: u16,
  pub dx: u16,
  pub ip: u16,
  pub sp: u16,
  pub bp: u16,
  pub si: u16,
  pub di: u16,
  pub cs: u16,
  pub ss: u16,
  pub ds: u16,
  pub es: u16,
  pub flags: u16,
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
