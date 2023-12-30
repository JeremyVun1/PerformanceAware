use std::vec::IntoIter;

mod util { include!("util.rs"); }
use util::*;
use util::decoder_types::*;

pub fn decode_mov_immediate_to_reg(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    let w = get_first_bit(&(curr_byte >> 3));
    let reg = decode_register(curr_byte & 0x7, w);

    let data = get_data(file_iter, w);

    return format!("mov {}, {}", reg, data);
}

pub fn decode_mov_immediate_to_reg_mem(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    let w = get_first_bit(&curr_byte);

    let byte_2 = file_iter.next().unwrap();
    let mode = get_mode(&byte_2);

    let (a, b) = match mode {
        Mode::MemoryMode0 => decode_mem_mode_0(byte_2, file_iter, w),
        Mode::MemoryMode8 => decode_mem_mode_8(byte_2, file_iter, w),
        Mode::MemoryMode16 => decode_mem_mode_16(byte_2, file_iter, w),
        Mode::RegisterMode => decode_reg_mode(byte_2, file_iter, w)
    };

    let word = match mode {
        Mode::MemoryMode16 => "word",
        Mode::MemoryMode8 => "byte",
        Mode::MemoryMode0 => "byte",
        _ => ""
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
    let w = get_first_bit(&curr_byte);
    let d = get_first_bit(&(curr_byte >> 1));

    let byte_2 = file_iter.next().unwrap();
    let mode = get_mode(&byte_2);

    let (a, b) = match mode {
        Mode::MemoryMode0 => decode_mem_mode_0(byte_2, file_iter, w),
        Mode::MemoryMode8 => decode_mem_mode_8(byte_2, file_iter, w),
        Mode::MemoryMode16 => decode_mem_mode_16(byte_2, file_iter, w),
        Mode::RegisterMode => decode_reg_mode(byte_2, file_iter, w)
    };

    if d {
        return format!("mov {}, {}", a, b);
    } else {
        return format!("mov {}, {}", b, a);
    }
}
