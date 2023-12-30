use std::vec::IntoIter;
use crate::decoder::Mode;

pub fn decode_mov_immediate_to_reg(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    let w = get_first_bit(&(curr_byte >> 3));
    let reg = decode_register(curr_byte & 0x7, w);

    let data = get_data(file_iter, w);

    return format!("mov {}, {}", reg, data);
}

fn get_data(file_iter: &mut IntoIter<u8>, w: bool) -> u16 {
    println!("{}", w);
    if w {
        let byte_2 = file_iter.next().unwrap();
        println!("byte_2: {}", byte_2);
        let byte_3 = file_iter.next().unwrap();
        println!("byte_3: {}", byte_3);
        return ((byte_3 as u16) << 8) | byte_2 as u16;
    } else {
        let byte_2 = file_iter.next().unwrap() as u16;
        println!("byte_2: {}", byte_2);
        return byte_2 as u16;
    };
}

pub fn decode_mov_immediate_to_reg_mem(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    println!("immediate to reg mem");
    let w = get_first_bit(&curr_byte);

    let byte_2 = file_iter.next().unwrap();
    let mode = get_mode(&byte_2);
    println!("mode: {}", mode);

    let (a, b) = match mode {
        Mode::MemoryMode0 => decode_mov_mem_mode_0(byte_2, file_iter, w),
        Mode::MemoryMode8 => decode_mov_mem_mode_8(byte_2, file_iter, w),
        Mode::MemoryMode16 => decode_mov_mem_mode_16(byte_2, file_iter, w),
        Mode::RegisterMode => decode_mov_reg_mode(byte_2, file_iter, w)
    };

    let word = match mode {
        Mode::MemoryMode16 => "word",
        _ => "byte",
    };
    let data = get_data(file_iter, w);

    return format!("mov {}, {} {}", b, word, data);
}

pub fn decode_mov_regmem_to_segreg(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    return String::new();
}

pub fn decode_mov_segreg_to_regmem(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    return String::new();
}

pub fn decode_mem_to_acc(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    let w = get_first_bit(&curr_byte);
    let reg = if w { "AX" } else { "AL" };

    let addr_lo = file_iter.next().unwrap();
    let addr_high = file_iter.next().unwrap();
    let addr = ((addr_high as u16) << 8) | addr_lo as u16;

    return format!("mov {}, [{}]", reg, addr);
}

pub fn decode_acc_to_mem(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    let w = get_first_bit(&curr_byte);
    let reg = if w { "AX" } else { "AL" };

    let addr_lo = file_iter.next().unwrap();
    let addr_high = file_iter.next().unwrap();
    let addr = ((addr_high as u16) << 8) | addr_lo as u16;

    return format!("mov [{}], {}", addr, reg);
}

pub fn decode_mov_register(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    println!("{:b}", curr_byte);
    let w = get_first_bit(&curr_byte);
    println!("w: {}", w);
    let d = get_first_bit(&(curr_byte >> 1));
    println!("d: {}", d);

    let byte_2 = file_iter.next().unwrap();
    let mode = get_mode(&byte_2);
    println!("mode: {}", mode);

    let (a, b) = match mode {
        Mode::MemoryMode0 => decode_mov_mem_mode_0(byte_2, file_iter, w),
        Mode::MemoryMode8 => decode_mov_mem_mode_8(byte_2, file_iter, w),
        Mode::MemoryMode16 => decode_mov_mem_mode_16(byte_2, file_iter, w),
        Mode::RegisterMode => decode_mov_reg_mode(byte_2, file_iter, w)
    };

    if d {
        return format!("mov {}, {}", a, b);
    } else {
        return format!("mov {}, {}", b, a);
    }
}

fn decode_mov_mem_mode_0(reg_byte: u8, file_iter: &mut IntoIter<u8>, w: bool) -> (String, String) {
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

fn decode_mov_mem_mode_8(reg_byte: u8, file_iter: &mut IntoIter<u8>, w: bool) -> (String, String) {
    println!("mem mode 8");
    let rm = reg_byte & 0x7;
    let reg = (reg_byte & 0x38) >> 3;

    let low_disp = file_iter.next().unwrap();

    let reg_a = decode_register(reg, w);
    let reg_b = format!("[{} {:+ }]", decode_rm(rm), low_disp as i8);

    return (reg_a, reg_b);
}

fn decode_mov_mem_mode_16(reg_byte: u8, file_iter: &mut IntoIter<u8>, w: bool) -> (String, String) {
    println!("mem_mode_16");
    let rm = reg_byte & 0x7;
    let reg = (reg_byte & 0x38) >> 3;

    let low_disp = file_iter.next().unwrap();
    let high_disp = file_iter.next().unwrap();
    let displacement: i16 = (((high_disp as u16) << 8) | low_disp as u16) as i16;

    let reg_a = decode_register(reg, w);
    let reg_b = format!("[{} {:+ }]", decode_rm(rm), displacement);

    return (reg_a, reg_b);
}

fn decode_rm(rm: u8) -> String {
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

fn decode_mov_reg_mode(reg_byte: u8, file_iter: &IntoIter<u8>, w: bool) -> (String, String) {
    let rm = reg_byte & 0x7;
    let reg = (reg_byte & 0x38) >> 3;

    let reg_a = decode_register(reg, w);
    let reg_b = decode_register(rm, w);

    return (reg_a, reg_b);
}

fn decode_register(reg_byte: u8, w: bool) -> String {
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

fn get_first_bit(byte: &u8) -> bool {
    return (byte & 0x1) != 0;
}

fn get_mode(byte: &u8) -> Mode {
    println!("{:b}", byte);
    let mode = (byte & 0xC0) >> 6;

    return match mode {
        0x0 => Mode::MemoryMode0,
        0x1 => Mode::MemoryMode8,
        0x2 => Mode::MemoryMode16,
        0x3 => Mode::RegisterMode,
        _ => panic!("Unsupported mode"),
    }
}