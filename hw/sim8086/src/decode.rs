use crate::buf::{Buf, BufferError};

const REG_TABLE: &[&str] = &[
    "al", "ax", "cl", "cx", "dl", "dx", "bl", "bx", "ah", "sp", "ch", "bp", "dh", "si", "bh", "di"
];
const EA_CALC_TABLE: &[&str] = &[
    "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx"
];
const OP_TABLE: &[&str] = &[
    "add", "or", "adc", "sbb", "and", "sub", "xor", "cmp" 
];
const RET_TABLE: &[&str] = &[
    "jo", "jno", "jb", "jnb", "je", "jne", "jbe", "jnbe", "js", "jns", "jp", "jnp", "jl", "jnl", "jle", "jnle", "loopnz", "loopz", "loop", "jcxz"
];

#[derive(Debug)]
pub enum DecodeError {
    BufferError(BufferError),
    InvalidMode,
    InvalidDBitValue,
    InvalidLabelRelPos,
    InvalidSWBitValue,
    UnknownInstruction
}

impl From<BufferError> for DecodeError {
    fn from(err: BufferError) -> Self {
        DecodeError::BufferError(err)
    }
}

#[repr(u8)]
enum IFlags {
    Acc = 0b00000001,
    Imm = 0b00000010,
    Op = 0b00000100,
}

#[derive(Default)]
struct Instruction {
    name: String,
    d: u8,
    w: u8,
    s: u8,
    reg: u8,
    flags: u8
}

fn decode_special(inst: Instruction, buf: &mut Buf) -> Result<String, DecodeError> {
    let data_fmt: String;
    let reg_table_idx: usize;

    if inst.flags & IFlags::Acc as u8 == IFlags::Acc as u8 {
        reg_table_idx = inst.w as usize;
    } else {
        reg_table_idx = (2 * inst.reg + inst.w) as usize;
    }

    if inst.flags & IFlags::Imm as u8 == IFlags::Imm as u8 {
        match inst.w {
            0 => {
                let data: i16 = buf.read_i8()?;
                data_fmt = format!("{}", data);
            },
            1 => {
                let data: i16 = buf.read_i16()?;
                data_fmt = format!("{}", data);  
            },
            _ => {
                return Err(DecodeError::InvalidSWBitValue);
            },
        }
    } else {
        // address
        let data: u16 = buf.read_u16()?;
        data_fmt = format!("[{}]", data);  
    }

    // if d bit is 1, addr is dest, if d bit is 0, AX/L register is dest.
    let reg_names: &[&str] = &[REG_TABLE[reg_table_idx], &format!("{}", data_fmt)];
    return Ok(format!(
        "{} {}, {}",
        inst.name,
        reg_names[(inst.d % 2) as usize],
        reg_names[((inst.d + 1) % 2) as usize]
    ));
}

fn decode_mrm(mut inst: Instruction, buf: &mut Buf) -> Result<String, DecodeError> {
    let mrm_byte: u8 = buf.read_u8()?;

    let mode: u8 = (mrm_byte >> 6) & 0x3;
    let reg: u8 = (mrm_byte >> 3) & 0x7;
    let rm: u8 = mrm_byte & 0x7;
    let rm_reg_name: String;

    let is_imm: bool = inst.flags & IFlags::Imm as u8 == IFlags::Imm as u8;
    let is_op: bool = inst.flags & IFlags::Op as u8 == IFlags::Op as u8;

    if is_op && is_imm {
        inst.name = format!("{}", OP_TABLE[reg as usize]);
    }

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
            rm_reg_name = format!("{}", REG_TABLE[(2 * rm + inst.w) as usize]);
        }
        _ => {
            return Err(DecodeError::InvalidMode);
        }
    }

    if is_imm {
        // Immediate mode needs to support writing up to two data bytes to the R/M reg.
        let data_fmt: String;
        match (inst.s, inst.w) {
            (0, 0) | (1, 0) => {
                let data: u8 = buf.read_u8()?;
                data_fmt = format!("byte {}", data);
            },
            (0, 1) => {
                let data: u16 = buf.read_u16()?;
                data_fmt = format!("word {}", data);  
            },
            (1, 1) => {
                let data: u8 = buf.read_u8()?;
                data_fmt = format!("word {}", data);
            }
            _ => {
                return Err(DecodeError::InvalidSWBitValue);
            },
        }

        return Ok(format!("{} {}, {}", inst.name, rm_reg_name, data_fmt));
    } else {
        // if d bit is 1, REG register is dest, if d bit is 0, R/M register is dest.
        let reg_names: &[&str] = &[&rm_reg_name, REG_TABLE[(2 * reg + inst.w) as usize]];
        return Ok(format!(
            "{} {}, {}",
            inst.name,
            reg_names[(inst.d % 2) as usize],
            reg_names[((inst.d + 1) % 2) as usize]
        ));
    }
}

