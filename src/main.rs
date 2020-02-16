mod mem;
mod i8086;

use mem::Memory;
use mem::LinearMemory;
use i8086::cpu::CPU;

fn create_cpu() -> CPU {
  // 1MB
  let memory = mem::LinearMemory::new(1024 * 1024);
  CPU::new(memory)
}

fn main() {
    let mut cpu = create_cpu();
    cpu.memory.write(0, 0xf000);
    cpu.step();
    println!("Hello, world!");
}
