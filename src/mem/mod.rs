pub mod callback;
pub mod linear;
pub mod paged;

pub trait Memory {
  fn read(&self, word_addr: usize) -> u32;
  fn write(&mut self, word_addr: usize, value: u32) -> ();
  fn read_u8(&self, byte_addr: usize) -> u8 {
    // x86 uses little endian; 12 34 56 78 would be saved as 78 56 34 12.
    // This means that the step has to be inverted.
    let word_addr = byte_addr >> 2;
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

pub trait MemoryValue {
  fn read_mem(memory: &dyn Memory, byte_addr: usize) -> Self;
  fn write_mem(memory: &mut dyn Memory, byte_addr: usize, value: Self) -> ();
}

impl MemoryValue for u8 {
  fn read_mem(memory: &dyn Memory, byte_addr: usize) -> u8 {
    memory.read_u8(byte_addr)
  }
  fn write_mem(memory: &mut dyn Memory, byte_addr: usize, value: u8) -> () {
    memory.write_u8(byte_addr, value)
  }
}

impl MemoryValue for u16 {
  fn read_mem(memory: &dyn Memory, byte_addr: usize) -> u16 {
    memory.read_u16(byte_addr)
  }
  fn write_mem(memory: &mut dyn Memory, byte_addr: usize, value: u16) -> () {
    memory.write_u16(byte_addr, value)
  }
}
