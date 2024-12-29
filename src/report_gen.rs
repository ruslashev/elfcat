use crate::elf::defs::*;
use crate::elf::parser::{Note, ParsedElf, ParsedPhdr, ParsedShdr, RangeType};
use crate::utils;
use std::fmt::Write;

const INDENT: &str = "  ";
const DEFAULT_COLUMNS: usize = 16;

fn indent(level: usize, line: &str) -> String {
    if line.is_empty() {
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

macro_rules! hex_dualfmt {
    ($id:expr) => {
        format!("<span class='number' title='{}'>{:#x}</span>", $id, $id)
    };
}

macro_rules! dec_dualfmt {
    ($id:expr) => {
        format!("<span class='number' title='{:#x}'>{}</span>", $id, $id)
    };
}

macro_rules! size_dualfmt {
    ($id:expr) => {
        if ($id >= 1024) {
            format!(
                "<span class='number' title='{:#x}'>{} ({})</span>",
                $id,
                $id,
                utils::human_format_bytes($id as u64)
            )
        } else {
            format!("<span class='number' title='{:#x}'>{}</span>", $id, $id)
        }
    };
}

macro_rules! wrow {
    ($dst:expr, $indent_level:expr, $lhs:expr, $rhs:expr) => {
        wnonl!($dst, $indent_level, "<tr> ");
        wnonl!($dst, 0, "<td>{}:</td> ", $lhs);
        wnonl!($dst, 0, "<td>{}</td> ", $rhs);
        w!($dst, 0, "</tr>");
    };
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

fn generate_head(o: &mut String, elf: &ParsedElf) -> Option<()> {
    let stylesheet: String = include_str!("style.css").indent_lines(3);
    let viewport = "width=900, initial-scale=1";

    w!(o, 1, "<head>");
    w!(o, 2, "<meta charset='utf-8'>");
    w!(o, 2, "<meta name='viewport' content='{}'>", viewport);
    w!(o, 2, "<title>{}</title>", utils::basename(&elf.filename)?);
    w!(o, 2, "<style>");
    wnonl!(o, 0, "{}", stylesheet);
    w!(o, 2, "</style>");
    w!(o, 1, "</head>");

    Some(())
}

fn generate_svg_element(o: &mut String) {
    w!(o, 2, "<svg width='100%' height='100%'>");

    w!(o, 3, "<defs>");

    wnonl!(o, 4, "<marker id='arrowhead' viewBox='0 0 10 10' ");
    wnonl!(o, 0, "refX='10' refY='5' ");
    w!(o, 0, "markerWidth='10' markerHeight='10' orient='auto'>");

    w!(o, 5, "<path d='M 0 0 L 10 5 L 0 10 z' />");

    w!(o, 4, "</marker>");

    w!(o, 3, "</defs>");

    wnonl!(o, 3, "<g id='arrows' stroke='black' ");
    wnonl!(o, 0, "stroke-width='1' marker-end='url(#arrowhead)'>");
    w!(o, 0, "</g>");

    w!(o, 2, "</svg>");
}

fn generate_file_info_table(o: &mut String, elf: &ParsedElf) {
    w!(o, 2, "<table>");

    for (cl, desc, value) in &elf.information {
        wnonl!(o, 3, "<tr class='fileinfo_{}'> ", cl);
        wnonl!(o, 0, "<td>{}:</td> ", desc);
        wnonl!(o, 0, "<td>{}</td> ", value);
        w!(o, 0, "</tr>");
    }

    w!(o, 2, "</table>");
}

fn generate_help(o: &mut String) {
    let legend_items = [
        ("ident", "ELF Identification"),
        ("ehdr", "ELF Header"),
        ("phdr", "Program Header"),
        ("shdr", "Section Header"),
        ("segment", "Segment"),
        ("section", "Section"),
        ("segm_sect_legend", "Segment &amp; Section overlap"),
    ];
    let help_text = "The leftmost column shows offsets within the file. \
                     The middle column is the file dump. It has ELF structs, sections and segments \
                     highlighted. Some fields that reference areas in the file are clickable and \
                     connected with arrows. The rightmost column shows printable ASCII characters \
                     corresponding to the file bytes.";

    w!(o, 4, "<p>{}</p>", help_text);

    w!(o, 4, "<p>Legend</p>");
    w!(o, 4, "<ul>");
    for (class, desc) in &legend_items {
        w!(o, 5, "<li><span class='legend_rect {}'></span>{}</li>", class, desc);
    }
    w!(o, 4, "</ul>");
}

fn generate_right_menu(o: &mut String) {
    let credits = format!("generated with elfcat {}", env!("CARGO_PKG_VERSION"));
    let url = "https://github.com/ruslashev/elfcat";

    w!(o, 3, "<a id='credits' href='{}'>{}</a>", url, credits);

    w!(o, 3, "<button class='textbutton' id='settings_toggle'>Settings</button>");

    w!(o, 3, "<button class='textbutton' id='help_toggle'>Help</button>");

    w!(o, 3, "<div class='right_hidden' id='settings'>");
    w!(o, 4, "<label for='arrow_opacity_range'>Arrow opacity:</label>");
    w!(o, 4, "<input type='range' id='arrow_opacity_range' min='0' max='100' value='100'>");
    w!(o, 3, "</div>");

    w!(o, 3, "<div class='right_hidden' id='help'>");
    generate_help(o);
    w!(o, 3, "</div>");
}

fn generate_phdr_info_tables(o: &mut String, elf: &ParsedElf) {
    for (idx, phdr) in elf.phdrs.iter().enumerate() {
        let items = [
            ("Type", &ptype_to_string(phdr.ptype)),
            ("Flags", &phdr.flags),
            ("Offset in file", &hex_dualfmt!(phdr.file_offset)),
            ("Size in file", &size_dualfmt!(phdr.file_size)),
            ("Vaddr in memory", &hex_dualfmt!(phdr.vaddr)),
            ("Size in memory", &size_dualfmt!(phdr.memsz)),
            ("Alignment", &hex_dualfmt!(phdr.alignment)),
        ];

        w!(o, 5, "<table class='conceal itable' id='info_phdr{}'>", idx);
        w!(o, 5, "<th colspan='2' class='phdr_itable'></th>");

        for (desc, value) in &items {
            wrow!(o, 6, desc, value);
        }

        w!(o, 5, "</table>");
    }
}

fn generate_shdr_info_tables(o: &mut String, elf: &ParsedElf) {
    for (idx, shdr) in elf.shdrs.iter().enumerate() {
        let items = [
            ("Index", format!("{}", idx)),
            ("Name", elf.shnstrtab.get(shdr.name).to_owned()),
            ("Type", shtype_to_string(shdr.shtype)),
            ("Flags", shflags_to_string(shdr.flags)),
            ("Vaddr in memory", hex_dualfmt!(shdr.addr)),
            ("Offset in file", hex_dualfmt!(shdr.file_offset)),
            ("Size in file", size_dualfmt!(shdr.size)),
            ("Linked section", format!("{}", shdr.link)),
            ("Extra info", dec_dualfmt!(shdr.info)),
            ("Alignment", hex_dualfmt!(shdr.addralign)),
            ("Size of entries", size_dualfmt!(shdr.entsize)),
        ];

        w!(o, 5, "<table class='conceal itable' id='info_shdr{}'>", idx);
        w!(o, 5, "<th colspan='2' class='shdr_itable'></th>");

        for (desc, value) in &items {
            wrow!(o, 6, desc, value);
        }

        w!(o, 5, "</table>");
    }
}

fn format_string_byte(byte: u8) -> String {
    if byte.is_ascii_graphic() {
        format!("{}", char::from(byte))
    } else {
        format!("<b>{:02x}</b> ", byte)
    }
}

fn format_string_slice(slice: &[u8]) -> String {
    slice
        .iter()
        .fold(String::new(), |s, b| s + &format_string_byte(*b))
}

fn generate_note_data(o: &mut String, note: &Note) {
    let name = if note.name.is_empty() {
        String::new()
    } else {
        format_string_slice(&note.name[0..note.name.len() - 1])
    };

    wrow!(o, 6, "Name", name);

    wrow!(o, 6, "Type", format!("{:#x}", note.ntype));

    if note.ntype == NT_GNU_BUILD_ID {
        let mut hash = String::new();

        for byte in &note.desc {
            append_hex_byte(&mut hash, *byte);
        }

        wrow!(o, 6, "Build ID", hash);
    } else {
        wrow!(o, 6, "Desc", format_string_slice(&note.desc[..]));
    }
}

fn generate_segment_info_table(o: &mut String, elf: &ParsedElf, phdr: &ParsedPhdr) {
    match phdr.ptype {
        PT_INTERP => {
            let interp_str = format_string_slice(
                &elf.contents[phdr.file_offset..phdr.file_offset + phdr.file_size - 1],
            );

            wrow!(o, 6, "Interpreter", interp_str);
        }
        PT_NOTE => {
            // this is really bad and made out of desperation.
            // notes stored in elf.notes don't have to have 1-to-1
            // correspondence with phdrs, yet here we are.
            for i in 0..elf.notes.len() {
                let note = &elf.notes[i];

                generate_note_data(o, note);

                if i != elf.notes.len() - 1 {
                    w!(o, 6, "<tr> <td><br></td> </tr>");
                }
            }
        }
        _ => {}
    }
}

fn generate_strtab_data(o: &mut String, section: &[u8]) {
    let mut curr_start = 0;

    w!(o, 6, "<tr>");
    w!(o, 7, "<td></td>");
    w!(o, 7, "<td>");
    w!(o, 8, "<div>");

    for (i, c) in section.iter().enumerate() {
        if *c == 0 {
            let end = if curr_start == 0 { 0 } else { i - 1 };

            let maybe = std::str::from_utf8(&section[curr_start..=end]);

            if let Ok(string) = maybe {
                if section[curr_start] != 0 {
                    w!(o, 9, "{}", string);
                }
            }

            curr_start = i + 1;
        }
    }

    w!(o, 8, "</div>");
    w!(o, 7, "</td>");
    w!(o, 6, "</tr>");
}

fn generate_section_info_table(o: &mut String, elf: &ParsedElf, shdr: &ParsedShdr) {
    let section = &elf.contents[shdr.file_offset..shdr.file_offset + shdr.size];

    if shdr.shtype == SHT_STRTAB {
        generate_strtab_data(o, section);
    }
}

fn has_segment_detail(ptype: u32) -> bool {
    matches!(ptype, PT_INTERP | PT_NOTE)
}

fn has_section_detail(ptype: u32) -> bool {
    matches!(ptype, SHT_STRTAB)
}

fn generate_segment_info_tables(o: &mut String, elf: &ParsedElf) {
    for (idx, phdr) in elf.phdrs.iter().enumerate() {
        w!(o, 5, "<table class='conceal itable' id='info_segment{}'>", idx);
        w!(o, 5, "<th colspan='2' class='segment_itable'></th>");

        wrow!(o, 6, "Type", &ptype_to_string(phdr.ptype));
        wrow!(o, 6, "Size in file", size_dualfmt!(phdr.file_size));
        wrow!(o, 6, "Size in memory", size_dualfmt!(phdr.memsz));

        if has_segment_detail(phdr.ptype) {
            w!(o, 6, "<tr><td><br></td></tr>");
            generate_segment_info_table(o, elf, phdr);
        }

        w!(o, 5, "</table>");
    }
}

fn generate_section_info_tables(o: &mut String, elf: &ParsedElf) {
    for (idx, shdr) in elf.shdrs.iter().enumerate() {
        w!(o, 5, "<table class='conceal itable' id='info_section{}'>", idx);
        w!(o, 5, "<th colspan='2' class='section_itable'></th>");

        wrow!(o, 6, "Type", &shtype_to_string(shdr.shtype));
        wrow!(o, 6, "Size", size_dualfmt!(shdr.size));

        if has_section_detail(shdr.shtype) {
            w!(o, 6, "<tr><td><br></td></tr>");
            generate_section_info_table(o, elf, shdr);
        }

        w!(o, 5, "</table>");
    }
}

fn generate_sticky_info_tables(o: &mut String, elf: &ParsedElf) {
    w!(o, 2, "<table id='sticky_table' cellspacing='0'>");
    w!(o, 3, "<tr>");

    w!(o, 4, "<td id='desc'></td>");

    w!(o, 4, "<td class='infotables'>");
    generate_phdr_info_tables(o, elf);

    generate_shdr_info_tables(o, elf);
    w!(o, 4, "</td>");

    w!(o, 4, "<td class='infotables'>");
    generate_segment_info_tables(o, elf);

    generate_section_info_tables(o, elf);
    w!(o, 4, "</td>");

    w!(o, 3, "</tr>");
    w!(o, 2, "</table>");
}

fn add_highlight_script(o: &mut String) {
    let classes = [
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

    w!(o, 2, "<script type='text/javascript'>");

    wnonl!(o, 0, "{}", include_str!("js/highlight.js").indent_lines(3));

    for class in &classes {
        w!(o, 3, "highlightClasses('fileinfo_{}', '{}')", class, class);
    }

    w!(o, 2, "</script>");
}

fn add_description_script(o: &mut String) {
    w!(o, 2, "<script type='text/javascript'>");

    wnonl!(o, 0, "{}", include_str!("js/description.js").indent_lines(3));

    w!(o, 2, "</script>");
}

fn add_conceal_script(o: &mut String) {
    w!(o, 2, "<script type='text/javascript'>");

    wnonl!(o, 0, "{}", include_str!("js/conceal.js").indent_lines(3));

    w!(o, 2, "</script>");
}

fn add_offsets_script(o: &mut String, elf: &ParsedElf) {
    w!(o, 2, "<script type='text/javascript'>");

    w!(o, 3, "let fileLen = {}", elf.file_size);

    wnonl!(o, 0, "{}", include_str!("js/offsets.js").indent_lines(3));

    w!(o, 2, "</script>");
}

fn add_arrows_script(o: &mut String, elf: &ParsedElf) {
    w!(o, 2, "<script type='text/javascript'>");

    wnonl!(o, 0, "{}", include_str!("js/arrows.js").indent_lines(3));

    w!(o, 3, "connect('.e_phoff', '.bin_phdr0');");
    w!(o, 3, "connect('.e_shoff', '.bin_shdr0');");

    for i in 0..elf.phdrs.len() {
        w!(o, 3, "connect('.bin_phdr{} > .p_offset', '.bin_segment{}');", i, i);
    }

    for i in 0..elf.shdrs.len() {
        w!(o, 3, "connect('.bin_shdr{} > .sh_offset', '.bin_section{}');", i, i);

        let link = elf.shdrs[i].link;

        if link != 0 {
            w!(o, 3, "connect('.bin_shdr{} > .sh_link', '.bin_shdr{}');", i, link);
        }
    }

    w!(o, 3, "pushArrowElems();");

    w!(o, 2, "</script>");
}

fn add_settings_script(o: &mut String) {
    w!(o, 2, "<script type='text/javascript'>");

    wnonl!(o, 0, "{}", include_str!("js/settings.js").indent_lines(3));

    w!(o, 2, "</script>");
}

fn add_scripts(o: &mut String, elf: &ParsedElf) {
    add_highlight_script(o);

    add_description_script(o);

    add_conceal_script(o);

    add_offsets_script(o, elf);

    add_arrows_script(o, elf);

    add_settings_script(o);
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

    for idx in 0..elf.contents.len() {
        let byte = elf.contents[idx];

        for range_type in &elf.ranges.data[idx] {
            if *range_type != RangeType::End {
                dump.push_str(format!("<span {}>", range_type.span_attributes()).as_str());
            }
        }

        if idx < 4 {
            dump.push_str(&format_magic(byte));
        } else {
            append_hex_byte(&mut dump, byte);
        }

        for _ in 0..elf.ranges.lookup_range_ends(idx) {
            dump.push_str("</span>");
        }

        if idx != elf.contents.len() - 1 {
            dump.push_str(if (idx + 1) % DEFAULT_COLUMNS == 0 { "\n" } else { " " });
        }
    }

    dump
}

fn generate_ascii_dump(o: &mut String, elf: &ParsedElf) {
    for (i, b) in elf.contents.iter().enumerate() {
        if b.is_ascii_graphic() {
            let ch = *b as char;

            if let Some(escaped) = utils::html_escape(ch) {
                wnonl!(o, 0, "{}", escaped);
            } else {
                wnonl!(o, 0, "{}", ch);
            }
        } else {
            wnonl!(o, 0, ".");
        }

        if (i + 1) % DEFAULT_COLUMNS == 0 {
            w!(o, 0, "");
        }
    }
}

fn generate_body(o: &mut String, elf: &ParsedElf) {
    w!(o, 1, "<body>");

    generate_svg_element(o);

    w!(o, 2, "<div id='rightmenu'>");
    generate_right_menu(o);
    w!(o, 2, "</div>");

    generate_file_info_table(o, elf);

    wnonl!(o, 2, "<div id='offsets'>");
    for off in (0..elf.contents.len()).step_by(DEFAULT_COLUMNS) {
        wnonl!(o, 0, "{:x}", off);
        if off != (elf.contents.len() / DEFAULT_COLUMNS) * DEFAULT_COLUMNS {
            w!(o, 0, "");
        }
    }
    w!(o, 0, "</div>");

    wnonl!(o, 2, "<div id='bytes'>");
    wnonl!(o, 0, "{}", generate_file_dump(elf));
    w!(o, 0, "</div>");

    wnonl!(o, 2, "<div id='ascii'>");
    generate_ascii_dump(o, elf);
    w!(o, 0, "</div>");

    generate_sticky_info_tables(o, elf);

    add_scripts(o, elf);

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
