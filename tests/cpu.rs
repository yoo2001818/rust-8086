extern crate rust_8086;

use std::cell::RefCell;
use std::rc::Rc;

use rust_8086::i8086::cpu::CPU;
use rust_8086::mem::linear::LinearMemory;
use rust_8086::mem::paged::*;
use rust_8086::mem::callback::CallbackMemory;
use rust_8086::mem::Memory;

fn create_cpu(io_map: Box<dyn Memory>) -> CPU {
  // 1MB
  let memory = LinearMemory::new(1024 * 1024);
  CPU::new(Box::new(memory), io_map)
}

#[test]
fn op_mov_imm() {
  let mut cpu = create_cpu(Box::new(PagedMemory::new()));
  let input: Vec<u8> = vec![
    // mov ax, 0x8086
    0xb8, 0x86, 0x80,
    // mov bl, al
    0x88, 0xc3,
    // mov cl, ah
    0x88, 0xe1,
  ];
  for (i, value) in input.iter().enumerate() {
    cpu.memory.write_u8(i, *value);
  }
  cpu.jmp(0, 0);
  cpu.step();
  assert_eq!(cpu.register.ax, 0x8086);
  cpu.step();
  assert_eq!(cpu.register.bx, 0x0086);
  cpu.step();
  assert_eq!(cpu.register.cx, 0x0080);
}

#[test]
fn op_mov_mem() {
  let mut cpu = create_cpu(Box::new(PagedMemory::new()));
  let input: Vec<u8> = vec![
    // mov [0x5353], 0xabcd,
    0xc7, 0x06, 0x53, 0x53, 0xcd, 0xab,
    // mov [0x2000], 0x8086
    0xc7, 0x06, 0x00, 0x20, 0x86, 0x80,
    // movw bx, [0x2000]
    0x8b, 0x1e, 0x00, 0x20,
    // movw [bx], 0x5353
    0xc7, 0x07, 0x53, 0x53,
    // movw bx, [bx]
    0x8b, 0x1f,
  ];
  for (i, value) in input.iter().enumerate() {
    cpu.memory.write_u8(i, *value);
  }
  cpu.jmp(0, 0);
  cpu.step();
  assert_eq!(cpu.memory.read_u16(0x5353), 0xabcd);
  cpu.step();
  assert_eq!(cpu.memory.read_u16(0x2000), 0x8086);
  cpu.step();
  assert_eq!(cpu.register.bx, 0x8086);
  cpu.step();
  assert_eq!(cpu.memory.read_u16(0x8086), 0x5353);
  cpu.step();
  assert_eq!(cpu.register.bx, 0x5353);
}

#[test]
fn op_push() {
  let mut cpu = create_cpu(Box::new(PagedMemory::new()));
  let input: Vec<u8> = vec![
    // mov [0x5353], 0xabcd,
    0xc7, 0x06, 0x53, 0x53, 0xcd, 0xab,
    // mov [0x2000], 0x8086
    0xc7, 0x06, 0x00, 0x20, 0x86, 0x80,
    // movw bx, [0x2000]
    0x8b, 0x1e, 0x00, 0x20,
    // movw [bx], 0x5353
    0xc7, 0x07, 0x53, 0x53,
    // movw bx, [bx]
    0x8b, 0x1f,
  ];
  for (i, value) in input.iter().enumerate() {
    cpu.memory.write_u8(i, *value);
  }
  cpu.jmp(0, 0);
}

#[test]
fn op_tests() {
  let failed_data: Rc<RefCell<Option<u32>>> =
    Rc::new(RefCell::new(None));
  let debugging: Rc<RefCell<bool>> = Rc::new(RefCell::new(false));
  let mut io_map = PagedMemory::new();
  {
    let failed_data_dup = failed_data.clone();
    let io_handler = CallbackMemory::new(
      Box::new(|_| 0),
      Box::new(move |_, value| {
        *failed_data_dup.borrow_mut() = Some(value);
      }),
    );
    io_map.insert_page(
      PagedMemorySegment::new(0, 1, Box::new(RefCell::new(io_handler))));
  }
  {
    let debugging_dup = debugging.clone();
    let io_handler = CallbackMemory::new(
      Box::new(|_| 0),
      Box::new(move |_, _| {
        *debugging_dup.borrow_mut() = true;
      }),
    );
    io_map.insert_page(
      PagedMemorySegment::new(1, 1, Box::new(RefCell::new(io_handler))));
  }
  let mut cpu = create_cpu(Box::new(io_map));
  let test_data = include_bytes!("tests.com");
  for (i, value) in test_data.into_iter().enumerate() {
    cpu.memory.write_u8(i + 0x100, *value);
  }
  cpu.jmp(0, 0x100);
  while cpu.running {
    let op = match cpu.next_op() {
      Some(v) => v,
      None => break,
    };
    cpu.exec_op(&op);
    if *debugging.borrow() {
      println!("{:#?} {:#?}", &op, &cpu.register);
    }
  }
  match *failed_data.borrow() {
    Some(err) => {
      panic!("Test failed at {:08X}\n{:#?}", err, &cpu.register);
    },
    None => (),
  };
}
