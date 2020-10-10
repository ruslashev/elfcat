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
        .map(|x| indent(3, x) + "\n")
        .collect();

    w!(o, 1, "<head>");
    w!(o, 2, "<meta charset='utf-8'>");
    w!(o, 2, "<title>{}</title>", basename(&elf.filename));
    w!(o, 2, "<style>");
    wnonl!(o, 0, "{}", stylesheet);
    w!(o, 2, "</style>");
    w!(o, 1, "</head>");
}

fn generate_info_table(o: &mut String, elf: &ParsedElf) {
    w!(o, 5, "<table>");

    for (id, desc, value) in elf.information.iter() {
        w!(o, 6, "<tr id='info_{}'>", id);

        w!(o, 7, "<td>{}:</td>", desc);
        w!(o, 7, "<td>{}</td>", value);

        w!(o, 6, "</tr>");
    }

    w!(o, 5, "</table>");
}

fn generate_header(o: &mut String, elf: &ParsedElf) {
    w!(o, 2, "<table>");
    w!(o, 3, "<tr>");

    w!(o, 4, "<td>");
    generate_info_table(o, elf);
    w!(o, 4, "</td>");

    w!(o, 4, "<td id='desc'></td>");

    w!(o, 3, "</tr>");
    w!(o, 2, "</table>");
}

fn add_highlight_script(o: &mut String) {
    let template: &str = include_str!("highlight.js");
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

    w!(o, 2, "<script type='text/javascript'>");

    for id in ids.iter() {
        let info = format!("info_{}", id);

        // this is really ugly
        let code = template
            .replace("primary_id", id)
            .as_str()
            .replace("secondary_id", info.as_str())
            .replace("color", color);
        let indented: String = code.lines().map(|x| indent(3, x) + "\n").collect();

        wnonl!(o, 0, "{}", indented);
    }

    w!(o, 2, "</script>");
}

fn format_magic(byte: u8) -> String {
    if byte.is_ascii_graphic() {
        format!("&nbsp;{}", char::from(byte))
    } else {
        format!("{:02x}", byte)
    }
}

fn generate_body(o: &mut String, elf: &ParsedElf) {
    w!(o, 1, "<body>");

    generate_header(o, elf);

    w!(o, 2, "<div class='box'>");

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

    w!(o, 2, "</div>");

    add_highlight_script(o);

    w!(o, 1, "</body>");
}

pub fn generate_report(elf: &ParsedElf) -> String {
    let mut output = String::new();

    w!(&mut output, 0, "<!doctype html>");
    w!(&mut output, 0, "<html>");

    generate_head(&mut output, elf);
    generate_body(&mut output, elf);

    w!(&mut output, 0, "</html>");

    output
}
