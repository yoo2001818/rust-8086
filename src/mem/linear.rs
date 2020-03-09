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
    println!("Read {:08X} {:04X}", address, self.words[address]);
    self.words[address]
  }
  fn write(&mut self, address: usize, value: u32) -> () {
    println!("Write {:08X} {:04X}", address, value);
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

  #[test]
  fn test_u16_write() {
    let mut mem = LinearMemory::new(1024);
    mem.write(1, 0xFFFFFFFF);
    mem.write(2, 0xFFFFFFFF);
    u16::write_mem(&mut mem, 0, 0xabcd);
    u16::write_mem(&mut mem, 2, 0xbeef);
    u16::write_mem(&mut mem, 7, 0xabcd);
    assert_eq!(mem.read(0), 0xBEEFABCD);
    assert_eq!(mem.read(1), 0xCDFFFFFF);
    assert_eq!(mem.read(2), 0xFFFFFFAB);
  }
}
