use std::vec::IntoIter;

mod util { include!("util.rs"); }
use util::*;
use util::decoder_types::*;

pub fn decode_add_register(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    return decode_op_register(curr_byte, file_iter, "add");
}

pub fn decode_sub_register(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    return decode_op_register(curr_byte, file_iter, "sub");
}

pub fn decode_op_register(curr_byte: u8, file_iter: &mut IntoIter<u8>, op: &str) -> String {
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
        return format!("{} {}, {}", op, a, b);
    } else {
        return format!("{} {}, {}", op, b, a);
    }
}

pub fn decode_add_immediate_to_acc(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    return decode_op_immediate_acc(curr_byte, file_iter, "add");
}

pub fn decode_sub_immediate_from_acc(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    return decode_op_immediate_acc(curr_byte, file_iter, "sub");
}

pub fn decode_op_immediate_acc(curr_byte: u8, file_iter: &mut IntoIter<u8>, op: &str) -> String {
    let w = get_first_bit(&curr_byte);
    let reg = if w { "AX" } else { "AL" };

    let data = get_signed_data(file_iter, w);

    return format!("{} {}, {}", op, reg, data);
}

pub fn decode_add_or_sub_immediate_from_to_reg(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    let w = get_first_bit(&curr_byte);
    println!("w: {}", w);
    let s = get_first_bit(&(curr_byte >> 1)); // 1 => sign extend
    println!("s: {}", s);

    let byte_2 = file_iter.next().unwrap();
    let mode = get_mode(&byte_2);
    let op_byte = (byte_2 >> 3) & 0x7;
    println!("mode: {}", mode);

    let (a, b) = match mode {
        Mode::MemoryMode0 => decode_mem_mode_0(byte_2, file_iter, w),
        Mode::MemoryMode8 => decode_mem_mode_8(byte_2, file_iter, w),
        Mode::MemoryMode16 => decode_mem_mode_16(byte_2, file_iter, w),
        Mode::RegisterMode => decode_reg_mode(byte_2, file_iter, w)
    };

    let data = get_data(file_iter, !s && w);
    println!("data: {}", data);

    let op: &str;
    if op_byte == 0x0 {
        op = "add";
    } else if op_byte == 0x5 {
        op ="sub";
    }
    else {
        panic!("unsupported opcode: {}", op_byte);
    }

    let word = match b.len() {
        1..=4 => "byte",
        _ => "word",
    };

    return format!("{} {} {}, {}", op, word, b, data);
}