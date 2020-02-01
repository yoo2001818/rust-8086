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
pub enum AddressOperand {
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
  pub fn get_operand_u16(&self, operand: &AddressOperand) -> u16 {
    match operand {
      AddressOperand::Register(reg) => self.register.get_u16(reg),
      AddressOperand::SegmentRegister(reg) => self.register.get_seg(reg),
      AddressOperand::Address(addr, offset) =>
        self.memory.read_u16(self.get_offset(addr, *offset)),
      AddressOperand::Direct(offset) =>
        self.memory.read_u16(offset),
      AddressOperand::ImmWord(value) => *value,
      AddressOperand::ImmByte(value) => *value as i16 as u16,
    }
  }
  pub fn set_operand_u16(&mut self, operand: &AddressOperand, value: u16) -> () {

  }
  pub fn get_operand_u8(&self, operand: &AddressOperand) -> u8 {

  }
  pub fn set_operand_u8(&mut self, operand: &AddressOperand, value: u8) -> () {

  }
}
