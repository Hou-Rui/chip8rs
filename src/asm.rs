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
    SHR { reg1: u8, reg2: u8 },
    SUBN { reg1: u8, reg2: u8 },
    SHL { reg1: u8, reg2: u8 },
    SNE { reg1: u8, reg2: u8 },
    LDIX { addr: u16 },
    JPA { addr: u16 },
    RND { reg: u8, value: u8 },
    DRW { reg1: u8, reg2: u8, size: u8 },
    SKP { reg: u8 },
    SKNP { reg: u8 },
    LDRD { reg: u8 },
    LDK { reg: u8 },
    LDDR { reg: u8 },
    LDST { reg: u8 },
    ADIX { reg: u8 },
    LDF { reg: u8 },
    LDB { reg: u8 },
    LDXR { reg: u8 },
    LDRX { reg: u8 },
    DATA { data: u16 },
}

macro_rules! parse_naaa {
    ($op: path, $code: expr) => {{
        $op {
            addr: $code & 0x0FFF,
        }
    }}
}

macro_rules! parse_nxkk {
    ($op: path, $code: expr) => {{
        $op {
            reg: (($code & 0x0F00) >> 8) as u8,
            value: ($code & 0x00FF) as u8,
        }
    }}
}

macro_rules! parse_nxyn {
    ($op: path, $code: expr) => {{
        $op {
            reg1: (($code & 0x0F00) >> 8) as u8,
            reg2: (($code & 0x00F0) >> 4) as u8,
        }
    }};
}

macro_rules! parse_nxys {
    ($op: path, $code: expr) => {{
        $op {
            reg1: (($code & 0x0F00) >> 8) as u8,
            reg2: (($code & 0x00F0) >> 4) as u8,
            size: ($code & 0x000F) as u8,
        }
    }};
}

macro_rules! parse_nxnn {
    ($op: path, $code: expr) => {{
        $op {
            reg: (($code & 0x0F00) >> 8) as u8,
        }
    }};
}

impl Op {
    pub fn from_raw(code: u16) -> Self {
        match code {
            0x00E0 => Self::CLS,
            0x00EE => Self::RET,
            0x1000..=0x1FFF => parse_naaa!(Self::JP, code),
            0x2000..=0x2FFF => parse_naaa!(Self::CALL, code),
            0x3000..=0x3FFF => parse_nxkk!(Self::SEI, code),
            0x4000..=0x4FFF => parse_nxkk!(Self::SNEI, code),
            0x5000..=0x5FFF => match code & 0x000F {
                0x0 => parse_nxyn!(Self::SE, code),
                _ => Self::DATA { data: code }
            },
            0x6000..=0x6FFF => parse_nxkk!(Self::LDI, code),
            0x7000..=0x7FFF => parse_nxkk!(Self::ADDI, code),
            0x8000..=0x8FFF => match code & 0x000F {
                0x0 => parse_nxyn!(Self::LD, code),
                0x1 => parse_nxyn!(Self::OR, code),
                0x2 => parse_nxyn!(Self::AND, code),
                0x3 => parse_nxyn!(Self::XOR, code),
                0x4 => parse_nxyn!(Self::ADD, code),
                0x5 => parse_nxyn!(Self::SUB, code),
                0x6 => parse_nxyn!(Self::SHR, code),
                0x7 => parse_nxyn!(Self::SUBN, code),
                0xE => parse_nxyn!(Self::SHL, code),
                _ => Self::DATA { data: code }
            },
            0x9000..=0x9FFF => match code & 0x000F {
                0x0 => parse_nxyn!(Self::SNE, code),
                _ => Self::DATA { data: code }
            },
            0xA000..=0xAFFF => parse_naaa!(Self::LDIX, code),
            0xB000..=0xBFFF => parse_naaa!(Self::JPA, code),
            0xC000..=0xCFFF => parse_nxkk!(Self::RND, code),
            0xD000..=0xDFFF => parse_nxys!(Self::DRW, code),
            0xE000..=0xEFFF => match code & 0x00FF {
                0x9E => parse_nxnn!(Self::SKP, code),
                0xA1 => parse_nxnn!(Self::SKNP, code),
                _ => Self::DATA { data: code }
            }
            0xF000..=0xFFFF => match code & 0x00FF {
                0x07 => parse_nxnn!(Self::LDRD, code),
                0x0A => parse_nxnn!(Self::LDK, code),
                0x15 => parse_nxnn!(Self::LDDR, code),
                0x18 => parse_nxnn!(Self::LDST, code),
                0x1E => parse_nxnn!(Self::ADIX, code),
                0x29 => parse_nxnn!(Self::LDF, code),
                0x33 => parse_nxnn!(Self::LDB, code),
                0x55 => parse_nxnn!(Self::LDXR, code),
                0x65 => parse_nxnn!(Self::LDRX, code),
                _ => Self::DATA { data: code }
            }
            _ => Self::DATA { data: code }
        }
    }
}
