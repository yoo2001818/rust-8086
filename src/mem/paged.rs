use std::cell::RefCell;
use std::rc::Rc;
use super::Memory;

pub struct PagedMemorySegment {
  start: usize,
  size: usize,
  memory: Box<RefCell<dyn Memory>>,
}

pub struct PagedMemory<'a> {
  cache: RefCell<Option<&'a PagedMemorySegment>>,
  get_item: Box<dyn Fn(usize) -> Option<&'a PagedMemorySegment>>,
}

impl<'a> PagedMemory<'a> {
  pub fn new(
    get_item: Box<dyn Fn(usize) -> Option<&'a PagedMemorySegment>>,
  ) -> PagedMemory {
    PagedMemory {
      cache: RefCell::new(None),
      get_item,
    }
  }
  fn get_page(&self, address: usize) -> &RefCell<Option<&'a PagedMemorySegment>> {
    match *self.cache.borrow() {
      Some(entry) => {
        if entry.start >= address && entry.start + entry.size < address {
          return &self.cache;
        }
      },
      _ => {},
    };
    let get_item = &self.get_item;
    let entry = get_item(address);
    self.cache.replace(entry);
    &self.cache
  }
}

impl<'a> Memory for PagedMemory<'a> {
  fn read(&self, address: usize) -> u32 {
    let segment = match *self.get_page(address).borrow() {
      Some(v) => v,
      None => return 0,
    };
    let memory = segment.memory.borrow();
    memory.read(address - segment.start)
  }
  fn write(&mut self, address: usize, value: u32) -> () {
    let segment = match *self.get_page(address).borrow() {
      Some(v) => v,
      None => return,
    };
    let mut memory = segment.memory.borrow_mut();
    memory.write(address - segment.start, value)
  }
}

#[cfg(test)]
mod tests {
  use crate::mem::callback::*;
  use super::*;
  use std::cell::RefCell;

  #[test]
  fn test() {
    let last = Rc::new(RefCell::new((0, 0, 0)));
    let first_mem = {
      let last_cp = last.clone();
      let get_value = |addr: usize| -> u32 { addr as u32 };
      let set_value = move |addr: usize, value: u32| -> () {
        *last_cp.borrow_mut() = (0, addr, value);
      };
      PagedMemorySegment {
        memory: Box::new(RefCell::new(
          CallbackMemory::new(Box::new(get_value), Box::new(set_value)))),
        start: 100,
        size: 100,
      }
    };
    let second_mem = {
      let last_cp = last.clone();
      let get_value = |addr: usize| -> u32 { addr as u32 };
      let set_value = move |addr: usize, value: u32| -> () {
        *last_cp.borrow_mut() = (1, addr, value);
      };
      PagedMemorySegment {
        memory: Box::new(RefCell::new(
          CallbackMemory::new(Box::new(get_value), Box::new(set_value)))),
        start: 1000,
        size: 100,
      }
    };
    let mem = PagedMemory::new(Box::new(move |addr| {
      match addr {
        100..=200 => Some(&first_mem),
        1000..=1100 => Some(&second_mem),
        _ => None,
      }
    }));
  }
}
