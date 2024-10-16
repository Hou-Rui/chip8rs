mod asm;
mod mem;
use crate::asm::Op;
use crate::mem::Mem;

const RAM_MAX: usize = 4096;
const REG_MAX: usize = 16;
const STACK_MAX: usize = 16;
const KEYPAD_MAX: usize = 16;
const VIDEO_WIDTH: usize = 64;
const VIDEO_HEIGHT: usize = 32;
const VIDEO_MAX: usize = VIDEO_WIDTH * VIDEO_HEIGHT;

const FONTSET_SIZE: usize = 80;
const DEFAULT_FONTSET: [u8; FONTSET_SIZE] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[derive(Default)]
struct Chip8 {
    reg: Mem<u8, u8, REG_MAX>,
    stack: Mem<u8, u16, STACK_MAX>,
    ram: Mem<u16, u8, RAM_MAX>,
    video: Mem<u16, bool, VIDEO_MAX>,
    keypad: Mem<u8, bool, KEYPAD_MAX>,
    pc: u16,
    sp: u8,
    index: u16,
    delay_timer: u8,
    sound_timer: u8,
}

impl Chip8 {
    pub fn new() -> Self {
        let mut chip8 = Self::default();
        chip8.reset();
        chip8
    }

    pub fn reset(&mut self) {
        self.init_ram();
        self.pc = 0x200;
    }

    fn init_ram(&mut self) {
        const FONT_START_ADDR: u16 = 0x50;
        for i in 0..FONTSET_SIZE as u16 {
            self.ram[FONT_START_ADDR + i] = DEFAULT_FONTSET[i as usize];
        }
    }

    fn fetch(&mut self) -> u16 {
        let b1 = self.ram[self.pc];
        let b2 = self.ram[self.pc + 1];
        self.pc += 2;
        u16::from_ne_bytes([b1, b2])
    }

    pub fn run(&mut self) {
        let code = self.fetch();
        match Op::new(code) {
            Op::CLS => {
                self.video.clear();
            }
            Op::RET => {
                self.sp -= 1;
                self.pc = self.stack[self.sp];
            }
            Op::JP { addr } => {
                self.pc = addr;
            }
            Op::CALL { addr } => {
                self.stack[self.sp] = self.pc;
                self.sp += 1;
                self.pc = addr;
            }
            Op::SEI { reg, value } => {
                if self.reg[reg] == value {
                    self.pc += 2;
                }
            }
            Op::SNEI { reg, value } => {
                if self.reg[reg] != value {
                    self.pc += 2;
                }
            }
            Op::SE { reg1, reg2 } => {
                if self.reg[reg1] == self.reg[reg2] {
                    self.pc += 2;
                }
            }
            Op::LDI { reg, value } => {
                self.reg[reg] = value;
            }
            Op::ADDI { reg, value } => {
                self.reg[reg] += value; // natural overflow
            }
            Op::LD { reg1, reg2 } => {
                self.reg[reg1] = self.reg[reg2];
            }
            Op::OR { reg1, reg2 } => {
                self.reg[reg1] |= self.reg[reg2];
            }
            Op::AND { reg1, reg2 } => {
                self.reg[reg1] &= self.reg[reg2];
            }
            Op::XOR { reg1, reg2 } => {
                self.reg[reg1] ^= self.reg[reg2];
            }
            Op::ADD { reg1, reg2 } => {
                let result = self.reg[reg1] as u16 + self.reg[reg2] as u16;
                self.reg[0xF] = if result > 0xFF { 0 } else { 1 };
                self.reg[reg1] += result as u8;
            }
            Op::SUB { reg1, reg2 } => {
                let (r1, r2) = (self.reg[reg1], self.reg[reg2]);
                self.reg[0xF] = if r1 > r2 { 1 } else { 0 };
                self.reg[reg1] -= r2;
            }
        }
    }
}

fn main() {
    let mut chip8 = Chip8::new();
    chip8.run();
}
