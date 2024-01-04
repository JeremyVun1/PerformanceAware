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

    let mut offset = 0;
    while source.len() > offset {
        let instruction = decode_8086_instruction(&source[offset..])
            .expect("Error decoding instruction");
        
        let transition = execute_instruction(&mut sim86, &instruction);
        offset += instruction.Size as usize;

        transitions.push(transition);
    }
    
    SimulationResult {
        listing: listing.to_string(),
        sim86,
        transitions,
    }
}