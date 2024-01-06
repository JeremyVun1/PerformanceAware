#![cfg(test)]

use std::fs;

use crate::{sim86_simulator::Simulator8086, util::assemble};
use super::run;

// #[test]
// fn simulate_43_test() {
//     simulate_test("listing_0043_immediate_movs");
// }

// #[test]
// fn simulate_44_test() {
//     simulate_test("listing_0044_register_movs");
// }

// #[test]
// fn simulate_45_test() {
//     simulate_test("listing_0045_challenge_register_movs");
// }

// #[test]
// fn simulate_46_test() {
//     simulate_test("listing_0046_add_sub_cmp");
// }

// fn simulate_test(listing: &str) {
//     // Arrange
//     let expected_file = format!("data/{listing}.txt");
//     let expected: String = fs::read_to_string(expected_file).unwrap()
//         .chars()
//         .filter(|c| !c.is_whitespace())
//         .collect();
    
//     // Act
//     let result = run(listing);

//     // Assert
//     let actual: String = format!("{}", result)
//         .chars()
//         .filter(|c| !c.is_whitespace())
//         .collect();

//     println!("ACTUAL");
//     println!("{}", actual);
//     println!("EXPECTED");
//     println!("{}", expected);

//     assert_eq!(actual, expected);
// }