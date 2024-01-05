use std::num::Wrapping;

use crate::sim86_wrapper::{
    instruction, instruction_operand, get_register_name_from_operand,
    operand_type_Operand_Register, operand_type_Operand_Immediate, operand_type_Operand_Memory
};

use super::{
    Simulator8086, transition::Transition,
    read_from_reg, read_full_reg,
    write_to_reg_and_set_flags, FLAG_REG_IDX, IP_REG_IDX
};

const OP_CODE: &'static str = "sub";

pub fn execute_sub(sim86: &mut Simulator8086, instruction: &instruction) -> Transition {
    let mut op_one = instruction.Operands[0];
    let mut op_two = instruction.Operands[1];

    // Sub from reg
    return match op_one.Type {
        operand_type_Operand_Register => match op_two.Type {
            operand_type_Operand_Register => reg_to_reg(sim86, &mut op_one, &mut op_two),
            operand_type_Operand_Immediate => imm_to_reg(sim86, &mut op_one, &mut op_two),
            operand_type_Operand_Memory => mem_to_reg(sim86, &mut op_one, &mut op_two),
            _ => panic!("op_two unsuppored operand"),
        },
        operand_type_Operand_Memory => match op_two.Type {
            operand_type_Operand_Register => reg_to_mem(sim86, &mut op_one, &mut op_two),
            operand_type_Operand_Memory => mem_to_mem(sim86, &mut op_one, &mut op_two),
            _ => panic!("op_two unsuppored operand"),
        },
        _ => panic!("op_one unsupported operand"),
    }
}

fn reg_to_reg(sim86: &mut Simulator8086, op_one: &mut instruction_operand, op_two: &mut instruction_operand) -> Transition {
    let value_two = Wrapping(read_from_reg(sim86, op_two));
    let value_before = Wrapping(read_full_reg(sim86, op_one));
    let new_value = value_before - value_two;

    write_to_reg_and_set_flags(sim86, op_one, new_value.0);

    let value_after = read_full_reg(sim86, op_one);

    Transition {
        op: OP_CODE,
        src: get_register_name_from_operand(op_two).to_string(),
        dst: get_register_name_from_operand(op_one).to_string(),
        before: value_before.0 as u16,
        after: value_after,
        flags: sim86.read_16(FLAG_REG_IDX),
        ip_before: sim86.read_16(IP_REG_IDX),
        ip_after: sim86.read_16(IP_REG_IDX),
    }
}

fn imm_to_reg(sim86: &mut Simulator8086, op_one: &mut instruction_operand, op_two: &mut instruction_operand) -> Transition {
    let src_value = Wrapping(unsafe { op_two.__bindgen_anon_1.Immediate.Value as u16 });
    let value_before = Wrapping(read_full_reg(sim86, op_one));
    let new_value = value_before - src_value;

    write_to_reg_and_set_flags(sim86, op_one, new_value.0);

    let value_after = read_full_reg(sim86, op_one);

    Transition {
        op: OP_CODE,
        src: src_value.to_string(),
        dst: get_register_name_from_operand(op_one).to_string(),
        before: value_before.0,
        after: value_after,
        flags: sim86.read_16(FLAG_REG_IDX),
        ip_before: sim86.read_16(IP_REG_IDX),
        ip_after: sim86.read_16(IP_REG_IDX),
    }
}

fn mem_to_reg(sim86: &mut Simulator8086, op_one: &mut instruction_operand, op_two: &mut instruction_operand) -> Transition {
    println!("mem to reg");

    Transition {
        op: OP_CODE,
        src: String::from(""),
        dst: get_register_name_from_operand(op_one).to_string(),
        before: 0,
        after: 0,
        flags: sim86.read_16(FLAG_REG_IDX),
        ip_before: sim86.read_16(IP_REG_IDX),
        ip_after: sim86.read_16(IP_REG_IDX),
    }
}

fn reg_to_mem(sim86: &mut Simulator8086, op_one: &mut instruction_operand, op_two: &mut instruction_operand) -> Transition {
    println!("reg to mem");

    Transition {
        op: OP_CODE,
        src: String::from(""),
        dst: "dst".to_string(),
        before: 0,
        after: 0,
        flags: sim86.read_16(FLAG_REG_IDX),
        ip_before: sim86.read_16(IP_REG_IDX),
        ip_after: sim86.read_16(IP_REG_IDX),
    }
}

fn mem_to_mem(sim86: &mut Simulator8086, op_one: &mut instruction_operand, op_two: &mut instruction_operand) -> Transition {
    println!("mem to mem");

    Transition {
        op: OP_CODE,
        src: String::from(""),
        dst: "dst".to_string(),
        before: 0,
        after: 0,
        flags: sim86.read_16(FLAG_REG_IDX),
        ip_before: sim86.read_16(IP_REG_IDX),
        ip_after: sim86.read_16(IP_REG_IDX),
    }
}