use crate::mem::Memory;
use crate::mem::MemoryValue;
use super::register::Register;
use super::op::Op;
use super::op::parse_op;

pub struct CPU {
  pub memory: Box<dyn Memory>,
  pub io_ports: Box<dyn Memory>,
  pub register: Register,
  pub running: bool,
}

impl CPU {
  pub fn new(memory: Box<dyn Memory>, io_ports: Box<dyn Memory>) -> Self {
    CPU { memory, io_ports, register: Register::new(), running: true }
  }

  pub fn iter(&mut self) -> CPUIterator {
    CPUIterator::new(self)
  }

  pub fn next_op(&mut self) -> Option<Op> {
    parse_op(&mut self.iter())
  }

  pub fn step(&mut self) -> Option<()> {
    if !self.running {
      return None;
    }
    let op = self.next_op()?;
    self.exec_op(&op);
    Some(())
  }

  pub fn jmp(&mut self, seg: u16, addr: u16) -> () {
    self.register.cs = seg;
    self.register.ip = addr;
  }

  pub fn hlt(&mut self) -> () {
    self.running = false;
  }

  pub fn unhlt(&mut self) -> () {
    self.running = true;
  }

  pub fn run(&mut self) -> () {
    while self.running {
      self.step();
    }
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
    let value = u8::read_mem(&*self.cpu.memory, addr);
    self.cpu.register.ip += 1;
    Some(value)
  }
}

#[cfg(test)]
mod tests {
  use crate::mem::*;
  use crate::mem::linear::LinearMemory;
  use super::CPU;
  use super::super::op::*;
  #[test]
  fn cpu_init() {
    let mut mem = LinearMemory::new(0xFFFFF);
    u8::write_mem(&mut mem, 0xFFFF0, 0b11101010);
    u16::write_mem(&mut mem, 0xFFFF1, 0x0000);
    u16::write_mem(&mut mem, 0xFFFF3, 0xf000);
    let io_ports = LinearMemory::new(0);
    let mut cpu = CPU::new(Box::new(mem), Box::new(io_ports));
    assert_eq!(
      cpu.next_op(),
      Some(Op::Jmp(OpCallType::InterDirect(0x0000, 0xf000))),
    );
  }
}
