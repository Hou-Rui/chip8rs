use qmetaobject::{prelude::*, QVariantList};
use rand::random;
use std::fs;

use crate::asm::Op;
use crate::mem::Mem;

const RAM_MAX: usize = 4096;
const REG_MAX: usize = 16;
const STACK_MAX: usize = 16;
const KEYPAD_MAX: usize = 16;

const VIDEO_WIDTH: u16 = 64;
const VIDEO_HEIGHT: u16 = 32;
const VIDEO_MAX: usize = VIDEO_WIDTH as usize * VIDEO_HEIGHT as usize;

const ADDR_FONT: u16 = 0x050;
const ADDR_START: u16 = 0x200;

const FONTSET_MAX: usize = 80;
const FONTSET_SIZE: u16 = 5;
const DEFAULT_FONTSET: [u8; FONTSET_MAX] = [
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

#[derive(QObject, Default)]
pub struct Chip8 {
    // Qt
    base: qt_base_class!(trait QObject),
    video_property: qt_property!(QVariantList; ALIAS video NOTIFY video_changed),
    video_changed: qt_signal!(),
    cycle: qt_method!(fn(&mut self)),
    reset: qt_method!(fn(&mut self)),
    load: qt_method!(fn(&mut self, path: QString)),

    // other things
    reg: Mem<u8, u8, REG_MAX>,
    stack: Mem<u8, u16, STACK_MAX>,
    ram: Mem<u16, u8, RAM_MAX>,
    video: Mem<u16, bool, VIDEO_MAX>,
    keypad: Mem<u8, bool, KEYPAD_MAX>,
    pc: u16,
    sp: u8,
    index: u16,
    dt: u8,
    st: u8,
}

impl Chip8 {
    pub fn reset(&mut self) {
        self.init_ram();
        self.update_video();
        self.pc = ADDR_START;
    }

    fn init_ram(&mut self) {
        self.ram.clear();
        for (i, d) in DEFAULT_FONTSET.iter().enumerate() {
            self.ram[ADDR_FONT + i as u16] = *d;
        }
    }

    fn fetch(&mut self) -> u16 {
        let b1 = self.ram[self.pc];
        let b2 = self.ram[self.pc + 1];
        self.pc += 2;
        u16::from_be_bytes([b1, b2])
    }

    fn update_video(&mut self) {
        self.video_property = QVariantList::from_iter(self.video.iter());
        self.video_changed();
    }

    fn exec_op(&mut self, op: Op) {
        match op {
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
                self.reg[reg] = self.reg[reg].wrapping_add(value);
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
                self.reg[reg1] = result as u8;
                self.reg[0xF] = if result > 0xFF { 1 } else { 0 };
            }
            Op::SUB { reg1, reg2 } => {
                let (r1, r2) = (self.reg[reg1], self.reg[reg2]);
                self.reg[reg1] = r1.wrapping_sub(r2);
                self.reg[0xF] = if r1 >= r2 { 1 } else { 0 };
            }
            Op::SHR { reg1, reg2 } => {
                let (r1, r2) = (self.reg[reg1], self.reg[reg2]);
                self.reg[reg1] >>= 1;
                self.reg[0xF] = r1 & 0x1;
                // todo!("SHR variant using both x and y");
            }
            Op::SUBN { reg1, reg2 } => {
                let (r1, r2) = (self.reg[reg1], self.reg[reg2]);
                self.reg[reg1] = r2.wrapping_sub(r1);
                self.reg[0xF] = if r1 <= r2 { 1 } else { 0 };
            }
            Op::SHL { reg1, reg2 } => {
                let (r1, r2) = (self.reg[reg1], self.reg[reg2]);
                self.reg[reg1] <<= 1;
                self.reg[0xF] = (r1 & 0x80) >> 7;
                // todo!("SHL variant using both x and y");
            }
            Op::SNE { reg1, reg2 } => {
                if self.reg[reg1] != self.reg[reg2] {
                    self.pc += 2;
                }
            }
            Op::LDIX { addr } => {
                self.index = addr;
            }
            Op::JPA { addr } => {
                let offset = self.reg[0x0] as u16;
                self.pc = addr + offset;
            }
            Op::RND { reg, value } => {
                self.reg[reg] = random::<u8>() & value;
            }
            Op::DRW { reg1, reg2, size } => {
                let x = self.reg[reg1] as u16 % VIDEO_WIDTH;
                let y = self.reg[reg2] as u16 % VIDEO_HEIGHT;
                self.reg[0xF] = 0;
                for row in 0..size as u16 {
                    let sprite = self.ram[self.index + row];
                    for col in 0..8 {
                        let sprite_pixel = (sprite & (0x80 >> col)) != 0;
                        let video_index = (y + row) * VIDEO_WIDTH + x + col;
                        if sprite_pixel {
                            if self.video[video_index] {
                                self.reg[0xF] = 1;
                            }
                            self.video[video_index] ^= true;
                        }
                    }
                }
                self.update_video();
            }
            Op::SKP { reg } => {
                if self.keypad[self.reg[reg]] {
                    self.pc += 2;
                }
            }
            Op::SKNP { reg } => {
                if !self.keypad[self.reg[reg]] {
                    self.pc += 2;
                }
            }
            Op::LDRD { reg } => {
                self.reg[reg] = self.dt;
            }
            Op::LDK { reg } => {
                match self.keypad.iter().position(|&pressed| pressed) {
                    Some(i) => self.reg[reg] = i as u8,
                    None => self.pc -= 2,
                };
            }
            Op::LDDR { reg } => {
                self.dt = self.reg[reg];
            }
            Op::LDST { reg } => {
                self.st = self.reg[reg];
            }
            Op::ADIX { reg } => {
                self.index += self.reg[reg] as u16;
            }
            Op::LDF { reg } => {
                let offset = FONTSET_SIZE * self.reg[reg] as u16;
                self.index = ADDR_FONT + offset;
            }
            Op::LDB { reg } => {
                let v = self.reg[reg];
                self.ram[self.index] = v / 100;
                self.ram[self.index + 1] = v % 100 / 10;
                self.ram[self.index + 2] = v % 10;
            }
            Op::LDXR { reg } => {
                for i in 0..=reg {
                    self.ram[self.index + i as u16] = self.reg[i];
                }
            }
            Op::LDRX { reg } => {
                for i in 0..=reg {
                    self.reg[i] = self.ram[self.index + i as u16];
                }
            }
            Op::DATA { data } => {
                println!("Error: unknown opcode: {:#06x}", data)
            }
        }
    }

    pub fn load(&mut self, path: QString) {
        self.reset();
        match fs::read(path.to_string()) {
            Ok(data) => {
                for (i, d) in data.iter().enumerate() {
                    self.ram[ADDR_START + i as u16] = *d;
                }
            }
            Err(err) => {
                println!("Error: failed to read ROM: {:?}", err);
            }
        }
    }

    pub fn cycle(&mut self) {
        let code = self.fetch();
        let op = Op::from_raw(code);
        self.exec_op(op);
        for timer in [&mut self.dt, &mut self.st] {
            if *timer > 0 {
                *timer -= 1;
            }
        }
    }
}
