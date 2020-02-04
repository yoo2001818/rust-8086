pub trait Memory {
  fn read(&self, word_addr: usize) -> u32;
  fn write(&mut self, word_addr: usize, value: u32) -> ();
}

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

pub trait MemoryValue {
  fn read_mem(memory: &dyn Memory, byte_addr: usize) -> Self;
  fn write_mem(memory: &mut dyn Memory, byte_addr: usize, value: Self) -> ();
}

impl MemoryValue for u8 {
  fn read_mem(memory: &dyn Memory, byte_addr: usize) -> u8 {
    let word_addr = byte_addr >> 2;
    // x86 uses little endian; 12 34 56 78 would be saved as 78 56 34 12.
    // This means that the step has to be inverted.
    let step = byte_addr & 3;
    let word = memory.read(word_addr);
    (match step {
      0 => word & 0xff,
      1 => (word & 0xff00) >> 8,
      2 => (word & 0xff0000) >> 16,
      3 => (word & 0xff000000) >> 24,
      _ => 0,
    }) as u8
  }
  fn write_mem(memory: &mut dyn Memory, byte_addr: usize, value: u8) -> () {
    let word_addr = byte_addr >> 2;
    // x86 uses little endian; 12 34 56 78 would be saved as 78 56 34 12.
    // This means that the step has to be inverted.
    let step = byte_addr & 3;
    let mut word = memory.read(word_addr);
    match step {
      0 => {
        word &= 0xffffff00;
        word |= value as u32;
      },
      1 => {
        word &= 0xffff00ff;
        word |= (value as u32) << 8;
      },
      2 => {
        word &= 0xff00ffff;
        word |= (value as u32) << 16;
      },
      3 => {
        word &= 0x00ffffff;
        word |= (value as u32) << 24;
      },
      _ => {},
    }
    memory.write(word_addr, word); 
  }
}

impl MemoryValue for u16 {
  fn read_mem(memory: &dyn Memory, byte_addr: usize) -> u16 {
    let word_addr = byte_addr >> 2;
    let step = byte_addr & 3;
    let word = memory.read(word_addr);
    (match step {
      0 => word & 0x0000ffff,
      1 => (word & 0x00ffff00) >> 8,
      // '78 56' 34 12 - We extract first 16 bits
      2 => (word & 0xffff0000) >> 16,
      3 => {
        // This case introduces misalignment; load next word and merge with it.
        let next_word = memory.read(word_addr + 1);
        ((next_word & 0xff) << 8) + ((word & 0xff000000) >> 24)
      },
      _ => 0,
    }) as u16
  }

  fn write_mem(memory: &mut dyn Memory, byte_addr: usize, value: u16) -> () {
    let word_addr = byte_addr >> 2;
    let step = byte_addr & 3;
    let mut word = memory.read(word_addr);
    match step {
      0 => {
        word &= 0xffff0000;
        word |= value as u32;
      },
      1 => {
        word &= 0xff0000ff;
        word |= (value as u32) << 8;
      },
      2 => {
        word &= 0x0000ffff;
        word |= (value as u32) << 16;
      },
      3 => {
        word |= ((value as u32) & 0xff) << 24;
        let mut next_word = memory.read(word_addr + 1) as u32;
        next_word &= 0xffffff00;
        next_word |= ((value as u32) & 0xff00) >> 8;
        memory.write(word_addr + 1, next_word);
      },
      _ => {},
    }
    memory.write(word_addr, word);
  }
}

#[cfg(test)]
mod tests {
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
