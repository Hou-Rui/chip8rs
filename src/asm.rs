#[derive(Debug)]
pub enum Op {
    CLS,
    RET,
    JP { addr: u16 },
    CALL { addr: u16 },
    SEI { reg: u8, value: u8 },
    SNEI { reg: u8, value: u8 },
    SE { reg1: u8, reg2: u8 },
    LDI { reg: u8, value: u8 },
    ADDI { reg: u8, value: u8 },
    LD { reg1: u8, reg2: u8 },
    OR { reg1: u8, reg2: u8 },
    AND { reg1: u8, reg2: u8 },
    XOR { reg1: u8, reg2: u8 },
    ADD { reg1: u8, reg2: u8 },
    SUB { reg1: u8, reg2: u8 },
}

macro_rules! parse_naaa {
    ($op:path, $code:expr) => {{
        $op {
            addr: $code & 0x0FFF,
        }
    }};
}

macro_rules! parse_nxkk {
    ($op:path, $code:expr) => {{
        $op {
            reg: (($code & 0x0F00) >> 8) as u8,
            value: ($code & 0x00FF) as u8,
        }
    }};
}

macro_rules! parse_nxyn {
    ($op:path, $code:expr) => {{
        $op {
            reg1: (($code & 0x0F00) >> 8) as u8,
            reg2: (($code & 0x00F0) >> 4) as u8,
        }
    }};
}

impl Op {
    pub fn new(code: u16) -> Self {
        match code {
            0x0E00 => Self::CLS,
            0x0EEE => Self::RET,
            0x1000..=0x1FFF => parse_naaa!(Self::JP, code),
            0x2000..=0x2FFF => parse_naaa!(Self::CALL, code),
            0x3000..=0x3FFF => parse_nxkk!(Self::SEI, code),
            0x4000..=0x4FFF => parse_nxkk!(Self::SNEI, code),
            code if code & 0xF00F == 0x5000 => parse_nxyn!(Self::SE, code),
            0x6000..=0x6FFF => parse_nxkk!(Self::LDI, code),
            0x7000..=0x7FFF => parse_nxkk!(Self::ADDI, code),
            code if code & 0xF00F == 0x8000 => parse_nxyn!(Self::LD, code),
            code if code & 0xF00F == 0x8001 => parse_nxyn!(Self::OR, code),
            code if code & 0xF00F == 0x8002 => parse_nxyn!(Self::AND, code),
            code if code & 0xF00F == 0x8003 => parse_nxyn!(Self::XOR, code),
            code if code & 0xF00F == 0x8004 => parse_nxyn!(Self::ADD, code),
            code if code & 0xF00F == 0x8005 => parse_nxyn!(Self::SUB, code),
            _ => panic!("Unknown opcode {:?}", code),
        }
    }
}
