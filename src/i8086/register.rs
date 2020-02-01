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
  pub fn get_u16(&self, reg: &RegisterType) -> u16 {
    match reg {
      RegisterType::Ax => self.ax,
      RegisterType::Cx => self.cx,
      RegisterType::Dx => self.dx,
      RegisterType::Bx => self.bx,
      RegisterType::Sp => self.sp,
      RegisterType::Bp => self.bp,
      RegisterType::Si => self.si,
      RegisterType::Di => self.di,
      _ => 0,
    }
  }
  pub fn set_u16(&mut self, reg: &RegisterType, value: u16) -> () {
    match reg {
      RegisterType::Ax => self.ax = value,
      RegisterType::Cx => self.cx = value,
      RegisterType::Dx => self.dx = value,
      RegisterType::Bx => self.bx = value,
      RegisterType::Sp => self.sp = value,
      RegisterType::Bp => self.bp = value,
      RegisterType::Si => self.si = value,
      RegisterType::Di => self.di = value,
      _ => (),
    }
  }
  pub fn get_u8(&self, reg: &RegisterType) -> u8 {
    match reg {
      RegisterType::Al => (self.ax & 0xff) as u8,
      RegisterType::Cl => (self.cx & 0xff) as u8,
      RegisterType::Dl => (self.dx & 0xff) as u8,
      RegisterType::Bl => (self.bx & 0xff) as u8,
      RegisterType::Ah => ((self.ax >> 8) & 0xff) as u8,
      RegisterType::Ch => ((self.cx >> 8) & 0xff) as u8,
      RegisterType::Dh => ((self.dx >> 8) & 0xff) as u8,
      RegisterType::Bh => ((self.bx >> 8) & 0xff) as u8,
      _ => 0,
    }
  }
  pub fn set_u8(&mut self, reg: &RegisterType, value: u8) -> () {
    match reg {
      RegisterType::Al => self.ax = (self.ax & !0xff) | value as u16,
      RegisterType::Cl => self.cx = (self.cx & !0xff) | value as u16,
      RegisterType::Dl => self.dx = (self.dx & !0xff) | value as u16,
      RegisterType::Bl => self.bx = (self.bx & !0xff) | value as u16,
      RegisterType::Ah => self.ax = (self.ax & !0xff00) | ((value as u16) << 8),
      RegisterType::Ch => self.cx = (self.cx & !0xff00) | ((value as u16) << 8),
      RegisterType::Dh => self.dx = (self.dx & !0xff00) | ((value as u16) << 8),
      RegisterType::Bh => self.bx = (self.bx & !0xff00) | ((value as u16) << 8),
      _ => (),
    }
  }
  pub fn get_seg(&self, reg: &SegmentRegisterType) -> u16 {
    match reg {
      SegmentRegisterType::Es => self.es,
      SegmentRegisterType::Cs => self.cs,
      SegmentRegisterType::Ss => self.ss,
      SegmentRegisterType::Ds => self.ds,
    }
  }
  pub fn set_seg(&mut self, reg: &SegmentRegisterType, value: u16) -> () {
    match reg {
      SegmentRegisterType::Es => self.es = value,
      SegmentRegisterType::Cs => self.cs = value,
      SegmentRegisterType::Ss => self.ss = value,
      SegmentRegisterType::Ds => self.ds = value,
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
