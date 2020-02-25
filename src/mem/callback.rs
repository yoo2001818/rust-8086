use super::Memory;

pub struct CallbackMemory {
  read_callback: Box<dyn Fn(usize) -> u32>,
  write_callback: Box<dyn FnMut(usize, u32) -> ()>,
}

impl CallbackMemory {
  pub fn new(
    read_callback: Box<dyn Fn(usize) -> u32>,
    write_callback: Box<dyn FnMut(usize, u32) -> ()>,
  ) -> CallbackMemory {
    CallbackMemory { read_callback, write_callback }
  }
}

impl Memory for CallbackMemory {
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
  use std::rc::Rc;

  #[test]
  fn test() {
    let last = Rc::new(RefCell::new((0, 0)));
    let get_value = |addr: usize| -> u32 { addr as u32 };
    let last_cp = last.clone();
    let set_value = move |addr: usize, value: u32| -> () {
      *last_cp.borrow_mut() = (addr, value);
    };
    let mut mem = CallbackMemory::new(
      Box::new(get_value),
      Box::new(set_value),
    );
    mem.write(1000, 0x7F7F7F7F);
    assert_eq!(*last.borrow(), (1000, 0x7F7F7F7F));
    assert_eq!(mem.read(1000), 1000);
  }
}
