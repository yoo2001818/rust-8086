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
}

impl Iterator for CPU {
  type Item = u8;

  fn next(&mut self) -> Option<u8> {
    Some(0)
  }
}
