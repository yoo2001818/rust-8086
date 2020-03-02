use std::cell::RefCell;
use std::collections::BTreeMap;
use super::Memory;

pub struct PagedMemorySegment {
  start: usize,
  size: usize,
  memory: Box<RefCell<dyn Memory>>,
}

impl PagedMemorySegment {
  pub fn new(
    start: usize,
    size: usize,
    memory: Box<RefCell<dyn Memory>>,
  ) -> PagedMemorySegment {
    PagedMemorySegment { start, size, memory }
  }
}

pub struct PagedMemory {
  cache: usize,
  map: BTreeMap<usize, RefCell<PagedMemorySegment>>,
}

impl PagedMemory {
  pub fn new() -> PagedMemory {
    PagedMemory {
      cache: 0,
      map: BTreeMap::new(),
    }
  }
  pub fn insert_page(&mut self, entry: PagedMemorySegment) -> () {
    self.map.insert(entry.start, RefCell::new(entry));
  }
  pub fn remove_page(&mut self, start: usize) -> bool {
    match self.map.remove(&start) {
      Some(_) => true,
      None => false,
    }
  }
  pub fn get_page(&self, address: usize) -> Option<&RefCell<PagedMemorySegment>> {
    let mut range = self.map.range(..=address);
    let item = match range.next_back() {
      Some(v) => v,
      None => return None,
    };
    let entry = item.1.borrow();
    if entry.start <= address && address < entry.start + entry.size {
      return Some(item.1);
    }
    return None;
  }
}

impl<'a> Memory for PagedMemory {
  fn read(&self, address: usize) -> u32 {
    let segment = match self.get_page(address) {
      Some(v) => v.borrow(),
      None => return 0,
    };
    let memory = segment.memory.borrow();
    memory.read(address - segment.start)
  }
  fn write(&mut self, address: usize, value: u32) -> () {
    let segment = match self.get_page(address) {
      Some(v) => v.borrow(),
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
  use std::rc::Rc;

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
    let mut mem = PagedMemory::new();
    mem.insert_page(first_mem);
    mem.insert_page(second_mem);
    assert_eq!(mem.read(100), 0);
    assert_eq!(mem.read(1050), 50);
    mem.write(105, 0x5353);
    assert_eq!(*last.borrow(), (0, 5, 0x5353));
    mem.write(1030, 0x2525);
    assert_eq!(*last.borrow(), (1, 30, 0x2525));
    assert_eq!(mem.read(200), 0);
    assert_eq!(mem.read(199), 99);
  }
}
