use std::vec::IntoIter;
use crate::decoder::Mode;

pub fn decode_mov(curr_byte: u8, file_iter: &mut IntoIter<u8>) -> String {
    let w = get_w_bit(&curr_byte);
    let d = get_d_bit(&curr_byte);

    let next_byte = file_iter.next().unwrap();

    let mode = get_mode(&next_byte);

    let (dst, src) = match mode {
        // Mode::MemoryMode0 => decode_mov_mem_mode_0(curr_byte, file_iter, w, d),
        // Mode::MemoryMode8 => decode_mov_mem_mode_8(curr_byte, file_iter, w, d),
        Mode::MemoryMode16 => decode_mov_mem_mode_16(curr_byte, file_iter, w, d),
        Mode::RegisterMode => decode_mov_reg_mode(next_byte, &file_iter, w, d),
        _ => panic!("unsupported mode"),
    };

    return format!("mov {}, {}", dst, src).to_lowercase();
}

fn decode_mov_reg_mode(byte: u8, file_iter: &IntoIter<u8>, w: bool, d: bool) -> (&str, &str) {
    let reg_a = decode_register((byte & 0x38) >> 3, w);
    let reg_b = decode_register(byte & 0x7, w);

    return if d { (reg_a, reg_b) }
        else { (reg_b, reg_a) };
}

fn decode_register(reg_byte: u8, w: bool) -> &'static str {
    if w {
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
    else {
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
}

fn get_w_bit(byte: &u8) -> bool {
    let w = byte & 0x1;
    if w > 1 {
        panic!("w bit: {} is invalid", w);
    }

    return w != 0;
}

fn get_d_bit(byte: &u8) -> bool {
    let d = byte & 0x2;
    if d > 1 {
        panic!("d bit: {} is invalid", d);
    }

    return d != 0;
}

fn get_mode(byte: &u8) -> Mode {
    let mode = (byte & 0xC0) >> 6;

    return match mode {
        0x0 => Mode::MemoryMode0,
        0x1 => Mode::MemoryMode8,
        0x2 => Mode::MemoryMode16,
        0x3 => Mode::RegisterMode,
        _ => panic!("Unsupported mode"),
    }
}