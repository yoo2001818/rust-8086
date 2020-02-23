mod mem;
mod i8086;

use mem::linear::LinearMemory;
use i8086::cpu::CPU;

fn create_cpu() -> CPU {
  // 1MB
  let memory = LinearMemory::new(1024 * 1024);
  let io_map = LinearMemory::new(0);
  CPU::new(Box::new(memory), Box::new(io_map))
}

fn main() {
    let mut cpu = create_cpu();
    cpu.memory.write(0, 0xf000);
    cpu.step();
    println!("Hello, world!");
}
