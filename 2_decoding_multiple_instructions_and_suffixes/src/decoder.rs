use std::vec::IntoIter;

mod decoder_types { include!("decoder_types.rs"); }
use decoder_types::*;

mod mov_strategy { include!("mov.rs"); }
use mov_strategy::decode_mov;

pub fn decode(file_iter: &mut IntoIter<u8>) -> Vec<String> {
    let mut content = Vec::new();

    let mut val = file_iter.next();
    while val.is_some() {
        let curr_byte = val.unwrap();
        let instruction = get_instruction(&curr_byte);

        let line = match instruction {
            Instruction::Mov => decode_mov(curr_byte, file_iter)
        };

        content.push(line);
        val = file_iter.next();
    }

    return content;
}

fn get_instruction(byte: &u8 ) -> Instruction {
    let i = byte >> 2;
    return match i {
        0x22 => Instruction::Mov,
        _ => panic!("Unknown instruction: {}", i)
    }
}