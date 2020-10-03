use std::fmt::Write;
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

const INDENT: &str = "  ";

fn indent(level: usize, line: &str) -> String {
    if line == "" {
        String::new()
    } else {
        INDENT.repeat(level) + line
    }
}

macro_rules! w {
    ($dst:expr, $indent_level:expr, $($arg:tt)*) => {
        write!($dst, "{}", INDENT.repeat($indent_level)).unwrap();
        writeln!($dst, $( $arg )* ).unwrap();
    }
}

fn generate_head(o: &mut String, elf: &ParsedElf) {
    let stylesheet: String = include_str!("style.css")
        .lines()
        .map(|x| indent(2, x) + "\n")
        .collect();

    w!(o, 0, "<!doctype html>");
    w!(o, 0, "<head>");
    w!(o, 1, "<meta charset=\"utf-8\">");
    w!(o, 1, "<title>{}</title>", basename(&elf.filename));
    w!(o, 1, "<style>\n{}</style>", stylesheet);
    w!(o, 0, "</head>");
}

fn generate_body(o: &mut String, elf: &ParsedElf) {
    let mut file = String::new();

    for b in elf.contents.iter() {
        write!(&mut file, "{:02x} ", b).unwrap();
    }

    w!(o, 0, "<body>");
    w!(o, 1, "{}", file);
    w!(o, 0, "</body>");
}

fn generate_report(elf: &ParsedElf) -> String {
    let mut output = String::new();

    generate_head(&mut output, elf);
    generate_body(&mut output, elf);

    output
}
