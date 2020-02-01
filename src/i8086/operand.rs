use super::register::RegisterType;
use super::register::SegmentRegisterType;
use super::cpu::CPU;
use crate::mem::Memory;

#[derive(PartialEq, Copy, Clone)]
#[derive(Debug)]
pub enum AddressType {
  BxSi,
  BxDi,
  BpSi,
  BpDi,
  Si,
  Di,
  Bp,
  Bx,
}

#[derive(PartialEq)]
#[derive(Debug)]
pub enum Operand {
  Register(RegisterType),
  SegmentRegister(SegmentRegisterType),
  Address(AddressType, u16),
  Direct(u16),
  ImmWord(u16),
  ImmByte(u8),
}

impl CPU {
  pub fn get_offset(&self, addr_type: &AddressType, offset: u16) -> u16 {
    let base_offset = match addr_type {
      AddressType::BxSi => self.register.bx + self.register.si,
      AddressType::BxDi => self.register.bx + self.register.di,
      AddressType::BpSi => self.register.bp + self.register.si,
      AddressType::BpDi => self.register.bp + self.register.di,
      AddressType::Si => self.register.si,
      AddressType::Di => self.register.di,
      AddressType::Bp => self.register.bp,
      AddressType::Bx => self.register.bx,
    };
    base_offset + offset
  }
  pub fn get_segment_addr(&self) -> usize {
    (self.register.ds as usize) << 4
  }
  pub fn get_operand_u16(&self, operand: &Operand) -> u16 {
    match operand {
      Operand::Register(reg) => self.register.get_u16(reg),
      Operand::SegmentRegister(reg) => self.register.get_seg(reg),
      Operand::Address(addr, offset) => self.memory.read_u16(
        (self.get_offset(addr, *offset) as usize) +
        self.get_segment_addr()),
      Operand::Direct(offset) => self.memory.read_u16(
        (*offset as usize) +
        self.get_segment_addr()),
      Operand::ImmWord(value) => *value,
      Operand::ImmByte(value) => *value as i16 as u16,
    }
  }
  pub fn set_operand_u16(&mut self, operand: &Operand, value: u16) -> () {
    match operand {
      Operand::Register(reg) => self.register.set_u16(reg, value),
      Operand::SegmentRegister(reg) => self.register.set_seg(reg, value),
      Operand::Address(addr, offset) => self.memory.write_u16(
        (self.get_offset(addr, *offset) as usize) +
        self.get_segment_addr(), value),
      Operand::Direct(offset) => self.memory.write_u16(
        (*offset as usize) +
        self.get_segment_addr(), value),
      _ => (),
    }
  }
  pub fn get_operand_u8(&self, operand: &Operand) -> u8 {
    match operand {
      Operand::Register(reg) => self.register.get_u8(reg),
      Operand::Address(addr, offset) => self.memory.read_u8(
        (self.get_offset(addr, *offset) as usize) +
        self.get_segment_addr()),
      Operand::Direct(offset) => self.memory.read_u8(
        (*offset as usize) +
        self.get_segment_addr()),
      Operand::ImmByte(value) => *value,
      _ => 0,
    }
  }
  pub fn set_operand_u8(&mut self, operand: &Operand, value: u8) -> () {
    match operand {
      Operand::Register(reg) => self.register.set_u8(reg, value),
      Operand::Address(addr, offset) => self.memory.write_u8(
        (self.get_offset(addr, *offset) as usize) +
        self.get_segment_addr(), value),
      Operand::Direct(offset) => self.memory.write_u8(
        (*offset as usize) +
        self.get_segment_addr(), value),
      _ => (),
    }
  }
}
