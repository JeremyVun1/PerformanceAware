mod sim86_simulator;
mod sim86_simulator_tests;
mod sim86_result;
mod sim86_wrapper;
mod util;


use std::env;

use sim86_simulator::Simulator8086;
use sim86_result::SimulationResult;
use util::*;

use crate::sim86_simulator::IP_REG_IDX;

fn main() {
    let args: Vec<String> = env::args().collect();
    let listing = &args[1];
    println!("listing: {listing}");

    let result = run(listing);
    //println!("{}", result);

    let out_file = format!("data/{listing}.simulation.result");
    write_to_file(out_file, format!("{result}"));
}

fn run(listing: &str) -> SimulationResult {
    let assembly_file = format!("data/{listing}.asm");
    let binary_file = format!("data/{listing}");

    assemble(&assembly_file);
    let source = read_from_file(&binary_file);

    let mut sim86 = Simulator8086::new();

    // load instructions into memory
    let start = (source.len() + 2) as u16;
    sim86.write_8_to_memory(2, &source);
    sim86.write_16(IP_REG_IDX, &(2 as u16));
    sim86.write_8_to_memory(0, &start.to_be_bytes());
    
    let mut transitions = Vec::new();
    let inst_end: u16 = (source.len() + 2) as u16;
    while sim86.read_16(IP_REG_IDX) < inst_end {
        let instruction = sim86.decode_8086_instruction();
        
        let transition = sim86.execute_instruction(&instruction);
        transitions.push(transition);
    }

    let bytes = &sim86.memory[start as usize..];
    write_bytes_to_file(format!("data/{listing}.data"), bytes);
    
    SimulationResult {
        listing: listing.to_string(),
        sim86,
        transitions,
    }
}