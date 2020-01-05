pub trait Memory {
  fn new(size: usize) -> Self;
  fn read(&self, word_addr: usize) -> u32;
  fn write(&mut self, word_addr: usize, value: u32) -> ();

  fn read_u8(&self, byte_addr: usize) -> u8 {
    let word_addr = byte_addr >> 2;
    // x86 uses little endian; 12 34 56 78 would be saved as 78 56 34 12.
    // This means that the step has to be inverted.
    let step = byte_addr & 3;
    let word = self.read(word_addr);
    (match step {
      0 => word & 0xff,
      1 => (word & 0xff00) >> 8,
      2 => (word & 0xff0000) >> 16,
      3 => (word & 0xff000000) >> 24,
      _ => 0,
    }) as u8
  }

  fn write_u8(&mut self, byte_addr: usize, value: u8) -> () {
    let word_addr = byte_addr >> 2;
    // x86 uses little endian; 12 34 56 78 would be saved as 78 56 34 12.
    // This means that the step has to be inverted.
    let step = byte_addr & 3;
    let mut word = self.read(word_addr);
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
    self.write(word_addr, word);
  }

  fn read_u16(&self, byte_addr: usize) -> u16 {
    let word_addr = byte_addr >> 2;
    let step = byte_addr & 3;
    let word = self.read(word_addr);
    (match step {
      0 => word & 0x0000ffff,
      1 => (word & 0x00ffff00) >> 8,
      // '78 56' 34 12 - We extract first 16 bits
      2 => (word & 0xffff0000) >> 16,
      3 => {
        // This case introduces misalignment; load next word and merge with it.
        let next_word = self.read(word_addr + 1);
        ((next_word & 0xff) << 8) + ((word & 0xff000000) >> 24)
      },
      _ => 0,
    }) as u16
  }

  fn write_u16(&mut self, byte_addr: usize, value: u16) -> () {
    let word_addr = byte_addr >> 2;
    let step = byte_addr & 3;
    let mut word = self.read(word_addr);
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
        // This case introduces misalignment...
        word &= 0x00ffffff;
        word |= ((value as u32) & 0xff) << 24;
        let mut next_word = self.read(word_addr + 1) as u32;
        next_word &= 0xffffff00;
        next_word |= ((value as u32) & 0xff00) >> 8;
        self.write(word_addr + 1, next_word);
      },
      _ => {},
    }
    self.write(word_addr, word);
  }
}

pub struct LinearMemory {
  words: Vec<u32>,
}

impl Memory for LinearMemory {
  fn new(size: usize) -> LinearMemory {
    let words = vec![0; size];
    LinearMemory { words }
  }
  fn read(&self, address: usize) -> u32 {
    self.words[address]
  }
  fn write(&mut self, address: usize, value: u32) -> () {
    self.words[address] = value
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
    assert_eq!(mem.read_u8(0), 0x78);
    assert_eq!(mem.read_u8(1), 0x56);
    assert_eq!(mem.read_u8(2), 0x34);
    assert_eq!(mem.read_u8(3), 0x12);
    assert_eq!(mem.read_u8(4), 0x00);
    assert_eq!(mem.read_u8(5), 0xef);
    assert_eq!(mem.read_u8(6), 0xcd);
    assert_eq!(mem.read_u8(7), 0xab);
  }

  #[test]
  fn test_u16() {
    let mut mem = LinearMemory::new(1024);
    mem.write(0, 0x12345678);
    mem.write(1, 0xabcdef00);
    assert_eq!(mem.read_u16(0), 0x5678);
    assert_eq!(mem.read_u16(1), 0x3456);
    assert_eq!(mem.read_u16(2), 0x1234);
    assert_eq!(mem.read_u16(3), 0x0012);
    assert_eq!(mem.read_u16(4), 0xef00);
    assert_eq!(mem.read_u16(5), 0xcdef);
    assert_eq!(mem.read_u16(6), 0xabcd);
    assert_eq!(mem.read_u16(7), 0x00ab);
  }
}