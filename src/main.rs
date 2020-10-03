use std::path::Path;

struct ParsedElf {
    filename: String,
    contents: Vec<u8>,
}

fn main() {
    let filename = parse_arguments();
    let contents = std::fs::read(&filename).unwrap();
    let elf = ParsedElf::from_bytes(&filename, contents);
    let report = generate_report(&elf);
}

fn parse_arguments() -> String {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        usage();
        std::process::exit(1);
    }

    args[1].clone()
}

fn usage() {
    println!("Usage: elfcat <filename>")
}

impl ParsedElf {
    pub fn from_bytes(filename: &String, buf: Vec<u8>) -> ParsedElf {
        // check header

        ParsedElf {
            filename: filename.clone(),
            contents: buf,
        }
    }
}

fn generate_report(elf: &ParsedElf) -> String {
    let mut output: String = String::from("");

    output.push_str("<!doctype html>\n");

    output
}
