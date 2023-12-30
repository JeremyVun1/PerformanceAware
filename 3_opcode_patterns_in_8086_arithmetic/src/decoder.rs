use std::vec::IntoIter;

mod decoder_types { include!("decoder_types.rs"); }
use decoder_types::*;
mod mov { include!("mov.rs"); }
use mov::*;
mod ops { include!("ops.rs"); }
use ops::*;

pub fn decode(file_iter: &mut IntoIter<u8>) -> Vec<String> {
    let mut content = Vec::new();

    let mut val = file_iter.next();
    while val.is_some() {
        let curr_byte = val.unwrap();
        let instruction = get_instruction(&curr_byte);
        println!("{}", instruction);

        let line = match instruction {
            // MOV
            Instruction::MovImmediateToReg => decode_mov_immediate_to_reg(curr_byte, file_iter),
            Instruction::MovImmediateToRegMem => decode_mov_immediate_to_reg_mem(curr_byte, file_iter),
            Instruction::MovRegister => decode_mov_register(curr_byte, file_iter),
            Instruction::MovRegMemToSegReg => decode_mov_regmem_to_segreg(curr_byte, file_iter),
            Instruction::MovSegRegToRegMem => decode_mov_segreg_to_regmem(curr_byte, file_iter),
            Instruction::MovMemToAcc => decode_mem_to_acc(curr_byte, file_iter),
            Instruction::MovAccToMem => decode_acc_to_mem(curr_byte, file_iter),

            // ADD
            Instruction::AddRegister => decode_add_register(curr_byte, file_iter),
            Instruction::AddImmediateToAcc => decode_add_immediate_to_acc(curr_byte, file_iter),

            // SUB
            Instruction::SubRegister => decode_sub_register(curr_byte, file_iter),
            Instruction::SubImmediateFromAcc => decode_sub_immediate_from_acc(curr_byte, file_iter),

            // ADD OR SUB
            Instruction::AddOrSubImmediateFromToReg => decode_add_or_sub_immediate_from_to_reg(curr_byte, file_iter),
        };

        content.push(line.to_lowercase());
        println!("{:?}", content);
        val = file_iter.next();
    }

    return content;
}

fn get_instruction(byte: &u8 ) -> Instruction {
    println!("get_instruction byte: {:b}", byte);

    // SUB
    if (byte >> 2) == 0xA {
        return Instruction::SubRegister;
    }
    if (byte >> 1) == 0x16 {
        return Instruction::SubImmediateFromAcc;
    }

    // ADD
    if (byte >> 2) == 0x0 {
        return Instruction::AddRegister;
    }
    if (byte >> 1) == 0x2 {
        return Instruction::AddImmediateToAcc;
    }

    // ADD OR SUB
    if (byte >> 2) == 0x20 {
        return Instruction::AddOrSubImmediateFromToReg;
    }

    // MOV
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
        0x63 => Instruction::MovImmediateToRegMem,
        0x50 => Instruction::MovMemToAcc,
        0x51 => Instruction::MovAccToMem,
        _ => panic!("this shouldn't be hit"),
    }
}