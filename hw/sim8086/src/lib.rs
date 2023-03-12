#[derive(Debug)]
pub enum BufferError {
    ReadOverrun,
}

pub struct Buf {
    bytes: Vec<u8>,
    pos: usize,
}

impl Buf {
    pub fn new(bytes: Vec<u8>) -> Buf {
        Buf {
            bytes,
            pos: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        return self.pos >= self.bytes.len();
    }

    fn step(&mut self, steps: usize) {
        self.pos += steps;
    }

    fn read_u8(&mut self) -> Result<u8, BufferError> {
        if self.pos >= self.bytes.len() {
            return Err(BufferError::ReadOverrun);
        }

        let res: u8 = self.bytes[self.pos];
        self.step(1);
        return Ok(res);
    }

    fn read_i8(&mut self) -> Result<i16, BufferError> {
        if self.pos >= self.bytes.len() {
            return Err(BufferError::ReadOverrun);
        }

        let res: i16 = ((self.bytes[self.pos] as i16) << 8) >> 8;
        self.step(1);
        return Ok(res);
    }

    fn read_u16(&mut self) -> Result<u16, BufferError> {
        if self.pos + 1 >= self.bytes.len() {
            return Err(BufferError::ReadOverrun);
        }

        let mut res: u16 = self.bytes[self.pos] as u16;
        self.step(1);
        res |= (self.bytes[self.pos] as u16) << 8;
        self.step(1);
        return Ok(res);
    }

    fn read_i16(&mut self) -> Result<i16, BufferError> {
        if self.pos + 1 >= self.bytes.len() {
            return Err(BufferError::ReadOverrun);
        }

        let mut res: i16 = self.bytes[self.pos] as i16;
        self.step(1);
        res |= (self.bytes[self.pos] as i16) << 8;
        self.step(1);
        return Ok(res);
    }
}

const REG_TABLE: &[&str] = &[
    "al", "ax", "cl", "cx", "dl", "dx", "bl", "bx", "ah", "sp", "ch", "bp", "dh", "si", "bh", "di",
];
const EA_CALC_TABLE: &[&str] = &[
    "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
];
const INST_STR: &str = "mov";

#[derive(Debug)]
pub enum DecodeError {
    BufferError(BufferError),
    InvalidMode,
    InvalidWBitValue,
    UnknownInstruction
}

impl From<BufferError> for DecodeError {
    fn from(err: BufferError) -> Self {
        DecodeError::BufferError(err)
    }
}

pub fn decode(mut buf: Buf) -> Result<Vec<String>, DecodeError> {
    let mut insts = Vec::new();

    while !buf.is_at_end() {
        let instr_byte: u8 = buf.read_u8()?;

        // MOV Register/memory to/from register and Immediate to register/memory instructions
        if (instr_byte & 0xFC) == 0x88 || (instr_byte & 0xC6) == 0xC6 {
            let is_imm: bool = (instr_byte & 0xC6) == 0xC6;

            let d: u8 = (instr_byte >> 1) & 0x1;
            let w: u8 = instr_byte & 0x1;
            
            let mrm_byte: u8 = buf.read_u8()?;

            let mode: u8 = (mrm_byte >> 6) & 0x3;
            let reg: u8 = (mrm_byte >> 3) & 0x7;
            let rm: u8 = mrm_byte & 0x7;
            let rm_reg_name: String;

            match mode {
                // Effective address without DISP
                0x0 => {
                    if rm != 0x6 {
                        rm_reg_name = format!("[{}]", EA_CALC_TABLE[rm as usize]);
                    } else {
                        // Special direct-addressing case
                        let addr = buf.read_u16()?;
                        rm_reg_name = format!("[{}]", addr.to_string());
                    }
                }
                // Effective address with DISP
                0x1 | 0x2 => {
                    let disp: i16;
                    let mut disp_fmt = String::new();
                    if mode == 0x2 {
                        disp = buf.read_i16()?;
                    } else {
                        disp = buf.read_i8()?;
                    }
                    if disp > 0 {
                        disp_fmt = format!(" + {}", disp);
                    } else if disp < 0 {
                        disp_fmt = format!(" - {}", disp.abs());
                    }

                    rm_reg_name = format!("[{}{}]", EA_CALC_TABLE[rm as usize], disp_fmt);
                }
                // Register-to-register
                0x3 => {
                    rm_reg_name = format!("{}", REG_TABLE[(2 * rm + w) as usize]);
                }
                _ => {
                    return Err(DecodeError::InvalidMode);
                }
            }

            if is_imm {
                // Immediate mode needs to support writing up to two data bytes to the R/M reg.
                let data_fmt: String;
                if w == 0 {
                    let data: u8 = buf.read_u8()?;
                    data_fmt = format!("byte {}", data);
                } else if w == 1 {
                    let data: u16 = buf.read_u16()?;
                    data_fmt = format!("word {}", data);
                } else {
                    return Err(DecodeError::InvalidWBitValue);
                }
                insts.push(format!("{} {}, {}", INST_STR, rm_reg_name, data_fmt));
            } else {
                // if d bit is 1, REG register is dest, if d bit is 0, R/M register is dest.
                let reg_names: &[&str] = &[&rm_reg_name, REG_TABLE[(2 * reg + w) as usize]];
                insts.push(format!(
                    "{} {}, {}",
                    INST_STR,
                    reg_names[(d % 2) as usize],
                    reg_names[((d + 1) % 2) as usize]
                ));
            }
        }
        // MOV Immediate to register instruction
        else if (instr_byte & 0xF0) == 0xB0 {
            let w: u8 = (instr_byte >> 3) & 0x1;
            let reg: u8 = instr_byte & 0x7;
            let reg_name: &str = REG_TABLE[(2 * reg + w) as usize];

            let data: u16;
            if w == 0 {
                data = buf.read_u8()? as u16;
            } else if w == 1 {
                data = buf.read_u16()?;
            } else {
                return Err(DecodeError::InvalidWBitValue);
            }
            insts.push(format!("{} {}, {}", INST_STR, reg_name, data));
        }
        // MOV Memory to/from accumulator
        else if (instr_byte & 0xA0) == 0xA0 {
            let d: u8 = (instr_byte >> 1) & 0x1;
            let w: u8 = instr_byte & 0x1;

            let addr: i16 = buf.read_i16()?;

            // if d bit is 1, addr is dest, if d bit is 0, AX/L register is dest.
            let reg_names: &[&str] = &[REG_TABLE[w as usize], &format!("[{}]", addr.to_string())];
            insts.push(format!(
                "{} {}, {}",
                INST_STR,
                reg_names[(d % 2) as usize],
                reg_names[((d + 1) % 2) as usize]
            ));
        } else {
            return Err(DecodeError::UnknownInstruction);
        }
    }

    return Ok(insts);
}
