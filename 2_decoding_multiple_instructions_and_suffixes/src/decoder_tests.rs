#![cfg(test)]

use std::{fs, process::Command, path::Path};

#[test]
fn decode_test() {
    assert_eq!(1, 1);
    
    super::run("sample_b");

    Command::new("nasm")
        .arg("data/sample_b.decoded.asm")
        .spawn()
        .expect("Failed to execute nasm");

    let original = fs::read(Path::new("data/sample_b")).unwrap();
    let decoded = fs::read(Path::new("data/sample_b.decoded")).unwrap();

    assert_eq!(original, decoded);

    assert_eq!(1, 1);
}