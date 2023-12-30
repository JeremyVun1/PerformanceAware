use std::env;
use std::fs::{self, OpenOptions};
use std::io::{prelude::*, LineWriter};

mod util;
use util::assemble;
mod decoder;
use decoder::decode;

mod decoder_tests;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() <= 1 {
        panic!("please specify filename");
    }

    run(&args[1]);
}

fn run(filename: &str) {
    assemble(&format!("data/{}.asm", filename));

    let in_file = format!("data/{}", filename);
    let out_file = format!("data/{}.decoded.asm", filename);

    let mut file_iter = fs::read(&in_file)
        .unwrap()
        .into_iter();

    let content: Vec<String> = decode(&mut file_iter);

    write_lines(&out_file, &in_file, content);
}

fn write_lines(out_file: &String, in_file: &String, content: Vec<String>)
{
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(out_file)
        .unwrap();

    let mut writer = LineWriter::new(file);
    let header = format!("; decoded from {}\n\nbits 16\n\n", in_file);
    writer.write(header.as_bytes())
        .expect("Failed to write line");

    for line in content {
        writer.write(line.as_bytes())
            .expect("Failed to write line");
        writer.write(b"\n")
            .expect("Failed to write line");
    }

    writer.flush()
        .expect("Failed to flush to file");

    println!("Saved decoded output to {out_file}");
}