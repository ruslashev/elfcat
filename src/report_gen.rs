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
        wnonl!($dst, $indent_level, $( $arg )* );
        writeln!($dst, "").unwrap();
    }
}

macro_rules! wnonl {
    ($dst:expr, $indent_level:expr, $($arg:tt)*) => {
        write!($dst, "{}", INDENT.repeat($indent_level)).unwrap();
        write!($dst, $( $arg )* ).unwrap();
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
    w!(o, 1, "<style>");
    wnonl!(o, 0, "{}", stylesheet);
    w!(o, 1, "</style>");
    w!(o, 0, "</head>");
}

fn generate_header(o: &mut String, elf: &ParsedElf) {
    w!(o, 1, "<table>");

    for (id, desc, value) in elf.information.iter() {
        w!(o, 2, "<tr id='desc_{}'>", id);

        w!(o, 3, "<td>{}:</td>", desc);
        w!(o, 3, "<td>{}</td>", value);

        w!(o, 2, "</tr>");
    }

    w!(o, 1, "</table>");
}

fn add_hover_scripts(o: &mut String) {
    let template: &str = include_str!("hover-template.js");
    let ids = [
        "class",
        "data",
        "abi",
        "abi_ver",
        "e_type",
        "e_machine",
        "e_entry",
        "ph",
        "sh",
    ];
    let color = "#ee9";

    w!(o, 1, "<script type='text/javascript'>");

    for id in ids.iter() {
        let desc = format!("desc_{}", id);

        // this is really ugly
        let code = template
            .replace("primary_id", id)
            .as_str()
            .replace("secondary_id", desc.as_str())
            .replace("color", color);
        let indented: String = code.lines().map(|x| indent(2, x) + "\n").collect();

        wnonl!(o, 0, "{}", indented);
    }

    w!(o, 1, "</script>");
}

fn format_magic(byte: u8) -> String {
    if byte.is_ascii_graphic() {
        format!("&nbsp;{}", char::from(byte))
    } else {
        format!("{:02x}", byte)
    }
}

fn generate_body(o: &mut String, elf: &ParsedElf) {
    w!(o, 0, "<body>");

    generate_header(o, elf);

    w!(o, 1, "<div class='box'>");

    for (i, b) in elf.contents.iter().take(192).enumerate() {
        for range_type in elf.ranges.lookup_range_inits(i) {
            wnonl!(o, 0, "<span id='{}'>", range_type.span_id());
        }

        if i < 4 {
            wnonl!(o, 0, "{}", format_magic(*b));
        } else {
            wnonl!(o, 0, "{:02x}", b);
        }

        for _ in 0..elf.ranges.lookup_range_ends(i) {
            wnonl!(o, 0, "</span>");
        }

        wnonl!(o, 0, "{}", if (i + 1) % 16 == 0 { "</br>\n" } else { " " });
    }

    w!(o, 1, "</div>");

    add_hover_scripts(o);

    w!(o, 0, "</body>");
}

pub fn generate_report(elf: &ParsedElf) -> String {
    let mut output = String::new();

    generate_head(&mut output, elf);
    generate_body(&mut output, elf);

    output
}
