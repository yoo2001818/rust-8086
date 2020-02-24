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

#[cfg(test)]
mod tests {
  use crate::mem::*;
  use super::*;
  use std::cell::RefCell;

  #[test]
  fn test() {
    let last_addr = RefCell::new(0);
    let last_value = RefCell::new(0);
    let get_value = |addr: usize| -> u32 { addr as u32 };
    let mut set_value = |addr: usize, value: u32| -> () {
      *last_addr.borrow_mut() = addr;
      *last_value.borrow_mut() = value;
    };
    let mut mem = CallbackMemory::new(
      &get_value,
      &mut set_value,
    );
    mem.write(1000, 0x7F7F7F7F);
    assert_eq!(*last_addr.borrow(), 1000);
    assert_eq!(*last_value.borrow(), 0x7F7F7F7F);
    assert_eq!(mem.read(1000), 1000);
  }
}
