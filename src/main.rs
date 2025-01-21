use cpu::Cpu;
use memory::Memory;
use minifb::{Scale, Window, WindowOptions};

mod cartridge;
mod cpu;
mod io;
mod memory;

const WIDTH: usize = 160;
const HEIGHT: usize = 144;

fn main() {
    let cpu = Cpu::new();
    let memory = Memory::new(Vec::new());

    let mut buffer: Vec<u32> = vec![0x0; WIDTH * HEIGHT];
    let opts = WindowOptions {
        scale: Scale::X4,
        ..WindowOptions::default()
    };
    let mut window = Window::new("gameboy", WIDTH, HEIGHT, opts).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);
    while window.is_open() {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
