use crate::mem;
use super::register;
use super::op;

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
  pub fn get_target_u16(&self, target: &op::OpTarget) -> u16 {
    match target {
      op::OpTarget::Register(reg) => {
        match reg {
          op::OpRegister::Ax => self.register.ax,
        }
      },
      op::OpTarget::SegmentRegister(seg) => {

      },
      op::OpTarget::Address(base, offset) => {

      },
      op::OpTarget::Direct(offset) => {

      },
      op::OpTarget::ImmWord(value) => {

      },
      op::OpTarget::ImmByte(value) => {

      },
    }
  }
}

impl Iterator for CPU {
  type Item = u8;

  fn next(&mut self) -> Option<u8> {
    Some(0)
  }
}
