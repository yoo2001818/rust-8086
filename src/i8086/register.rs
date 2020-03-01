use std::fmt;

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

impl fmt::Debug for Register {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "ax: {:04X} bx: {:04X} cx: {:04X} dx: {:04X}\n\
      sp: {:04X} bp: {:04X} si: {:04X} di: {:04X}\n\
      cs: {:04X} ss: {:04X} ds: {:04X} es: {:04X}\n\
      fl: {:04X} ip: {:04X}",
      self.ax, self.bx, self.cx, self.dx,
      self.sp, self.bp, self.si, self.di,
      self.cs, self.ss, self.ds, self.es,
      self.flags, self.ip,
    )
  }
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
pub enum RegisterWordType {
  Ax,
  Cx,
  Dx,
  Bx,
  Sp,
  Bp,
  Si,
  Di,
  Es,
  Cs,
  Ss,
  Ds,
}

#[derive(PartialEq, Copy, Clone)]
#[derive(Debug)]
pub enum RegisterByteType {
  Al,
  Cl,
  Dl,
  Bl,
  Ah,
  Ch,
  Dh,
  Bh,
}

pub trait RegisterType {
  type Value;
  fn read_reg(store: &Register, register: &Self) -> Self::Value;
  fn write_reg(store: &mut Register, register: &Self, value: Self::Value) -> ();
}

impl RegisterType for RegisterWordType {
  type Value = u16;
  fn read_reg(store: &Register, register: &RegisterWordType) -> u16 {
    match register {
      RegisterWordType::Ax => store.ax,
      RegisterWordType::Cx => store.cx,
      RegisterWordType::Dx => store.dx,
      RegisterWordType::Bx => store.bx,
      RegisterWordType::Sp => store.sp,
      RegisterWordType::Bp => store.bp,
      RegisterWordType::Si => store.si,
      RegisterWordType::Di => store.di,
      RegisterWordType::Es => store.es,
      RegisterWordType::Cs => store.cs,
      RegisterWordType::Ss => store.ss,
      RegisterWordType::Ds => store.ds,
    }
  }
  fn write_reg(store: &mut Register, register: &RegisterWordType, value: u16) -> () {
    match register {
      RegisterWordType::Ax => store.ax = value,
      RegisterWordType::Cx => store.cx = value,
      RegisterWordType::Dx => store.dx = value,
      RegisterWordType::Bx => store.bx = value,
      RegisterWordType::Sp => store.sp = value,
      RegisterWordType::Bp => store.bp = value,
      RegisterWordType::Si => store.si = value,
      RegisterWordType::Di => store.di = value,
      RegisterWordType::Es => store.es = value,
      RegisterWordType::Cs => store.cs = value,
      RegisterWordType::Ss => store.ss = value,
      RegisterWordType::Ds => store.ds = value,
    }
  }
}

impl RegisterType for RegisterByteType {
  type Value = u8;
  fn read_reg(store: &Register, register: &RegisterByteType) -> u8 {
    match register {
      RegisterByteType::Al => (store.ax & 0xff) as u8,
      RegisterByteType::Cl => (store.cx & 0xff) as u8,
      RegisterByteType::Dl => (store.dx & 0xff) as u8,
      RegisterByteType::Bl => (store.bx & 0xff) as u8,
      RegisterByteType::Ah => ((store.ax >> 8) & 0xff) as u8,
      RegisterByteType::Ch => ((store.cx >> 8) & 0xff) as u8,
      RegisterByteType::Dh => ((store.dx >> 8) & 0xff) as u8,
      RegisterByteType::Bh => ((store.bx >> 8) & 0xff) as u8,
      _ => 0,
    }
  }
  fn write_reg(store: &mut Register, register: &RegisterByteType, value: u8) -> () {
    match register {
      RegisterByteType::Al => store.ax = (store.ax & !0xff) | value as u16,
      RegisterByteType::Cl => store.cx = (store.cx & !0xff) | value as u16,
      RegisterByteType::Dl => store.dx = (store.dx & !0xff) | value as u16,
      RegisterByteType::Bl => store.bx = (store.bx & !0xff) | value as u16,
      RegisterByteType::Ah => store.ax = (store.ax & !0xff00) | ((value as u16) << 8),
      RegisterByteType::Ch => store.cx = (store.cx & !0xff00) | ((value as u16) << 8),
      RegisterByteType::Dh => store.dx = (store.dx & !0xff00) | ((value as u16) << 8),
      RegisterByteType::Bh => store.bx = (store.bx & !0xff00) | ((value as u16) << 8),
      _ => (),
    }
  }
}

pub trait RegisterValue<T> {
  fn read_reg(store: &Register, register: &T) -> Self;
  fn write_reg(store: &mut Register, register: &T, value: Self) -> ();
}

impl<T, V> RegisterValue<T> for V where T: RegisterType<Value = V> {
  fn read_reg(store: &Register, register: &T) -> V {
    T::read_reg(store, register)
  }
  fn write_reg(store: &mut Register, register: &T, value: V) -> () {
    T::write_reg(store, register, value)
  }
}
