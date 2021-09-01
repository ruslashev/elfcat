mod elf;
mod report_gen;
mod utils;

use elf::parser::ParsedElf;
use utils::PrintableError;

fn main() {
    let filename = parse_arguments();
    let contents = std::fs::read(&filename).unwrap_or_exit(&format!("read file \"{}\"", filename));
    let elf = ParsedElf::from_bytes(&filename, &contents).unwrap_or_exit("parse ELF");
    let report_filename = utils::construct_filename(&filename).unwrap_or_exit("construct filename");
    let report = report_gen::generate_report(&elf).unwrap_or_exit("generate report");

    std::fs::write(report_filename, report).unwrap_or_exit("write report");
}

fn parse_arguments() -> String {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        usage(1);
    }

    if args[1] == "-h" || args[1] == "--help" {
        usage(0);
    }

    if args[1] == "-v" || args[1] == "--version" {
        println!("elfcat {}", env!("CARGO_PKG_VERSION"));
        std::process::exit(0);
    }

    args[1].clone()
}

fn usage(ret: i32) {
    println!("Usage: elfcat <filename>");
    println!("Writes <filename>.html to CWD.");

    std::process::exit(ret);
}
