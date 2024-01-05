mod sim86_simulator;
mod sim86_simulator_tests;
mod sim86_result;
mod sim86_wrapper;
mod util;


use std::env;

use sim86_simulator::{execute_instruction, Simulator8086};
use sim86_wrapper::decode_8086_instruction;
use sim86_result::SimulationResult;
use util::*;

use crate::sim86_simulator::IP_REG_IDX;

fn main() {
    let args: Vec<String> = env::args().collect();
    let listing = &args[1];
    println!("listing: {listing}");

    let result = run(listing);
    println!("{}", result);

    let out_file = format!("data/{listing}.simulation.result");
    write_to_file(out_file, format!("{result}"));
}

fn run(listing: &str) -> SimulationResult {
    let assembly_file = format!("data/{listing}.asm");
    let binary_file = format!("data/{listing}");

    assemble(&assembly_file);
    let source = read_from_file(&binary_file);

    let mut sim86 = Simulator8086::new();
    let mut transitions = Vec::new();

    while sim86.read_16(IP_REG_IDX) < source.len() as u16 {
        let curr_ip: usize = sim86.read_16(IP_REG_IDX) as usize;

        let instruction = decode_8086_instruction(&source[curr_ip..])
            .expect("Error decoding instruction");

        // update ip after decoding instruction
        sim86.write_16(IP_REG_IDX, &(curr_ip as u16 + instruction.Size as u16));
        
        let transition = execute_instruction(&mut sim86, &instruction);
        transitions.push(transition);
    }
    
    SimulationResult {
        listing: listing.to_string(),
        sim86,
        transitions,
    }
}