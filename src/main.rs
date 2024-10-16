mod asm;
mod cpu;
mod mem;

fn main() {
    let mut chip8 = cpu::Cpu::new();
    chip8.run();
}
