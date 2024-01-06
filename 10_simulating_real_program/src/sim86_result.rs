use std::fmt;

use super::Simulator8086;
use super::sim86_simulator::transition::Transition;

pub struct SimulationResult {
    pub listing: String,
    pub sim86: Simulator8086,
    pub transitions: Vec<Transition>
}

impl fmt::Display for SimulationResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines: Vec<String> = Vec::new();

        lines.push(format!("--- test\\{} execution ---", self.listing));

        for transition in self.transitions.iter() {
            lines.push(format!("{}", transition));
        }

        lines.push(format!("\nFinal registers:\n\t{}", self.sim86));

        write!(f, "{}", lines.join("\n"))
    }
}