mod elf;
mod report_gen;

use elf::parser::ParsedElf;

fn main() {
    let filename = parse_arguments();
    let contents = std::fs::read(&filename).unwrap();
    let elf = ParsedElf::from_bytes(&filename, &contents).unwrap();
    let report_filename = report_gen::construct_filename(&filename);
    let report = report_gen::generate_report(&elf);

    std::fs::write(report_filename, report).expect("failed to write report");
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
