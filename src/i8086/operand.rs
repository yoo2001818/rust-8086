use std::convert::From;
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
  pub fn get_operand<T>(&self, operand: &Operand) -> T
    where T: OperandValue
  {
    match operand {
      Operand::Register(reg) => T::read_reg(&self.register, reg),
      Operand::SegmentRegister(reg) => T::read_seg(&self.register, reg),
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
  pub fn set_operand<T>(&self, operand: &Operand, value: T) -> ()
    where T: OperandValue
  {
    match operand {
      Operand::Register(reg) => T::write_reg(&mut self.register, reg, value),
      Operand::SegmentRegister(reg) => T::write_seg(&mut self.register, reg, value),
      Operand::Address(addr, offset) => T::write_mem(
        &mut self.memory,
        (self.get_offset(addr, *offset) as usize) +
        self.get_segment_addr(), value),
      Operand::Direct(offset) => T::write_mem(
        &mut self.memory,
        (*offset as usize) +
        self.get_segment_addr(), value),
      _ => (),
    }
  }
}

pub trait OperandValue: MemoryValue + RegisterValue + Add<Output=Self> + Sized {
  fn from_u8(value: u8) -> Self;
  fn from_u16(value: u16) -> Self;
}

impl OperandValue for u8 {
  fn from_u8(value: u8) -> u8 { value }
  fn from_u16(value: u16) -> u8 { value as u8 }
}

impl OperandValue for u16 {
  fn from_u8(value: u8) -> u16 { value as i8 as i16 as u16 }
  fn from_u16(value: u16) -> u16 { value }
}
