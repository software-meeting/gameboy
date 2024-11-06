use cpu::Cpu;
use memory::Memory;

mod cpu;
mod memory;

fn main() {
    let cpu = Cpu::new();
    let memory = Memory::new();

    println!("Hello, world!");
}
