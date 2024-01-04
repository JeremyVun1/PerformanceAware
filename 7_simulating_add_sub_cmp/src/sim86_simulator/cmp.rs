use crate::sim86_wrapper::{
    instruction, instruction_operand, get_register_name_from_operand,
    operand_type_Operand_Register, operand_type_Operand_Immediate, operand_type_Operand_Memory
};

use super::{
    Simulator8086, transition::Transition,
    read_from_reg, read_full_reg,
    write_to_reg_and_set_flags, set_flags, get_flag,
    RegisterType, FlagType, FLAG_REG_IDX, set_zero_flag
};

const OP_CODE: &'static str = "cmp";

pub fn execute_cmp(sim86: &mut Simulator8086, instruction: &instruction) -> Transition {
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
    let value_one = read_from_reg(sim86, op_one);
    let value_two = read_from_reg(sim86, op_two);

    let cmp_value = value_one - value_two;
    let before = get_flag(sim86, FlagType::ZF);
    set_zero_flag(sim86, cmp_value);
    let after = get_flag(sim86, FlagType::ZF);

    Transition {
        op: OP_CODE,
        src: get_register_name_from_operand(op_two).to_string(),
        dst: get_register_name_from_operand(op_one).to_string(),
        before: before as u16,
        after: after as u16,
        flags: sim86.read_16(FLAG_REG_IDX),
    }
}

fn imm_to_reg(sim86: &mut Simulator8086, op_one: &mut instruction_operand, op_two: &mut instruction_operand) -> Transition {
    let value_one = read_from_reg(sim86, op_one);
    let value_two = unsafe { op_two.__bindgen_anon_1.Immediate.Value as u16 };

    let cmp_value = value_one - value_two;
    let before = get_flag(sim86, FlagType::ZF);
    set_flags(sim86, RegisterType::RegisterTypeFull, cmp_value);
    let after = get_flag(sim86, FlagType::ZF);

    Transition {
        op: OP_CODE,
        src: get_register_name_from_operand(op_two).to_string(),
        dst: get_register_name_from_operand(op_one).to_string(),
        before: before as u16,
        after: after as u16,
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