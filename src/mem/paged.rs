use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::BTreeMap;
use super::Memory;

pub struct PagedMemorySegment {
  start: usize,
  size: usize,
  memory: Box<RefCell<dyn Memory>>,
}

impl Ord for PagedMemorySegment {
  fn cmp(&self, other: &Self) -> Ordering {
    self.start.cmp(&other.start)
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
  fn insert_page(&mut self, entry: PagedMemorySegment) -> () {
    self.map.insert(entry.start, RefCell::new(entry));
  }
  fn remove_page(&mut self, start: usize) -> bool {
    match self.map.remove(&start) {
      Some(v) => true,
      None => false,
    }
  }
  fn get_page(&self, address: usize) -> &RefCell<PagedMemorySegment> {
  }
}

impl<'a> Memory for PagedMemory {
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
      let moved_first_mem = first_mem;
      let moved_second_mem = second_mem;
      match addr {
        100..=200 => Some(&moved_first_mem),
        1000..=1100 => Some(&moved_second_mem),
        _ => None,
      }
    }));
  }
}
