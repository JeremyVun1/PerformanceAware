pub mod transition;
mod mov;

use std::fmt;
use transition::Transition;
use mov::execute_mov;
use super::sim86_wrapper::*;

pub struct Simulator8086 {
    pub registers: [u8; 24],
}

impl Simulator8086 {
    pub fn new() -> Simulator8086 {
        Simulator8086 { registers: [0; 24] }
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
        for i in (0..24).step_by(2) {
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
                _ => panic!("no reg name for index {}", i)
            };

            let value = ((self.registers[i+1] as u16) << 8) | (self.registers[i] as u16);
            if value == 0 {
                continue;
            }

            let line = format!("{}: {:#06x} ({})", reg_name, value, value);
            lines.push(line);
        }

        write!(f, "{}", lines.join("\n\t"))
    }
}

pub fn execute_instruction(sim86: &mut Simulator8086, instruction: &instruction) -> Transition {
    return match instruction.Op {
        operation_type_Op_mov => execute_mov(sim86, instruction),
        //operation_type_Op_add => execute_add(&mut machine. instruction),
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
