use std::cell::RefCell;
use std::rc::Rc;
use super::Memory;

pub struct PagedMemorySegment {
  start: usize,
  size: usize,
  memory: Rc<RefCell<dyn Memory>>,
}

pub struct PagedMemory<'a> {
  cache: RefCell<Option<&'a PagedMemorySegment>>,
  get_item: &'a dyn Fn(usize) -> Option<&'a PagedMemorySegment>,
}

impl<'a> PagedMemory<'a> {
  pub fn new(
    get_item: &'a dyn Fn(usize) -> Option<&'a PagedMemorySegment>,
  ) -> PagedMemory {
    PagedMemory {
      cache: RefCell::new(None),
      get_item: get_item,
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
    let get_item = self.get_item;
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
