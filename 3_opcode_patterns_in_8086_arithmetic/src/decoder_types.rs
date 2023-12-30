use std::fmt;

pub enum Instruction {
    MovImmediateToReg,
    MovImmediateToRegMem,
    MovRegister,
    MovRegMemToSegReg,
    MovSegRegToRegMem,
    MovMemToAcc,
    MovAccToMem,

    AddRegister,
    AddImmediateToAcc,
    SubRegister,
    SubImmediateFromAcc,
    AddOrSubImmediateFromToReg,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Instruction::MovImmediateToReg => write!(f, "MovImmediateToReg"),
            Instruction::MovImmediateToRegMem => write!(f, "MovImmediateToRegMem"),
            Instruction::MovRegister => write!(f, "MovRegister"),
            Instruction::MovRegMemToSegReg => write!(f, "RegMovRegMemToSegRegisterMode"),
            Instruction::MovSegRegToRegMem => write!(f, "MovSegRegToRegMem"),
            Instruction::MovMemToAcc => write!(f, "MovMemToAcc"),
            Instruction::MovAccToMem => write!(f, "MovAccToMem"),
            Instruction::AddImmediateToAcc => write!(f, "AddImmediateToAcc"),
            Instruction::AddRegister => write!(f, "AddRegister"),
            Instruction::SubRegister => write!(f, "SubRegister"),
            Instruction::AddOrSubImmediateFromToReg => write!(f, "AddOrSubImmediateFromToReg"),
            Instruction::SubImmediateFromAcc => write!(f, "SubImmediateFromAcc"),
        }
     }
}

pub enum Mode {
    MemoryMode0,
    MemoryMode8,
    MemoryMode16,
    RegisterMode
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Mode::MemoryMode0 => write!(f, "MemoryMode0"),
            Mode::MemoryMode8 => write!(f, "MemoryMode8"),
            Mode::MemoryMode16 => write!(f, "MemoryMode16"),
            Mode::RegisterMode => write!(f, "RegisterMode"),
        }
     }
}