use super::defs::*;
use super::elf32::Elf32;
use super::elf64::Elf64;
use super::elfxx::ElfXX;
use std::convert::TryInto;

pub type InfoTuple = (&'static str, &'static str, String);

pub type ReadErr = std::array::TryFromSliceError;

#[repr(u8)]
#[derive(Clone, PartialEq)]
pub enum RangeType {
    End,
    Ident,
    FileHeader,
    HeaderField(&'static str),
    ProgramHeader(u32),
    SectionHeader(u32),
    PhdrField(&'static str),
    ShdrField(&'static str),
    Segment(u16),
    Section(u16),
    SegmentSubrange,
}

// Interval tree that allows querying point for all intervals that intersect it should be better.
// We can't beat O(n * m) but the average case should improve.
pub struct Ranges {
    pub data: Vec<Vec<RangeType>>,
}

pub struct ParsedIdent {
    pub magic: [u8; 4],
    pub class: u8,
    pub endianness: u8,
    pub version: u8,
    pub abi: u8,
    pub abi_ver: u8,
}

pub struct ParsedElf<'a> {
    pub filename: String,
    pub file_size: usize,
    pub information: Vec<(&'static str, &'static str, String)>,
    pub contents: &'a [u8],
    pub ranges: Ranges,
    pub phdrs: Vec<ParsedPhdr>,
    pub shdrs: Vec<ParsedShdr>,
    pub strtab: StrTab<'a>,
    pub shstrndx: u16,
    pub shnstrtab: StrTab<'a>,
    pub notes: Vec<Note>,
}

pub struct ParsedPhdr {
    pub ptype: u32,
    pub flags: String,
    pub file_offset: usize,
    pub file_size: usize,
    pub vaddr: usize,
    pub memsz: usize,
    pub alignment: usize,
}

pub struct ParsedShdr {
    pub name: usize,
    pub shtype: u32,
    pub flags: u64,
    pub addr: usize,
    pub file_offset: usize,
    pub size: usize,
    pub link: usize,
    pub info: usize,
    pub addralign: usize,
    pub entsize: usize,
}

pub struct Note {
    pub name: Vec<u8>,
    pub desc: Vec<u8>,
    pub ntype: u32,
}

pub struct StrTab<'a> {
    strings: &'a [u8],
    section_size: usize,
}

impl RangeType {
    // this is a bit of a clusterfuck
    fn needs_class(&self) -> bool {
        match self {
            RangeType::Ident => true,
            RangeType::FileHeader => true,
            RangeType::ProgramHeader(_) => true,
            RangeType::SectionHeader(_) => true,
            RangeType::PhdrField(_) => true,
            RangeType::ShdrField(_) => true,
            RangeType::Segment(_) => true,
            RangeType::Section(_) => true,
            RangeType::SegmentSubrange => true,
            _ => false,
        }
    }

    // for those who need_class()
    fn needs_id(&self) -> bool {
        match self {
            RangeType::ProgramHeader(_) => true,
            RangeType::SectionHeader(_) => true,
            RangeType::Segment(_) => true,
            RangeType::Section(_) => true,
            _ => false,
        }
    }

    fn id(&self) -> String {
        match self {
            RangeType::ProgramHeader(idx) => format!("bin_phdr{}", idx),
            RangeType::SectionHeader(idx) => format!("bin_shdr{}", idx),
            RangeType::HeaderField(class) => String::from(*class),
            RangeType::Segment(idx) => format!("bin_segment{}", idx),
            RangeType::Section(idx) => format!("bin_section{}", idx),
            _ => String::new(),
        }
    }

    fn class(&self) -> String {
        match self {
            RangeType::Ident => String::from("ident"),
            RangeType::FileHeader => String::from("ehdr"),
            RangeType::ProgramHeader(_) => String::from("phdr"),
            RangeType::SectionHeader(_) => String::from("shdr"),
            RangeType::PhdrField(field) => format!("{} phdr_hover", field),
            RangeType::ShdrField(field) => format!("{} shdr_hover", field),
            RangeType::Segment(_) => String::from("segment"),
            RangeType::Section(_) => String::from("section"),
            RangeType::SegmentSubrange => String::from("segment_subrange"),
            _ => String::new(),
        }
    }

    fn always_highlight(&self) -> bool {
        match self {
            RangeType::HeaderField(class) => match *class {
                "magic" => true,
                "ver" => true,
                "abi_ver" => true,
                "pad" => true,
                "e_version" => true,
                "e_flags" => true,
                "e_ehsize" => true,
                "e_shstrndx" => true,
                _ => false,
            },
            RangeType::Segment(_) => true,
            RangeType::Section(_) => true,
            RangeType::SegmentSubrange => true,
            _ => false,
        }
    }

