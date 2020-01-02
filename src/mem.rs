pub trait Memory {
  fn new(size: usize) -> Self;
  fn read(&self, address: usize) -> i32;
  fn write(&mut self, address: usize, value: i32) -> ();
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
