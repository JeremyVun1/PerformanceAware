#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::mem::MaybeUninit;
use std::ffi::CStr;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn get_version() -> u32 {
    unsafe { Sim86_GetVersion() }
}

pub fn decode_8086_instruction(source: &[u8]) -> Option<instruction> {
    let mut buffer = MaybeUninit::uninit();

    let decoded_instruction = unsafe {
        Sim86_Decode8086Instruction(
            source.len() as u32,
            source.as_ptr() as *mut u8,
            buffer.as_mut_ptr(),
        );

        buffer.assume_init()
    };

    if decoded_instruction.Op != operation_type_Op_None {
        Some(decoded_instruction)
    } else {
        None
    }
}

pub fn get_mnemonic_from_operation_type(op: operation_type) -> &'static str {
    unsafe {
        let x = Sim86_MnemonicFromOperationType(op);
        return CStr::from_ptr(x)
            .to_str()
            .expect("error converting operation to string");
    }
}

pub fn get_register_name_from_operand(op: &mut instruction_operand) -> &str {
    unsafe {
        let x = Sim86_RegisterNameFromOperand(&mut op.__bindgen_anon_1.Register as *mut register_access);
        
        return CStr::from_ptr(x)
        .to_str()
        .expect("error converting operand to string");
    };
}