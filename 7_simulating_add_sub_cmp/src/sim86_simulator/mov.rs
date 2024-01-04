use crate::sim86_wrapper::{
    operand_type_Operand_Register,
    operand_type_Operand_Immediate,
    operand_type_Operand_Memory,
    instruction_operand,
    instruction,
    get_register_name_from_operand
};

use super::{Simulator8086, write_to_reg, read_from_reg, read_full_reg, FLAG_REG_IDX};
use super::transition::Transition;

const OP_CODE: &'static str = "mov";

pub fn execute_mov(sim86: &mut Simulator8086, instruction: &instruction) -> Transition {
    let mut op_one = instruction.Operands[0];
    let mut op_two = instruction.Operands[1];

    // Mov to reg
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
    let src_value = read_from_reg(sim86, op_two);

    let value_before = read_full_reg(sim86, op_one);
    write_to_reg(sim86, op_one, src_value);
    let value_after = read_full_reg(sim86, op_one);

    Transition {
        op: OP_CODE,
        src: get_register_name_from_operand(op_two).to_string(),
        dst: get_register_name_from_operand(op_one).to_string(),
        before: value_before,
        after: value_after,
        flags: sim86.read_16(FLAG_REG_IDX),
    }
}

fn imm_to_reg(sim86: &mut Simulator8086, op_one: &mut instruction_operand, op_two: &mut instruction_operand) -> Transition {
    let src_value = unsafe { op_two.__bindgen_anon_1.Immediate.Value as u16 };

    let value_before = read_full_reg(sim86, op_one);
    write_to_reg(sim86, op_one, src_value);
    let value_after = read_full_reg(sim86, op_one);

    Transition {
        op: OP_CODE,
        src: src_value.to_string(),
        dst: get_register_name_from_operand(op_one).to_string(),
        before: value_before,
        after: value_after,
        flags: sim86.read_16(FLAG_REG_IDX),
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
    }
}