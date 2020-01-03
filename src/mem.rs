pub trait Memory {
  fn new(size: usize) -> Self;
  fn read(&self, address: usize) -> i32;
  fn write(&mut self, address: usize, value: i32) -> ();

  fn read_i8(&self, address: usize) -> i8 {
    let wordAddress = address >> 2;
    let step = address & 3;
    (read(wordAddress) >> (step << 3) & 0xff)
  }

  fn write_i8(&mut self, address: usize, value: i8) -> () {

  }

  fn read_i16(&self, address: usize) -> i16 {

  }

  fn write_i16(&mut self, address: usize, value: i16) -> () {

  }
}

pub struct LinearMemory {
  words: Vec<i32>,
}

impl Memory for LinearMemory {
  fn new(size: usize) -> LinearMemory {
    let words = vec![0; size];
    LinearMemory { words }
  }
  fn read(&self, address: usize) -> i32 {
    self.words[address]
  }
  fn write(&mut self, address: usize, value: i32) -> () {
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
}
