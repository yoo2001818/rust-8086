use crate::mem;
use super::register;
use super::op;
use crate::mem::Memory;

pub enum CPUEffectiveTarget {
  Register(op::OpRegister),
  SegmentRegister(op::OpSegmentRegister),
  Address(usize),
  ImmWord(u16),
  ImmByte(u8),
}

pub struct CPU {
  memory: mem::LinearMemory,
  register: register::Register,
}

impl CPU {
  pub fn new(memory: mem::LinearMemory) -> Self {
    CPU { memory, register: register::Register::new() }
  }
  pub fn next() -> () {

  }
  pub fn get_effective_addr(&self, target: &op::OpTarget) -> CPUEffectiveTarget {
    match target {
      op::OpTarget::Register(reg) => CPUEffectiveTarget::Register(*reg),
      op::OpTarget::SegmentRegister(seg) => CPUEffectiveTarget::SegmentRegister(*seg),
      op::OpTarget::Address(base, offset) => {
        let base_offset = match base {
          op::OpAddressType::BxSi => self.register.bx + self.register.si,
          op::OpAddressType::BxDi => self.register.bx + self.register.di,
          op::OpAddressType::BpSi => self.register.bp + self.register.si,
          op::OpAddressType::BpDi => self.register.bp + self.register.di,
          op::OpAddressType::Si => self.register.si,
          op::OpAddressType::Di => self.register.di,
          op::OpAddressType::Bp => self.register.bp,
          op::OpAddressType::Bx => self.register.bx,
        };
        let total_offset = base_offset + offset;
        let segment = (self.register.ds as usize) << 4;
        CPUEffectiveTarget::Address(segment + (total_offset as usize))
      },
      op::OpTarget::Direct(offset) => {
        let segment = (self.register.ds as usize) << 4;
        CPUEffectiveTarget::Address(segment + (*offset as usize))
      },
      op::OpTarget::ImmWord(value) => CPUEffectiveTarget::ImmWord(*value),
      op::OpTarget::ImmByte(value) => CPUEffectiveTarget::ImmByte(*value),
    }
  }
  pub fn get_target_u16(&self, target: &CPUEffectiveTarget) -> u16 {
    match target {
      CPUEffectiveTarget::Register(reg) => {
        match reg {
          op::OpRegister::Ax => self.register.ax,
          op::OpRegister::Cx => self.register.cx,
          op::OpRegister::Dx => self.register.dx,
          op::OpRegister::Bx => self.register.bx,
          op::OpRegister::Sp => self.register.sp,
          op::OpRegister::Bp => self.register.bp,
          op::OpRegister::Si => self.register.si,
          op::OpRegister::Di => self.register.di,
          _ => 0,
        }
      },
      CPUEffectiveTarget::SegmentRegister(seg) => {
        match seg {
          op::OpSegmentRegister::Es => self.register.es,
          op::OpSegmentRegister::Cs => self.register.cs,
          op::OpSegmentRegister::Ss => self.register.ss,
          op::OpSegmentRegister::Ds => self.register.ds,
        }
      },
      CPUEffectiveTarget::Address(addr) => self.memory.read_u16(*addr),
      CPUEffectiveTarget::ImmWord(value) => *value,
      CPUEffectiveTarget::ImmByte(value) => *value as i16 as u16,
    }
  }
  pub fn set_target_u16(&mut self, target: &CPUEffectiveTarget, value: u16) -> () {
    match target {
      CPUEffectiveTarget::Register(reg) => {
        match reg {
          op::OpRegister::Ax => self.register.ax = value,
          op::OpRegister::Cx => self.register.cx = value,
          op::OpRegister::Dx => self.register.dx = value,
          op::OpRegister::Bx => self.register.bx = value,
          op::OpRegister::Sp => self.register.sp = value,
          op::OpRegister::Bp => self.register.bp = value,
          op::OpRegister::Si => self.register.si = value,
          op::OpRegister::Di => self.register.di = value,
          _ => (),
        }
      },
      CPUEffectiveTarget::SegmentRegister(seg) => {
        match seg {
          op::OpSegmentRegister::Es => self.register.es = value,
          op::OpSegmentRegister::Cs => self.register.cs = value,
          op::OpSegmentRegister::Ss => self.register.ss = value,
          op::OpSegmentRegister::Ds => self.register.ds = value,
        }
      },
      CPUEffectiveTarget::Address(addr) => self.memory.write_u16(*addr, value),
      _ => (),
    }
  }
  pub fn get_target_u8(&self, target: &CPUEffectiveTarget) -> u8 {
    match target {
      CPUEffectiveTarget::Register(reg) => {
        match reg {
          op::OpRegister::Al => (self.register.ax & 0xff) as u8,
          op::OpRegister::Cl => (self.register.cx & 0xff) as u8,
          op::OpRegister::Dl => (self.register.dx & 0xff) as u8,
          op::OpRegister::Bl => (self.register.bx & 0xff) as u8,
          op::OpRegister::Ah => ((self.register.ax >> 8) & 0xff) as u8,
          op::OpRegister::Ch => ((self.register.cx >> 8) & 0xff) as u8,
          op::OpRegister::Dh => ((self.register.dx >> 8) & 0xff) as u8,
          op::OpRegister::Bh => ((self.register.bx >> 8) & 0xff) as u8,
          _ => 0,
        }
      },
      CPUEffectiveTarget::Address(addr) => self.memory.read_u8(*addr),
      CPUEffectiveTarget::ImmByte(value) => *value as u8,
      _ => 0,
    }
  }
  pub fn set_target_u8(&mut self, target: &CPUEffectiveTarget, value: u8) -> () {
    match target {
      CPUEffectiveTarget::Register(reg) => {
        match reg {
          op::OpRegister::Al => self.register.ax = (self.register.ax & !0xff) | value as u16,
          op::OpRegister::Cl => self.register.cx = (self.register.cx & !0xff) | value as u16,
          op::OpRegister::Dl => self.register.dx = (self.register.dx & !0xff) | value as u16,
          op::OpRegister::Bl => self.register.bx = (self.register.bx & !0xff) | value as u16,
          op::OpRegister::Ah => self.register.ax = (self.register.ax & !0xff00) | ((value as u16) << 8),
          op::OpRegister::Ch => self.register.cx = (self.register.cx & !0xff00) | ((value as u16) << 8),
          op::OpRegister::Dh => self.register.dx = (self.register.dx & !0xff00) | ((value as u16) << 8),
          op::OpRegister::Bh => self.register.bx = (self.register.bx & !0xff00) | ((value as u16) << 8),
          _ => (),
        }
      },
      CPUEffectiveTarget::Address(addr) => self.memory.write_u8(*addr, value),
      _ => (),
    }
  }
}

impl Iterator for CPU {
  type Item = u8;

  fn next(&mut self) -> Option<u8> {
    Some(0)
  }
}