    pub fn span_attributes(&self) -> String {
        if self.needs_class() {
            format!(
                "{}class='{}{}'",
                if self.needs_id() {
                    format!("id='{}' ", self.id())
                } else {
                    String::new()
                },
                self.class(),
                if self.always_highlight() {
                    " hover"
                } else {
                    ""
                }
            )
        } else {
            format!("id='{}'", self.id())
                + if self.always_highlight() {
                    " class='hover'"
                } else {
                    ""
                }
        }
    }

    pub fn skippable(&self) -> bool {
        match self {
            RangeType::Segment(_) => true,
            RangeType::Section(_) => true,
            _ => false,
        }
    }
}

impl Ranges {
    fn new(capacity: usize) -> Ranges {
        Ranges {
            data: vec![vec![]; capacity],
        }
    }

    pub fn add_range(&mut self, start: usize, end: usize, range_type: RangeType) {
        self.data[start].push(range_type);
        self.data[start + end - 1].push(RangeType::End);
    }

    pub fn lookup_range_ends(&self, point: usize) -> usize {
        self.data[point]
            .iter()
            .filter(|&x| *x == RangeType::End)
            .count()
    }
}

impl ParsedIdent {
    fn from_bytes(buf: &[u8]) -> ParsedIdent {
        ParsedIdent {
            magic: [buf[0], buf[1], buf[2], buf[3]],
            class: buf[ELF_EI_CLASS as usize],
            endianness: buf[ELF_EI_DATA as usize],
            version: buf[ELF_EI_VERSION as usize],
            abi: buf[ELF_EI_OSABI as usize],
            abi_ver: buf[ELF_EI_ABIVERSION as usize],
        }
    }
}

impl ParsedElf<'_> {
    pub fn from_bytes<'a>(filename: &str, buf: &'a [u8]) -> Result<ParsedElf<'a>, String> {
        if buf.len() < ELF_EI_NIDENT as usize {
            return Err(String::from("file is smaller than ELF header's e_ident"));
        }

        let ident = ParsedIdent::from_bytes(&buf);

        if ident.magic != [0x7f, b'E', b'L', b'F'] {
            return Err(String::from("mismatched magic: not an ELF file"));
        }

        let mut elf = ParsedElf {
            filename: filename.to_string(),
            file_size: buf.len(),
            information: vec![],
            contents: buf,
            ranges: Ranges::new(buf.len()),
            phdrs: vec![],
            shdrs: vec![],
            strtab: StrTab::empty(),
            shstrndx: 0,
            shnstrtab: StrTab::empty(),
            notes: vec![],
        };

        elf.push_file_info();

        elf.push_ident_info(&ident)?;

        if ident.class == ELF_CLASS32 {
            Elf32::parse(&buf, &ident, &mut elf)?;
        } else {
            Elf64::parse(&buf, &ident, &mut elf)?;
        }

        elf.add_ident_ranges();

        elf.parse_string_tables();

        elf.parse_notes(ident.endianness);

        Ok(elf)
    }

    fn push_file_info(&mut self) {
        self.information
            .push(("file_name", "File name", self.filename.to_string()));

        let file_size_str = if self.file_size < 1024 {
            format!("{} B", self.file_size)
        } else {
            format!(
                "{} ({} B)",
                crate::utils::human_format_bytes(self.file_size as u64),
                self.file_size
            )
        };

        self.information
            .push(("file_size", "File size", file_size_str));
    }

    fn push_ident_info(&mut self, ident: &ParsedIdent) -> Result<(), String> {
        let information = &mut self.information;

        information.push((
            "class",
            "Object class",
            match ident.class {
                ELF_CLASS32 => String::from("32-bit"),
                ELF_CLASS64 => String::from("64-bit"),
                x => return Err(format!("Unknown bitness: {}", x)),
            },
        ));

        information.push((
            "data",
            "Data encoding",
            match ident.endianness {
                ELF_DATA2LSB => String::from("Little endian"),
                ELF_DATA2MSB => String::from("Big endian"),
                x => return Err(format!("Unknown endianness: {}", x)),
            },
        ));

        if ident.version != ELF_EV_CURRENT {
            information.push(("ver", "Uncommon version(!)", format!("{}", ident.version)));
        }

        information.push((
            "abi",
            if ident.abi == ELF_OSABI_SYSV {
                "ABI"
            } else {
                "Uncommon ABI(!)"
            },
            abi_to_string(ident.abi),
        ));

        if !(ident.abi == ELF_OSABI_SYSV && ident.abi_ver == 0) {
            information.push((
                "abi_ver",
                if ident.abi == ELF_OSABI_SYSV && ident.abi_ver != 0 {
                    "Uncommon ABI version(!)"
                } else {
                    "ABI version"
                },
                format!("{}", ident.abi_ver),
            ));
        }

        Ok(())
    }

    fn add_ident_ranges(&mut self) {
        let ranges = &mut self.ranges;

        ranges.add_range(0, ELF_EI_NIDENT as usize, RangeType::Ident);

        ranges.add_range(0, 4, RangeType::HeaderField("magic"));
        ranges.add_range(4, 1, RangeType::HeaderField("class"));
        ranges.add_range(5, 1, RangeType::HeaderField("data"));
        ranges.add_range(6, 1, RangeType::HeaderField("ver"));
        ranges.add_range(7, 1, RangeType::HeaderField("abi"));
        ranges.add_range(8, 1, RangeType::HeaderField("abi_ver"));
        ranges.add_range(9, 7, RangeType::HeaderField("pad"));
    }

    fn find_strtab_shdr(shdrs: &[ParsedShdr]) -> Option<&ParsedShdr> {
        for shdr in shdrs {
            if shdr.shtype == SHT_STRTAB {
                return Some(shdr);
            }
        }

        None
    }

    fn parse_string_tables(&mut self) {
        let shdr = ParsedElf::find_strtab_shdr(&self.shdrs);

        if let Some(shdr) = shdr {
            let section = &self.contents[shdr.file_offset..shdr.file_offset + shdr.size];

            self.strtab.populate(&section, shdr.size);
        }

        if self.shstrndx != SHN_UNDEF {
            let shdr = &self.shdrs[self.shstrndx as usize];
            let section = &self.contents[shdr.file_offset..shdr.file_offset + shdr.size];

            self.shnstrtab.populate(&section, shdr.size);
        }
    }

    fn parse_notes(&mut self, endianness: u8) {
        let mut areas = vec![];

        for phdr in &self.phdrs {
            if phdr.ptype == PT_NOTE {
                areas.push((phdr.file_offset, phdr.file_size));
            }
        }

        for (start, len) in areas {
            self.parse_note_area(start, len, endianness);
        }
    }

    // this is pretty ugly in terms of raw addressing, unwieldly offsets, etc.
    // area here stands for segment or section because notes may come from either of them.
    fn parse_note_area(&mut self, area_start: usize, area_size: usize, endianness: u8) {
        let area = &self.contents[area_start..area_start + area_size];
        let mut start = 0;

        loop {
            if start >= area_size {
                break;
            }

            match Note::from_bytes(&area[start..area_size], endianness) {
                None => break,
                Some((note, len_taken)) => {
                    // not an ideal look because this can also be a SectionSubrange
                    self.ranges.add_range(
                        area_start + start,
                        len_taken,
                        RangeType::SegmentSubrange,
                    );
                    self.notes.push(note);
                    start += len_taken;
                }
            }
        }
    }
}

