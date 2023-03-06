const REG_TABLE: &[&str] = &[
    "al", "ax", "cl", "cx", "dl", "dx", "bl", "bx", "ah", "sp", "ch", "bp", "dh", "si", "bh", "di",
];
const EA_CALC_TABLE: &[&str] = &[
    "bx + si", "bx + di", "bp + si", "bp + di", "si", "di", "bp", "bx",
];
const INST_STR: &str = "mov";

// FIXME: no buffer length checks are currently performed.
pub fn decode(bytes: Vec<u8>) -> Vec<String> {
    let mut idx: usize = 0;
    let mut insts = Vec::new();

    while idx < bytes.len() {
        // MOV Register/memory to/from register and Immediate to register/memory instructions
        if (bytes[idx] & 0xFC) == 0x88 || (bytes[idx] & 0xC6) == 0xC6 {
            let is_imm: bool = (bytes[idx] & 0xC6) == 0xC6;

            let d: u8 = (bytes[idx] >> 1) & 0x1;
            let w: u8 = bytes[idx] & 0x1;

            idx += 1;

            let mode: u8 = (bytes[idx] >> 6) & 0x3;
            let reg: u8 = (bytes[idx] >> 3) & 0x7;
            let rm: u8 = bytes[idx] & 0x7;
            let rm_reg_name: String;

            match mode {
                // Effective address without DISP
                0x0 => {
                    if rm != 0x6 {
                        rm_reg_name = format!("[{}]", EA_CALC_TABLE[rm as usize]);
                    } else {
                        // Special direct-addressing case
                        idx += 1;
                        let mut addr: u16 = bytes[idx].into();
                        idx += 1;
                        addr = addr | ((bytes[idx] as u16) << 8);

                        rm_reg_name = format!("[{}]", addr.to_string());
                    }
                }
                // Effective address with DISP
                0x1 | 0x2 => {
                    idx += 1;
                    let mut disp: i16 = bytes[idx] as i16;
                    let mut disp_fmt = String::new();
                    if mode == 0x2 {
                        idx += 1;
                        disp = disp | ((bytes[idx] as i16) << 8);
                    } else {
                        disp = ((bytes[idx] as i16) << 8) >> 8;
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
                    panic!("Mode must've been incorrectly parsed. We should never reach this case!")
                }
            }

            if is_imm {
                // Immediate mode needs to support writing up to two data bytes to the R/M reg.
                idx += 1;
                let mut data: u16 = bytes[idx] as u16;
                let mut data_fmt = format!("byte {}", data);
                if w == 1 {
                    idx += 1;
                    data = data | ((bytes[idx] as u16) << 8);
                    data_fmt = format!("word {}", data);
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
        else if (bytes[idx] & 0xF0) == 0xB0 {
            let w: u8 = (bytes[idx] >> 3) & 0x1;
            let reg: u8 = bytes[idx] & 0x7;
            let reg_name: &str = REG_TABLE[(2 * reg + w) as usize];

            idx += 1;

            let mut data: u16 = bytes[idx] as u16;
            if w == 1 {
                idx += 1;
                data = data | ((bytes[idx] as u16) << 8);
            }

            insts.push(format!("{} {}, {}", INST_STR, reg_name, data));
        }
        // MOV Memory to/from accumulator
        else if (bytes[idx] & 0xA0) == 0xA0 {
            let d: u8 = (bytes[idx] >> 1) & 0x1;
            let w: u8 = bytes[idx] & 0x1;

            idx += 1;

            let mut addr: u16 = bytes[idx] as u16;
            idx += 1;
            addr = addr | ((bytes[idx] as u16) << 8);

            // if d bit is 1, addr is dest, if d bit is 0, AX/L register is dest.
            let reg_names: &[&str] = &[REG_TABLE[w as usize], &format!("[{}]", addr.to_string())];
            insts.push(format!(
                "{} {}, {}",
                INST_STR,
                reg_names[(d % 2) as usize],
                reg_names[((d + 1) % 2) as usize]
            ));
        } else {
            // FIXME: probably should have decode return an Error.
            panic!("Unsupported instruction found! Exiting...");
        }

        idx += 1;
    }

    return insts;
}
