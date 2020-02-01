use crate::mem;
use super::register;
use super::op;
use crate::mem::Memory;

pub struct CPU {
  pub memory: mem::LinearMemory,
  pub register: register::Register,
}

impl CPU {
  pub fn new(memory: mem::LinearMemory) -> Self {
    CPU { memory, register: register::Register::new() }
  }
  pub fn next() -> () {

  }
  pub fn get_effective_addr(&self, target: &op::AddressOperand) -> CPUEffectiveTarget {
    match target {
      op::AddressOperand::Register(reg) => CPUEffectiveTarget::Register(*reg),
      op::AddressOperand::SegmentRegister(seg) => CPUEffectiveTarget::SegmentRegister(*seg),
      op::AddressOperand::Address(base, offset) => {
        let base_offset = match base {
          op::AddressType::BxSi => self.register.bx + self.register.si,
          op::AddressType::BxDi => self.register.bx + self.register.di,
          op::AddressType::BpSi => self.register.bp + self.register.si,
          op::AddressType::BpDi => self.register.bp + self.register.di,
          op::AddressType::Si => self.register.si,
          op::AddressType::Di => self.register.di,
          op::AddressType::Bp => self.register.bp,
          op::AddressType::Bx => self.register.bx,
        };
        let total_offset = base_offset + offset;
        let segment = (self.register.ds as usize) << 4;
        CPUEffectiveTarget::Address(segment + (total_offset as usize))
      },
      op::AddressOperand::Direct(offset) => {
        let segment = (self.register.ds as usize) << 4;
        CPUEffectiveTarget::Address(segment + (*offset as usize))
      },
      op::AddressOperand::ImmWord(value) => CPUEffectiveTarget::ImmWord(*value),
      op::AddressOperand::ImmByte(value) => CPUEffectiveTarget::ImmByte(*value),
    }
  }
  pub fn get_target_u16(&self, target: &CPUEffectiveTarget) -> u16 {
    match target {
      CPUEffectiveTarget::Register(reg) => {
        match reg {
          op::RegisterType::Ax => self.register.ax,
          op::RegisterType::Cx => self.register.cx,
          op::RegisterType::Dx => self.register.dx,
          op::RegisterType::Bx => self.register.bx,
          op::RegisterType::Sp => self.register.sp,
          op::RegisterType::Bp => self.register.bp,
          op::RegisterType::Si => self.register.si,
          op::RegisterType::Di => self.register.di,
          _ => 0,
        }
      },
      CPUEffectiveTarget::SegmentRegister(seg) => {
        match seg {
          op::SegmentRegisterType::Es => self.register.es,
          op::SegmentRegisterType::Cs => self.register.cs,
          op::SegmentRegisterType::Ss => self.register.ss,
          op::SegmentRegisterType::Ds => self.register.ds,
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
          op::RegisterType::Ax => self.register.ax = value,
          op::RegisterType::Cx => self.register.cx = value,
          op::RegisterType::Dx => self.register.dx = value,
          op::RegisterType::Bx => self.register.bx = value,
          op::RegisterType::Sp => self.register.sp = value,
          op::RegisterType::Bp => self.register.bp = value,
          op::RegisterType::Si => self.register.si = value,
          op::RegisterType::Di => self.register.di = value,
          _ => (),
        }
      },
      CPUEffectiveTarget::SegmentRegister(seg) => {
        match seg {
          op::SegmentRegisterType::Es => self.register.es = value,
          op::SegmentRegisterType::Cs => self.register.cs = value,
          op::SegmentRegisterType::Ss => self.register.ss = value,
          op::SegmentRegisterType::Ds => self.register.ds = value,
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
          op::RegisterType::Al => (self.register.ax & 0xff) as u8,
          op::RegisterType::Cl => (self.register.cx & 0xff) as u8,
          op::RegisterType::Dl => (self.register.dx & 0xff) as u8,
          op::RegisterType::Bl => (self.register.bx & 0xff) as u8,
          op::RegisterType::Ah => ((self.register.ax >> 8) & 0xff) as u8,
          op::RegisterType::Ch => ((self.register.cx >> 8) & 0xff) as u8,
          op::RegisterType::Dh => ((self.register.dx >> 8) & 0xff) as u8,
          op::RegisterType::Bh => ((self.register.bx >> 8) & 0xff) as u8,
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
          op::RegisterType::Al => self.register.ax = (self.register.ax & !0xff) | value as u16,
          op::RegisterType::Cl => self.register.cx = (self.register.cx & !0xff) | value as u16,
          op::RegisterType::Dl => self.register.dx = (self.register.dx & !0xff) | value as u16,
          op::RegisterType::Bl => self.register.bx = (self.register.bx & !0xff) | value as u16,
          op::RegisterType::Ah => self.register.ax = (self.register.ax & !0xff00) | ((value as u16) << 8),
          op::RegisterType::Ch => self.register.cx = (self.register.cx & !0xff00) | ((value as u16) << 8),
          op::RegisterType::Dh => self.register.dx = (self.register.dx & !0xff00) | ((value as u16) << 8),
          op::RegisterType::Bh => self.register.bx = (self.register.bx & !0xff00) | ((value as u16) << 8),
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