impl Note {
    fn from_bytes(buf: &[u8], endianness: u8) -> Option<(Note, usize)> {
        let (namesz, descsz, ntype) = Note::read_header(buf, endianness).ok()?;
        let (namesz, descsz) = (namesz as usize, descsz as usize);

        let name = buf.get(12..12 + namesz)?.to_vec();
        let desc = buf.get(12 + namesz..12 + namesz + descsz)?.to_vec();

        let mut len: usize = 12 + namesz + descsz;

        while len % 4 != 0 {
            len += 1;
        }

        Some((Note { name, desc, ntype }, len))
    }

    fn read_header(buf: &[u8], endianness: u8) -> Result<(u32, u32, u32), ReadErr> {
        Ok(if endianness == ELF_DATA2LSB {
            (
                u32::from_le_bytes(buf[0..4].try_into()?),
                u32::from_le_bytes(buf[4..8].try_into()?),
                u32::from_le_bytes(buf[8..12].try_into()?),
            )
        } else {
            (
                u32::from_be_bytes(buf[0..4].try_into()?),
                u32::from_be_bytes(buf[4..8].try_into()?),
                u32::from_be_bytes(buf[8..12].try_into()?),
            )
        })
    }
}

impl<'a> StrTab<'a> {
    // decently ugly
    fn empty() -> StrTab<'static> {
        StrTab {
            strings: &[],
            section_size: 0,
        }
    }

    // something could be better than references with lifetimes
    fn populate(&mut self, section: &'a [u8], section_size: usize) {
        self.strings = section;
        self.section_size = section_size;
    }

    pub fn get(&self, idx: usize) -> &str {
        let start_idx = idx;

        for end_idx in start_idx..start_idx + self.section_size {
            if self.strings[end_idx] == 0 {
                let maybe = std::str::from_utf8(&self.strings[start_idx..end_idx]);

                if maybe.is_err() {
                    return "";
                }

                return maybe.unwrap();
            }
        }

        ""
    }
}
