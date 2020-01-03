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
