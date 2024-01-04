pub mod transition;
mod mov;
mod add;
mod cmp;
mod sub;

use std::fmt;
use transition::Transition;
use mov::execute_mov;
use add::execute_add;
use cmp::execute_cmp;
use sub::execute_sub;
use crate::sim86_simulator::transition::get_flags;

use super::sim86_wrapper::*;

pub struct Simulator8086 {
    pub registers: [u8; 28],
}

impl Simulator8086 {
    pub fn new() -> Simulator8086 {
        Simulator8086 {
            registers: [0; 28]
        }
    }

    pub fn write_8(&mut self, idx: usize, value: u8) {
        self.registers[idx] = value;
    }

    pub fn write_16(&mut self, idx: usize, value: &u16) {
        self.write_8(idx, (value & 0xFF) as u8);
        self.write_8(idx+1, (value >> 8) as u8);
    }

    pub fn read_8(&mut self, idx: usize) -> u16 {
        return self.registers[idx] as u16;
    }

    pub fn read_16(&mut self, idx: usize) -> u16 {
        let low = self.read_8(idx);
        let high = self.read_8(idx+1) << 8;

        return high | low;
    }
}

impl fmt::Display for Simulator8086 {
    /*
    Final registers:
      ax: 0x0001 (1)
      bx: 0x0002 (2)
      cx: 0x0003 (3)
      dx: 0x0004 (4)
      sp: 0x0005 (5)
      bp: 0x0006 (6)
      si: 0x0007 (7)
      di: 0x0008 (8)
    */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<String> = Vec::new();
        for i in (0..28).step_by(2) {
            let reg_name = match i {
                0 => "ax",
                2 => "bx",
                4 => "cx",
                6 => "dx",
                8 => "sp",
                10 => "bp",
                12 => "si",
                14 => "di",
                16 => "es",
                18 => "cs",
                20 => "ss",
                22 => "ds",
                24 => "ip",
                26 => "flags",
                _ => panic!("no reg name for index {}", i)
            };

            // print the register value
            let value = ((self.registers[i+1] as u16) << 8) | (self.registers[i] as u16);
            if value == 0 {
                continue;
            }

            if i == 26 {
                // flags: SF,ZF
                let flags = get_flags(&value);
                if flags.len() > 0 {
                    let line = format!("flags: {}", flags.join(","));
                    lines.push(line);
                }
            }
            else {
                let line = format!("{}: {:#06x} ({})", reg_name, value, value);
                lines.push(line);
            }
        }

        write!(f, "{}", lines.join("\n\t"))
    }
}

pub fn execute_instruction(sim86: &mut Simulator8086, instruction: &instruction) -> Transition {
    return match instruction.Op {
        operation_type_Op_mov => execute_mov(sim86, instruction),
        operation_type_Op_add => execute_add(sim86, instruction),
        operation_type_Op_sub => execute_sub(sim86, instruction),
        operation_type_Op_cmp => execute_cmp(sim86, instruction),
        _ => panic!("unsupported operation")
    }
}

pub fn read_from_reg(sim86: &mut Simulator8086, reg: &mut instruction_operand) -> u16 {
    let register_type = get_register_type(reg);
    let idx = unsafe { (reg.__bindgen_anon_1.Register.Index -1) * 2 } as usize;

    match register_type {
        RegisterType::RegisterTypeLow => sim86.read_8(idx),
        RegisterType::RegisterTypeHigh => sim86.read_8(idx+1),
        RegisterType::RegisterTypeFull => sim86.read_16(idx),
    }
}

pub fn read_full_reg(sim86: &mut Simulator8086, reg: &mut instruction_operand) -> u16 {
    let idx = unsafe { (reg.__bindgen_anon_1.Register.Index -1) * 2 } as usize;
    sim86.read_16(idx)
}

pub fn write_to_reg(sim86: &mut Simulator8086, reg: &mut instruction_operand, value: u16) {
    let register_type = get_register_type(reg);
    let idx = unsafe { (reg.__bindgen_anon_1.Register.Index - 1) * 2 } as usize;
    //println!("writing {} to {}", value, idx);

    match register_type {
        RegisterType::RegisterTypeLow => sim86.write_8(idx, value as u8),
        RegisterType::RegisterTypeHigh => sim86.write_8(idx+1, value as u8),
        RegisterType::RegisterTypeFull => sim86.write_16(idx, &value),
    }
}

pub fn write_to_reg_and_set_flags(sim86: &mut Simulator8086, reg: &mut instruction_operand, value: u16) {
    let register_type = get_register_type(reg);
    let idx = unsafe { (reg.__bindgen_anon_1.Register.Index - 1) * 2 } as usize;
    //println!("writing {} to {}", value, idx);

    match register_type {
        RegisterType::RegisterTypeLow => sim86.write_8(idx, value as u8),
        RegisterType::RegisterTypeHigh => sim86.write_8(idx+1, value as u8),
        RegisterType::RegisterTypeFull => sim86.write_16(idx, &value),
    }

    set_flags(sim86, register_type, value);
}

pub const SF_BIT: u16 = 0x80;
pub const ZF_BIT: u16 = 0x40;
pub const FLAG_REG_IDX: usize = 26;

fn set_flags(sim86:  &mut Simulator8086, register_type: RegisterType, val: u16) {
    set_sign_flag(sim86, register_type, val);
    set_zero_flag(sim86, val);
}

fn set_sign_flag(sim86: &mut Simulator8086, register_type: RegisterType, val: u16) {
    let mut flags = sim86.read_16(FLAG_REG_IDX);

    if (matches!(register_type, RegisterType::RegisterTypeFull) && val & 0x8000 > 0)
        || (!matches!(register_type, RegisterType::RegisterTypeFull) && val & 0x80 > 0) {
            flags |= SF_BIT;
    }
    else {
        flags &= !SF_BIT;
    }

    sim86.write_16(26, &flags)
}

pub fn set_zero_flag(sim86: &mut Simulator8086, val: u16) {
    let mut flags = sim86.read_16(FLAG_REG_IDX);

    if val == 0 {
        flags |= ZF_BIT;
    }
    else {
        flags &= !ZF_BIT;
    }

    sim86.write_16(26, &flags)
}

fn get_flag(sim86:  &mut Simulator8086, flag_type: FlagType) -> bool {
    let flags = sim86.read_16(FLAG_REG_IDX);

    match flag_type {
        FlagType::ZF => flags & ZF_BIT > 0,
        FlagType::SF => flags & SF_BIT > 0,
    }
}

pub enum FlagType {
    ZF,
    SF
}

fn get_register_type(instruction: &mut instruction_operand) -> RegisterType {
    if instruction.Type != operand_type_Operand_Register {
        panic!("cannot get the register type of a non register operand");
    }

    let reg_name = get_register_name_from_operand(instruction);

    let char = reg_name.chars().nth(1)
        .expect("register mnem should have length of two")
        .to_ascii_lowercase();

    match char {
        'l' => RegisterType::RegisterTypeLow,
        'h' => RegisterType::RegisterTypeHigh,
        _ => RegisterType::RegisterTypeFull,
    }
}

enum RegisterType {
    RegisterTypeLow,
    RegisterTypeHigh,
    RegisterTypeFull
}
