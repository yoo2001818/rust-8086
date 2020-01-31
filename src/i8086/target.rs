use super::register::OpRegister;
use super::register::OpSegmentRegister;

#[derive(PartialEq, Copy, Clone)]
#[derive(Debug)]
pub enum OpAddressType {
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
pub enum OpTarget {
  Register(OpRegister),
  SegmentRegister(OpSegmentRegister),
  Address(OpAddressType, u16),
  Direct(u16),
  ImmWord(u16),
  ImmByte(u8),
}
