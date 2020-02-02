use crate::mem::LinearMemory;
use crate::mem::Memory;
use super::register::Register;
use super::op::Op;
use super::op::parse_op;

pub struct CPU {
  pub memory: LinearMemory,
  pub register: Register,
}

impl CPU {
  pub fn new(memory: LinearMemory) -> Self {
    CPU { memory, register: Register::new() }
  }

  pub fn iter(&mut self) -> CPUIterator {
    CPUIterator::new(self)
  }

  pub fn next_op(&mut self) -> Option<Op> {
    parse_op(&mut self.iter())
  }
}

pub struct CPUIterator<'cpu> {
  cpu: &'cpu mut CPU,
}

impl<'cpu> CPUIterator<'cpu> {
  pub fn new(cpu: &'cpu mut CPU) -> Self {
    CPUIterator { cpu: cpu }
  }
}

impl<'cpu> Iterator for CPUIterator<'cpu> {
  type Item = u8;

  fn next(&mut self) -> Option<u8> {
    let addr = self.cpu.register.ip as usize +
      ((self.cpu.register.cs as usize) << 4);
    let value = self.cpu.memory.read_u8(addr);
    self.cpu.register.ip += 1;
    Some(value)
  }
}

#[cfg(test)]
mod tests {
  use crate::mem::*;
  use super::CPU;
  use super::super::op::*;
  #[test]
  fn cpu_init() {
    let mut mem = LinearMemory::new(0xFFFFF);
    mem.write_u8(0xFFFF0, 0b11101010);
    mem.write_u16(0xFFFF1, 0x0000);
    mem.write_u16(0xFFFF3, 0xf000);
    let mut cpu = CPU::new(mem);
    assert_eq!(
      cpu.next_op(),
      Some(Op::Jmp(OpCallType::InterDirect(0x0000, 0xf000))),
    );
  }
}
