use std::process::Command;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub fn assemble(filename: &str) {
    Command::new("nasm")
        .arg(filename)
        .spawn()
        .unwrap()
        .wait()
        .expect("Failed to execute nasm");
}

pub fn read_from_file(filename: &str) -> Vec<u8> {
    let mut file = File::open(filename)
        .expect("error opening file");

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .expect("error reading file");

    return buffer;
}

pub fn write_bytes_to_file(filename: String, content: &[u8]) {
    println!("writing to file {filename}");
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(filename)
        .unwrap();

    file.write_all(content)
        .expect("error writing to file");
}

pub fn write_to_file(filename: String, content: String) {
    write_bytes_to_file(filename, content.as_bytes());
}