pub fn decode(mut buf: Buf) -> Result<Vec<String>, DecodeError> {
    let mut insts: Vec<String> = Vec::new();
    let mut labels: Vec<(usize, String)> = Vec::new();

    while !buf.is_at_end() {
        let inst_byte: u8 = buf.read_u8()?;
        let mut inst: Instruction = Default::default();

        match inst_byte {
            // Reg/mem/acc ALU ops, instruction byte contains octal op identifier.
            0x00..=0x05 | 0x28..=0x2D | 0x38..=0x3D => {
                inst.name = format!("{}", OP_TABLE[((inst_byte >> 3) & 0x7) as usize]);
                inst.d = (inst_byte >> 1) & 0x1;
                inst.w = inst_byte & 0x1;
                inst.flags = IFlags::Op as u8;
                
                if (inst_byte >> 2) & 0x1 == 0 {
                    insts.push(decode_mrm(inst, &mut buf)?);  
                } else {
                    inst.flags |= IFlags::Acc as u8 | IFlags ::Imm as u8;
                    insts.push(decode_special(inst, &mut buf)?);  
                }
            },
            // Immediate ALU ops, reg field in second byte repurposed to contain octal op identifier.
            0x80..=0x83 => {
                inst.s = (inst_byte >> 1) & 0x1;
                inst.w = inst_byte & 0x1;
                inst.flags = IFlags::Imm as u8 | IFlags::Op as u8;

                insts.push(decode_mrm(inst, &mut buf)?);      
            },
            // Reg/mem/acc + immediate MOV.
            0x88..=0x8B | 0xA0..=0xA3 | 0xC6..=0xC7 => {
                inst.name = "mov".to_string();
                inst.d = (inst_byte >> 1) & 0x1;
                inst.w = inst_byte & 0x1;
                if (inst_byte & 0xC6) == 0xC6 {
                    inst.flags = IFlags::Imm as u8;
                } else if (inst_byte & 0xA0) == 0xA0 {
                    inst.flags = IFlags::Acc as u8;
                }
                
                if inst.flags & IFlags::Acc as u8 == IFlags::Acc as u8 {
                    insts.push(decode_special(inst, &mut buf)?);
                } else {
                    insts.push(decode_mrm(inst, &mut buf)?);
                }
            },
            // Immediate to reg MOV, special shortened encoding.
            0xB0..=0xBF => {
                inst.name = "mov".to_string();
                inst.w = (inst_byte >> 3) & 0x1;
                inst.reg = inst_byte & 0x7;
                inst.flags = IFlags::Imm as u8;

                insts.push(decode_special(inst, &mut buf)?);
            },
            // Conditional jumps + label management
            0x70..=0x7F | 0xE0..=0xE3 => {
                let mut ret_table_idx: usize = (inst_byte & 0xF) as usize;
                if inst_byte & 0xF0 == 0xE0 {
                    ret_table_idx += 0x10;
                }
                inst.name = format!("{}", RET_TABLE[ret_table_idx as usize]);
    
                let rpos: i16 = buf.read_i8()? / 2;
                let lpos: usize = if rpos >= 1 {
                    insts.len().saturating_add((rpos.abs() + 1) as usize)
                } else if rpos <= -1 {
                    insts.len().saturating_sub((rpos.abs() - 1) as usize)
                } else {
                    return Err(DecodeError::InvalidLabelRelPos)
                };
                let mut label: String = Default::default();
                for (p, l) in &labels {
                    if *p == lpos {
                        label = l.clone();
                    }
                }
                if label.is_empty() {
                    label = format!("label_{}", labels.len() + 1);
                    labels.push((lpos, label.clone()));
                }

                insts.push(format!("{} {}", inst.name, label));
            }
            _ => {
                return Err(DecodeError::UnknownInstruction);
            },
        };
    }

    let asm_len: usize = insts.len() + labels.len();
    let mut asm: Vec<String> = vec![Default::default(); asm_len];
    let mut cur_label_idx: usize = 0;
    let mut cur_inst_idx: usize = 0;
    labels.sort_by(|a, b| a.0.cmp(&b.0));
    for (p, l) in labels {
        asm[p + cur_label_idx] = format!("{}:", l);
        cur_label_idx += 1;
    }
    for i in 0..asm_len {
        if asm[i].is_empty() {
            asm[i] = insts[cur_inst_idx].clone();
            cur_inst_idx += 1;
        }
    }
    return Ok(asm);
}
