enum Instruction {
    MOV
}

fn get_instruction(byte: &u8 ) -> Instruction {
    let i = byte >> 2;
    return match i {
        0x22 => Instruction::MOV,
        _ => panic!("Unknown instruction: {}", i)
    }
}

fn decode_mov(u16_byte: &Vec<u8>) -> String {
    let w = u16_byte[0] & 0x1;
    let d = (u16_byte[0] & 0x2) >> 1;
    if d > 1 {
        panic!("d bit: {} is invalid", d);
    }
    
    // MOD
    // 00 -> memory mode
    // 11 -> register to register
    let mode = (u16_byte[1] & 0xC0) >> 6;
    if mode != 0x3 {
        panic!("mode: {mode} is not register to register");
    }

    let reg_a = decode_register((u16_byte[1] & 0x38) >> 3, w);
    let reg_b = decode_register(u16_byte[1] & 0x7, w);

    if d == 1 {
        return format!("MOV {}, {}", reg_a, reg_b).to_lowercase();
    } else {
        return format!("MOV {}, {}", reg_b, reg_a).to_lowercase();
    };
}

fn decode_register(reg_byte: u8, w: u8) -> &'static str {
    if w > 1 {
        panic!("w bit: {} is invalid", w);
    }

    if w == 0 {
        return match reg_byte {
            0x0 => "AL",
            0x1 => "CL",
            0x2 => "DL",
            0x3 => "BL",
            0x4 => "AH",
            0x5 => "CH",
            0x6 => "DH",
            0x7 => "BH",
            _ => panic!("Invalid register byte: {reg_byte}"),
        }
    }
    else {
        return match reg_byte {
            0x0 => "AX",
            0x1 => "CX",
            0x2 => "DX",
            0x3 => "BX",
            0x4 => "SP",
            0x5 => "BP",
            0x6 => "SI",
            0x7 => "DI",
            _ => panic!("Invalid register byte: {reg_byte}"),
        }
    }
}

pub fn decode(u16_byte: &Vec<u8>) -> String {
    let instruction = get_instruction(&u16_byte[0]);

    return match instruction {
        Instruction::MOV => decode_mov(&u16_byte),
    }
}