use std::vec::IntoIter;

mod decoder_types { include!("decoder_types.rs"); }
use decoder_types::*;

mod mov_strategy { include!("mov.rs"); }
use mov_strategy::*;

pub fn decode(file_iter: &mut IntoIter<u8>) -> Vec<String> {
    let mut content = Vec::new();

    let mut val = file_iter.next();
    while val.is_some() {
        let curr_byte = val.unwrap();
        let instruction = get_instruction(&curr_byte);
        println!("{}", instruction);

        let line = match instruction {
            Instruction::MovImmediateToReg => decode_mov_immediate_to_reg(curr_byte, file_iter),
            Instruction::MovImmediateToRegMem => decode_mov_immediate_to_reg_mem(curr_byte, file_iter),
            Instruction::MovRegister => decode_mov_register(curr_byte, file_iter),
            Instruction::MovRegMemToSegReg => decode_mov_regmem_to_segreg(curr_byte, file_iter),
            Instruction::MovSegRegToRegMem => decode_mov_segreg_to_regmem(curr_byte, file_iter),
            Instruction::MovMemToAcc => decode_mem_to_acc(curr_byte, file_iter),
            Instruction::MovAccToMem => decode_acc_to_mem(curr_byte, file_iter),
        };

        content.push(line.to_lowercase());
        println!("{:?}", content);
        val = file_iter.next();
    }

    return content;
}

fn get_instruction(byte: &u8 ) -> Instruction {
    if byte == &142 {
        return Instruction::MovRegMemToSegReg;
    }
    if byte == &140 {
        return Instruction::MovSegRegToRegMem;
    }

    if (byte >> 4) == 0xB {
        return Instruction::MovImmediateToReg;
    }

    if (byte >> 2) == 0x22 {
        return Instruction::MovRegister;
    }

    return match byte >> 1 {
        0x99 => Instruction::MovImmediateToRegMem,
        0x80 => Instruction::MovMemToAcc,
        0x81 => Instruction::MovAccToMem,
        _ => panic!("this shouldn't be hit"),
    }
}