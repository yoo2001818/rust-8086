use std::ops::*;
use super::cpu::CPU;
use super::register::*;
use crate::mem::*;

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
pub enum Operand<T: RegisterType> {
  Register(T),
  Address(AddressType, u16),
  Direct(u16),
  ImmWord(u16),
  ImmByte(u8),
}

pub type OperandWord = Operand<RegisterWordType>;
pub type OperandByte = Operand<RegisterByteType>;

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
  pub fn get_operand<T, R>(&self, operand: &Operand<R>) -> T
    where T: OperandValue<R>, R: RegisterType
  {
    match operand {
      Operand::Register(reg) => T::read_reg(&self.register, reg),
      Operand::Address(addr, offset) => T::read_mem(
        &self.memory,
        (self.get_offset(addr, *offset) as usize) +
        self.get_segment_addr()),
      Operand::Direct(offset) => T::read_mem(
        &self.memory,
        (*offset as usize) +
        self.get_segment_addr()),
      Operand::ImmWord(value) => T::from_u16(*value),
      Operand::ImmByte(value) => T::from_u8(*value),
    }
  }
  pub fn set_operand<T, R>(&mut self, operand: &Operand<R>, value: T) -> ()
    where T: OperandValue<R>, R: RegisterType
  {
    match operand {
      Operand::Register(reg) => T::write_reg(&mut self.register, reg, value),
      Operand::Address(addr, offset) => {
        let address = (self.get_offset(addr, *offset) as usize) +
          self.get_segment_addr();
        T::write_mem(&mut self.memory, address, value);
      }
      Operand::Direct(offset) => {
        let address = (*offset as usize) + self.get_segment_addr();
        T::write_mem(&mut self.memory, address, value);
      }
      _ => (),
    }
  }
}

pub trait OperandValue<R>: MemoryValue + RegisterValue<R> + Add<Output=Self> + Sized {
  fn from_u8(value: u8) -> Self;
  fn from_u16(value: u16) -> Self;
}

impl OperandValue<RegisterByteType> for u8 {
  fn from_u8(value: u8) -> u8 { value }
  fn from_u16(value: u16) -> u8 { value as u8 }
}

impl OperandValue<RegisterWordType> for u16 {
  fn from_u8(value: u8) -> u16 { value as i8 as i16 as u16 }
  fn from_u16(value: u16) -> u16 { value }
}
