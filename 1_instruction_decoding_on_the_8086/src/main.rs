use std::env;
use std::fs::{self, OpenOptions};
use std::io::{prelude::*, LineWriter};

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
    let in_file = format!("data/{}", filename);
    let out_file = format!("data/{}.decoded.asm", filename);

    let u16_bytes = load_binary_as_u16(&in_file);
    
    println!("Decoding {in_file}");
    let mut content: Vec<String> = Vec::new();
    for u16_byte in u16_bytes {
        content.push(decode(&u16_byte));
    }

    write_lines(&out_file, &in_file, content);
}

fn load_binary_as_u16(filename: &String) -> Vec<Vec<u8>> {
    println!("Loading {filename}");
    let data = fs::read(filename)
        .unwrap();

    return data.chunks(2)
        .map(|x| {
            return vec!(x[0], x[1])
        })
        .collect::<Vec<Vec<u8>>>();
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