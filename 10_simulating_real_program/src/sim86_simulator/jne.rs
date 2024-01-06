use crate::sim86_wrapper::instruction;

use super::{
    Simulator8086, transition::Transition, FLAG_REG_IDX, FlagType, get_flag, IP_REG_IDX
};

const OP_CODE: &'static str = "jne";

pub fn execute_jne(sim86: &mut Simulator8086, instruction: &instruction) -> Transition {
    let op_one = instruction.Operands[0];
    let jump_val = unsafe { op_one.__bindgen_anon_1.Immediate.Value };
    let curr_ip = sim86.read_16(IP_REG_IDX);

    if get_flag(sim86, FlagType::ZF) {
        return Transition {
            op: OP_CODE,
            src: String::from(""),
            dst: jump_val.to_string(),
            before: curr_ip,
            after: curr_ip,
            flags: sim86.read_16(FLAG_REG_IDX),
            ip_before: sim86.read_16(IP_REG_IDX),
            ip_after: sim86.read_16(IP_REG_IDX),
        };
    }

    let new_ip: i32 = curr_ip as i32 + jump_val;
    if new_ip < 0 { panic!("ip cannot be negative"); }

    sim86.write_16(IP_REG_IDX, &(new_ip as u16));

    Transition {
        op: OP_CODE,
        src: String::from("ip"),
        dst: jump_val.to_string(),
        before: curr_ip as u16,
        after: new_ip as u16,
        flags: sim86.read_16(FLAG_REG_IDX),
        ip_before: curr_ip,
        ip_after: sim86.read_16(IP_REG_IDX),
    }
}