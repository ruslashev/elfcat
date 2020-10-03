use std::path::Path;

struct ParsedElf {
    filename: String,
    contents: Vec<u8>,
}

fn main() {
    let filename = parse_arguments();
    let contents = std::fs::read(&filename).unwrap();
    let elf = ParsedElf::from_bytes(&filename, contents);
    let report_filename = construct_filename(&filename);
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
    println!("Usage: elfcat <filename>");
    println!("Writes <filename>.html to CWD.");
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

fn basename(path: &str) -> &str {
    // Wish expect() could use String. This is messy.
    match Path::new(path).file_name() {
        Some(name) => name.to_str().unwrap(),
        None => panic!("basename: failed for path \"{}\"", path),
    }
}

fn stem(path: &str) -> &str {
    match Path::new(path).file_stem() {
        Some(stem) => stem.to_str().unwrap(),
        None => panic!("stem: failed for path \"{}\"", path),
    }
}

fn construct_filename(filename: &String) -> String {
    stem(basename(filename)).to_string() + ".html"
}

fn generate_report(elf: &ParsedElf) -> String {
    let mut output: String = String::from("");

    output.push_str("<!doctype html>\n");

    output
}
