use super::Memory;

pub struct LinearMemory {
  words: Vec<u32>,
}

impl LinearMemory {
  pub fn new(size: usize) -> LinearMemory {
    let words = vec![0; size];
    LinearMemory { words }
  }
}

impl Memory for LinearMemory {
  fn read(&self, address: usize) -> u32 {
    self.words[address]
  }
  fn write(&mut self, address: usize, value: u32) -> () {
    self.words[address] = value
  }
}

#[cfg(test)]
mod tests {
  use crate::mem::*;
  use super::*;

  #[test]
  fn test() {
    let mut mem = LinearMemory::new(1024);
    mem.write(1000, 0x7F7F7F7F);
    assert_eq!(mem.read(1000), 0x7F7F7F7F);
  }

  #[test]
  fn test_u8() {
    let mut mem = LinearMemory::new(1024);
    mem.write(0, 0x12345678);
    mem.write(1, 0xabcdef00);
    assert_eq!(u8::read_mem(&mem, 0), 0x78);
    assert_eq!(u8::read_mem(&mem, 1), 0x56);
    assert_eq!(u8::read_mem(&mem, 2), 0x34);
    assert_eq!(u8::read_mem(&mem, 3), 0x12);
    assert_eq!(u8::read_mem(&mem, 4), 0x00);
    assert_eq!(u8::read_mem(&mem, 5), 0xef);
    assert_eq!(u8::read_mem(&mem, 6), 0xcd);
    assert_eq!(u8::read_mem(&mem, 7), 0xab);
  }

  #[test]
  fn test_u16() {
    let mut mem = LinearMemory::new(1024);
    mem.write(0, 0x12345678);
    mem.write(1, 0xabcdef00);
    assert_eq!(u16::read_mem(&mem, 0), 0x5678);
    assert_eq!(u16::read_mem(&mem, 1), 0x3456);
    assert_eq!(u16::read_mem(&mem, 2), 0x1234);
    assert_eq!(u16::read_mem(&mem, 3), 0x0012);
    assert_eq!(u16::read_mem(&mem, 4), 0xef00);
    assert_eq!(u16::read_mem(&mem, 5), 0xcdef);
    assert_eq!(u16::read_mem(&mem, 6), 0xabcd);
    assert_eq!(u16::read_mem(&mem, 7), 0x00ab);
  }
}
