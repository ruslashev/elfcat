use crate::elf::parser::ParsedElf;
use std::fmt::Write;
use std::path::Path;

const INDENT: &str = "  ";

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

pub fn construct_filename(filename: &String) -> String {
    stem(basename(filename)).to_string() + ".html"
}

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
    w!(o, 1, "<meta charset='utf-8'>");
    w!(o, 1, "<title>{}</title>", basename(&elf.filename));
    w!(o, 1, "<style>\n{}</style>", stylesheet);
    w!(o, 0, "</head>");
}

fn generate_header(o: &mut String, elf: &ParsedElf) {
    w!(o, 1, "<table>");

    for (desc, value) in elf.information.iter() {
        w!(o, 2, "<tr>");

        w!(o, 3, "<td>{}:</td>", desc);
        w!(o, 3, "<td>{}</td>", value);

        w!(o, 2, "</tr>");
    }

    w!(o, 1, "</table>");
}

fn generate_body(o: &mut String, elf: &ParsedElf) {
    w!(o, 0, "<body>");

    generate_header(o, elf);

    w!(o, 1, "<div class='box'>");

    for (i, b) in elf.contents.iter().take(192).enumerate() {
        for range_type in elf.ranges.lookup_range_inits(i) {
            write!(o, "<span class='{}'>", range_type.span_class()).unwrap();
        }

        write!(o, "{:02x}", b).unwrap();

        for _ in 0..elf.ranges.lookup_range_ends(i) {
            write!(o, "</span>").unwrap();
        }

        write!(o, "{}", if (i + 1) % 16 == 0 { "</br>\n" } else { " " }).unwrap();
    }

    w!(o, 1, "</div>");

    w!(o, 0, "</body>");
}

pub fn generate_report(elf: &ParsedElf) -> String {
    let mut output = String::new();

    generate_head(&mut output, elf);
    generate_body(&mut output, elf);

    output
}
