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
    LDID { addr: u16 },
    JPA { addr: u16 },
    //
    DRW { reg1: u8, reg2: u8, size: u8 },
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

impl Op {
    pub fn try_from_raw(code: u16) -> Result<Self, u16> {
        match code {
            0x0E00 => Ok(Self::CLS),
            0x0EEE => Ok(Self::RET),
            0x1000..=0x1FFF => Ok(parse_naaa!(Self::JP, code)),
            0x2000..=0x2FFF => Ok(parse_naaa!(Self::CALL, code)),
            0x3000..=0x3FFF => Ok(parse_nxkk!(Self::SEI, code)),
            0x4000..=0x4FFF => Ok(parse_nxkk!(Self::SNEI, code)),
            0x5000..=0x5FFF => match code & 0x000F {
                0x0 => Ok(parse_nxyn!(Self::SE, code)),
                _ => Err(code),
            },
            0x6000..=0x6FFF => Ok(parse_nxkk!(Self::LDI, code)),
            0x7000..=0x7FFF => Ok(parse_nxkk!(Self::ADDI, code)),
            0x8000..=0x8FFF => match code & 0x000F {
                0x0 => Ok(parse_nxyn!(Self::LD, code)),
                0x1 => Ok(parse_nxyn!(Self::OR, code)),
                0x2 => Ok(parse_nxyn!(Self::AND, code)),
                0x3 => Ok(parse_nxyn!(Self::XOR, code)),
                0x4 => Ok(parse_nxyn!(Self::ADD, code)),
                0x5 => Ok(parse_nxyn!(Self::SUB, code)),
                0x6 => Ok(parse_nxyn!(Self::SHR, code)),
                0x7 => Ok(parse_nxyn!(Self::SUBN, code)),
                0xE => Ok(parse_nxyn!(Self::SHL, code)),
                _ => Err(code),
            },
            0x9000..=0x9FFF => match code & 0x000F {
                0x0 => Ok(parse_nxyn!(Self::SNE, code)),
                _ => Err(code),
            },
            0xA000..=0xAFFF => Ok(parse_naaa!(Self::LDID, code)),
            0xB000..=0xBFFF => Ok(parse_naaa!(Self::JPA, code)),
            //
            0xD000..=0xDFFF => Ok(parse_nxys!(Self::DRW, code)),
            _ => Err(code)
        }
    }
}
