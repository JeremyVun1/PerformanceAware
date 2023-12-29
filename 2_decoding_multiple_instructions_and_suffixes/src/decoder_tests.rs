#![cfg(test)]

use std::{fs, process::Command, path::Path};

#[test]
fn decode_test_37() {
    decode_test("37");
}

#[test]
fn decode_test_38() {
    decode_test("38");
}

#[test]
fn decode_test_39() {
    decode_test("39");
}

#[test]
fn decode_test_40() {
    decode_test("40");
}

fn decode_test(testcase: &str) {
    assemble(&format!("data/{testcase}.asm"));
        
    super::run(testcase);

    assemble(&format!("data/{testcase}.decoded.asm"));

    let original_filename = format!("data/{testcase}");
    let original_path = Path::new(&original_filename);
    let original = fs::read(original_path).unwrap();

    let decoded_filename = format!("data/{testcase}.decoded");
    let decoded_path = Path::new(&decoded_filename);
    let decoded = fs::read(decoded_path).unwrap();
    
    assert_eq!(original, decoded);
}

fn assemble(filename: &str) {
    Command::new("nasm")
        .arg(filename)
        .spawn()
        .unwrap()
        .wait()
        .expect("Failed to execute nasm");
}