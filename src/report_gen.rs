use crate::elf::parser::{ParsedElf, ParsedPhdr, RangeType};
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

trait Indentable {
    fn indent_lines(&self, level: usize) -> String;
}

impl Indentable for str {
    fn indent_lines(&self, level: usize) -> String {
        self.lines().map(|x| indent(level, x) + "\n").collect()
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
    let stylesheet: String = include_str!("style.css").indent_lines(3);

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

fn generate_phdr_info_table(o: &mut String, phdr: &ParsedPhdr, idx: usize) {
    let items = [
        ("Type", &phdr.ptype),
        ("Flags", &phdr.flags),
        ("Offset in file", &format!("{}", phdr.file_offset)),
        ("Size in file", &format!("{:#x}", phdr.file_size)),
        ("Vaddr in memory", &format!("{:#x}", phdr.vaddr)),
        ("Size in memory", &format!("{:#x}", phdr.memsz)),
        ("Alignment", &format!("{:#x}", phdr.alignment)),
    ];

    w!(o, 4, "<table class='info_phdr' id='info_phdr{}'>", idx);

    for (desc, value) in items.iter() {
        w!(o, 5, "<tr>");

        w!(o, 6, "<td>{}:</td>", desc);
        w!(o, 6, "<td>{}</td>", value);

        w!(o, 5, "</tr>");
    }

    w!(o, 4, "</table>");
}

fn generate_phdr_info_tables(o: &mut String, elf: &ParsedElf) {
    for (idx, phdr) in elf.phdrs.iter().enumerate() {
        generate_phdr_info_table(o, &phdr, idx);
    }
}

fn generate_header(o: &mut String, elf: &ParsedElf) {
    w!(o, 2, "<table class='header'>");
    w!(o, 3, "<tr>");

    w!(o, 4, "<td>");
    generate_info_table(o, elf);
    w!(o, 4, "</td>");

    w!(o, 4, "<td id='desc'></td>");

    w!(o, 4, "<td>");
    generate_phdr_info_tables(o, elf);
    w!(o, 4, "</td>");

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
        "e_phoff",
        "e_shoff",
        "e_flags",
        "e_ehsize",
        "e_phentsize",
        "e_phnum",
        "e_shentsize",
        "e_shnum",
        "e_shstrndx",
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
        let indented: String = code.indent_lines(3);

        wnonl!(o, 0, "{}", indented);
    }

    w!(o, 2, "</script>");
}

fn add_description_script(o: &mut String) {
    w!(o, 2, "<script type='text/javascript'>");

    wnonl!(o, 0, "{}", include_str!("description.js").indent_lines(3));

    w!(o, 2, "</script>");
}

fn add_conceal_script(o: &mut String) {
    w!(o, 2, "<script type='text/javascript'>");

    wnonl!(o, 0, "{}", include_str!("conceal.js").indent_lines(3));

    w!(o, 2, "</script>");
}

fn add_scripts(o: &mut String) {
    add_highlight_script(o);

    add_description_script(o);

    add_conceal_script(o);
}

fn format_magic(byte: u8) -> String {
    if byte.is_ascii_graphic() {
        format!("&nbsp;{}", char::from(byte))
    } else {
        format!("{:02x}", byte)
    }
}

fn digit_to_hex(digit: u8) -> char {
    [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
    ][digit as usize]
}

fn append_hex_byte(s: &mut String, byte: u8) {
    if byte < 0x10 {
        s.push('0');

        s.push(digit_to_hex(byte));
    } else {
        let trailing_digit = byte % 16;
        let leading_digit = byte / 16;

        s.push(digit_to_hex(leading_digit));
        s.push(digit_to_hex(trailing_digit));
    }
}

fn generate_file_dump(elf: &ParsedElf) -> String {
    let mut dump = String::new();

    for (i, b) in elf.contents.iter().take(192).enumerate() {
        for range_type in &elf.ranges.data[i] {
            if *range_type != RangeType::End {
                dump += format!("<span {}>", range_type.span_attributes()).as_str();
            }
        }

        if i < 4 {
            dump += format!("{}", format_magic(*b)).as_str();
        } else {
            append_hex_byte(&mut dump, *b);
        }

        for _ in 0..elf.ranges.lookup_range_ends(i) {
            dump += "</span>";
        }

        dump += if (i + 1) % 16 == 0 { "</br>\n" } else { " " };
    }

    dump
}

fn generate_body(o: &mut String, elf: &ParsedElf) {
    w!(o, 1, "<body>");

    generate_header(o, elf);

    w!(o, 2, "<div class='box'>");

    wnonl!(o, 0, "{}", generate_file_dump(elf));

    w!(o, 2, "</div>");

    add_scripts(o);

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
