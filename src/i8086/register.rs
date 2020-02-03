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
      cs: 0xffff,
      ss: 0,
      ds: 0,
      es: 0,
      flags: 0,
    }
  }
}

#[derive(PartialEq, Copy, Clone)]
#[derive(Debug)]
pub enum RegisterType {
  Ax,
  Cx,
  Dx,
  Bx,
  Sp,
  Bp,
  Si,
  Di,
  Al,
  Cl,
  Dl,
  Bl,
  Ah,
  Ch,
  Dh,
  Bh,
}

#[derive(PartialEq, Copy, Clone)]
#[derive(Debug)]
pub enum SegmentRegisterType {
  Es,
  Cs,
  Ss,
  Ds,
}

pub trait RegisterValue {
  fn read_reg(store: &Register, register: &RegisterType) -> Self;
  fn write_reg(store: &mut Register, register: &RegisterType, value: Self) -> ();
  fn read_seg(store: &Register, register: &SegmentRegisterType) -> Self;
  fn write_seg(store: &mut Register, register: &SegmentRegisterType, value: Self) -> ();
}

impl RegisterValue for u8 {
  fn read_reg(store: &Register, register: &RegisterType) -> u8 {
    match register {
      RegisterType::Al => (store.ax & 0xff) as u8,
      RegisterType::Cl => (store.cx & 0xff) as u8,
      RegisterType::Dl => (store.dx & 0xff) as u8,
      RegisterType::Bl => (store.bx & 0xff) as u8,
      RegisterType::Ah => ((store.ax >> 8) & 0xff) as u8,
      RegisterType::Ch => ((store.cx >> 8) & 0xff) as u8,
      RegisterType::Dh => ((store.dx >> 8) & 0xff) as u8,
      RegisterType::Bh => ((store.bx >> 8) & 0xff) as u8,
      _ => 0,
    }
  }
  fn write_reg(store: &mut Register, register: &RegisterType, value: u8) -> () {
    match register {
      RegisterType::Al => store.ax = (store.ax & !0xff) | value as u16,
      RegisterType::Cl => store.cx = (store.cx & !0xff) | value as u16,
      RegisterType::Dl => store.dx = (store.dx & !0xff) | value as u16,
      RegisterType::Bl => store.bx = (store.bx & !0xff) | value as u16,
      RegisterType::Ah => store.ax = (store.ax & !0xff00) | ((value as u16) << 8),
      RegisterType::Ch => store.cx = (store.cx & !0xff00) | ((value as u16) << 8),
      RegisterType::Dh => store.dx = (store.dx & !0xff00) | ((value as u16) << 8),
      RegisterType::Bh => store.bx = (store.bx & !0xff00) | ((value as u16) << 8),
      _ => (),
    }
  }
  fn read_seg(store: &Register, register: &SegmentRegisterType) -> u8 {
    0
  }
  fn write_seg(store: &mut Register, register: &SegmentRegisterType, value: u8) -> () {
  }
}

impl RegisterValue for u16 {
  fn read_reg(store: &Register, register: &RegisterType) -> u16 {
    match register {
      RegisterType::Ax => store.ax,
      RegisterType::Cx => store.cx,
      RegisterType::Dx => store.dx,
      RegisterType::Bx => store.bx,
      RegisterType::Sp => store.sp,
      RegisterType::Bp => store.bp,
      RegisterType::Si => store.si,
      RegisterType::Di => store.di,
      _ => 0,
    }
  }
  fn write_reg(store: &mut Register, register: &RegisterType, value: u16) -> () {
    match register {
      RegisterType::Ax => store.ax = value,
      RegisterType::Cx => store.cx = value,
      RegisterType::Dx => store.dx = value,
      RegisterType::Bx => store.bx = value,
      RegisterType::Sp => store.sp = value,
      RegisterType::Bp => store.bp = value,
      RegisterType::Si => store.si = value,
      RegisterType::Di => store.di = value,
      _ => (),
    }
  }
  fn read_seg(store: &Register, register: &SegmentRegisterType) -> u16 {
    match register {
      SegmentRegisterType::Es => store.es,
      SegmentRegisterType::Cs => store.cs,
      SegmentRegisterType::Ss => store.ss,
      SegmentRegisterType::Ds => store.ds,
    }
  }
  fn write_seg(store: &mut Register, register: &SegmentRegisterType, value: u16) -> () {
    match register {
      SegmentRegisterType::Es => store.es = value,
      SegmentRegisterType::Cs => store.cs = value,
      SegmentRegisterType::Ss => store.ss = value,
      SegmentRegisterType::Ds => store.ds = value,
    }
  }
}
