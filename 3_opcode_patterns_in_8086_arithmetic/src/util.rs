use std::vec::IntoIter;
use std::process::Command;

pub mod decoder_types { include!("decoder_types.rs"); }
use decoder_types::*;

pub fn get_first_bit(byte: &u8) -> bool {
    return (byte & 0x1) != 0;
}

pub fn get_mode(byte: &u8) -> Mode {
    //println!("{:b}", byte);
    let mode = (byte & 0xC0) >> 6;

    return match mode {
        0x0 => Mode::MemoryMode0,
        0x1 => Mode::MemoryMode8,
        0x2 => Mode::MemoryMode16,
        0x3 => Mode::RegisterMode,
        _ => panic!("Unsupported mode"),
    }
}

pub fn decode_register(reg_byte: u8, w: bool) -> String {
    if w {
        let result = match reg_byte {
            0x0 => "AX",
            0x1 => "CX",
            0x2 => "DX",
            0x3 => "BX",
            0x4 => "SP",
            0x5 => "BP",
            0x6 => "SI",
            0x7 => "DI",
            _ => panic!("Invalid register byte: {}", reg_byte),
        };
        return result.to_string();
    }
    else {
        let result = match reg_byte {
            0x0 => "AL",
            0x1 => "CL",
            0x2 => "DL",
            0x3 => "BL",
            0x4 => "AH",
            0x5 => "CH",
            0x6 => "DH",
            0x7 => "BH",
            _ => panic!("Invalid register byte: {}", reg_byte),
        };
        return result.to_string();
    }
}

pub fn decode_reg_mode(reg_byte: u8, file_iter: &IntoIter<u8>, w: bool) -> (String, String) {
    println!("reg mode");
    let rm = reg_byte & 0x7;
    let reg = (reg_byte & 0x38) >> 3;

    let reg_a = decode_register(reg, w);
    let reg_b = decode_register(rm, w);

    return (reg_a, reg_b);
}

pub fn decode_mem_mode_0(reg_byte: u8, file_iter: &mut IntoIter<u8>, w: bool) -> (String, String) {
    let rm = reg_byte & 0x7;
    let reg = (reg_byte & 0x38) >> 3;

    let reg_a = decode_register(reg, w);
    
    // Special case for direct address
    let reg_b: String;
    if rm == 0x6 {
        let low_disp = file_iter.next().unwrap();
        let high_disp = file_iter.next().unwrap();
        let displacement: i16 = (((high_disp as u16) << 8) | low_disp as u16) as i16;
        reg_b = format!("[{}]", displacement);
    }
    else {
        reg_b = format!("[{}]", decode_rm(rm));
    }

    return (reg_a, reg_b);
}

pub fn decode_mem_mode_8(reg_byte: u8, file_iter: &mut IntoIter<u8>, w: bool) -> (String, String) {
    //println!("mem mode 8");
    let rm = reg_byte & 0x7;
    let reg = (reg_byte & 0x38) >> 3;

    let low_disp = file_iter.next().unwrap();

    let reg_a = decode_register(reg, w);
    let reg_b = format!("[{} {:+ }]", decode_rm(rm), low_disp as i8);

    return (reg_a, reg_b);
}

pub fn decode_mem_mode_16(reg_byte: u8, file_iter: &mut IntoIter<u8>, w: bool) -> (String, String) {
    //println!("mem_mode_16");
    let rm = reg_byte & 0x7;
    let reg = (reg_byte & 0x38) >> 3;

    let low_disp = file_iter.next().unwrap();
    let high_disp = file_iter.next().unwrap();
    let displacement: i16 = (((high_disp as u16) << 8) | low_disp as u16) as i16;

    let reg_a = decode_register(reg, w);
    let reg_b = format!("[{} {:+ }]", decode_rm(rm), displacement);

    return (reg_a, reg_b);
}

pub fn decode_rm(rm: u8) -> String {
    let result = match rm {
        0x0 => "BX + SI",
        0x1 => "BX + DI",
        0x2 => "BP + SI",
        0x3 => "BP + DI",
        0x4 => "SI",
        0x5 => "DI",
        0x6 => "BP",
        0x7 => "BX",
        _ => panic!("Invalid register byte: {}", rm),
    };
    return result.to_string();
}

pub fn get_signed_data(file_iter: &mut IntoIter<u8>, w: bool) -> i16 {    
    if w {
        let byte_2 = file_iter.next().unwrap();
        let byte_3 = file_iter.next().unwrap();
        return (((byte_3 as u16) << 8) | byte_2 as u16) as i16;
    } else {
        let byte_2 = file_iter.next().unwrap() as i8;
        return byte_2 as i16;
    };
}

pub fn get_data(file_iter: &mut IntoIter<u8>, w: bool) -> u16 {
    if w {
        let byte_2 = file_iter.next().unwrap();
        let byte_3 = file_iter.next().unwrap();        
        return ((byte_3 as u16) << 8) | byte_2 as u16;
    } else {
        return file_iter.next().unwrap() as u16;
    };
}

pub fn assemble(filename: &str) {
    Command::new("nasm")
        .arg(filename)
        .spawn()
        .unwrap()
        .wait()
        .expect("Failed to execute nasm");
}