use super::Memory;

pub struct CallbackMemory<'a> {
  read_callback: &'a dyn Fn(usize) -> u32,
  write_callback: &'a mut dyn FnMut(usize, u32) -> (),
}

impl<'a> CallbackMemory<'a> {
  pub fn new(
    read_callback: &'a dyn Fn(usize) -> u32,
    write_callback: &'a mut dyn FnMut(usize, u32) -> (),
  ) -> CallbackMemory<'a> {
    CallbackMemory { read_callback, write_callback }
  }
}

impl Memory for CallbackMemory<'_> {
  fn read(&self, address: usize) -> u32 {
    (self.read_callback)(address)
  }
  fn write(&mut self, address: usize, value: u32) -> () {
    (self.write_callback)(address, value)
  }
}
