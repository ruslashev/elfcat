use super::defs::*;
use super::elf32;
use super::elf64;
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
    PhdrField(&'static str),
    Segment(u16),
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

pub struct ParsedElf {
    pub filename: String,
    pub information: Vec<(&'static str, &'static str, String)>,
    pub contents: Vec<u8>,
    pub ranges: Ranges,
    pub phdrs: Vec<ParsedPhdr>,
}

pub struct ParsedPhdr {
    pub ptype: u32,
    pub flags: String,
    pub file_offset: usize,
    pub file_size: usize,
    pub vaddr: usize,
    pub memsz: usize,
    pub alignment: usize,
    pub notes: Vec<Note>,
}

pub struct Note {
    pub name: Vec<u8>,
    pub desc: Vec<u8>,
    pub ntype: u32,
}

impl RangeType {
    fn needs_class(&self) -> bool {
        match self {
            RangeType::ProgramHeader(_) => true,
            RangeType::PhdrField(_) => true,
            RangeType::Segment(_) => true,
            _ => false,
        }
    }

    // for those who need_class()
    fn needs_id(&self) -> bool {
        match self {
            RangeType::ProgramHeader(_) => true,
            RangeType::Segment(_) => true,
            _ => false,
        }
    }

    fn id(&self) -> String {
        match self {
            RangeType::Ident => String::from("ident"),
            RangeType::FileHeader => String::from("ehdr"),
            RangeType::ProgramHeader(idx) => format!("bin_phdr{}", idx),
            RangeType::HeaderField(class) => String::from(*class),
            RangeType::Segment(idx) => format!("bin_segment{}", idx),
            _ => String::new(),
        }
    }

    fn class(&self) -> String {
        match self {
            RangeType::ProgramHeader(_) => String::from("phdr"),
            RangeType::PhdrField(field) => format!("{} phdr_hover", field),
            RangeType::Segment(_) => String::from("segment"),
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

impl ParsedElf {
    pub fn from_bytes(filename: &str, buf: Vec<u8>) -> Result<ParsedElf, String> {
        if buf.len() < ELF_EI_NIDENT as usize {
            return Err(String::from("file is smaller than ELF header's e_ident"));
        }

        let ident = ParsedIdent::from_bytes(&buf);

        if ident.magic != [0x7f, b'E', b'L', b'F'] {
            return Err(String::from("mismatched magic: not an ELF file"));
        }

        let mut elf = ParsedElf {
            filename: filename.to_string(),
            information: vec![],
            contents: vec![],
            ranges: Ranges::new(buf.len()),
            phdrs: vec![],
        };

        elf.push_ident_info(&ident)?;

        if ident.class == ELF_CLASS32 {
            elf32::parse(&buf, &ident, &mut elf)?;
        } else {
            elf64::parse(&buf, &ident, &mut elf)?;
        }

        elf.add_ident_ranges();

        elf.contents = buf;

        Ok(elf)
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
}

impl Note {
    fn from_bytes(buf: &[u8], endianness: u8) -> Option<(Note, usize)> {
        let (namesz, descsz, ntype) = Note::read_header(buf, endianness).ok()?;
        let (namesz, descsz) = (namesz as usize, descsz as usize);

        let name = buf[12..12 + namesz].to_vec();
        let desc = buf[12 + namesz..12 + namesz + descsz].to_vec();

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

// this is pretty ugly
pub fn parse_notes(segment: &[u8], segment_size: usize, endianness: u8) -> Vec<Note> {
    let mut start = 0;
    let mut notes = vec![];

    loop {
        if start >= segment_size {
            break;
        }

        match Note::from_bytes(&segment[start..segment_size], endianness) {
            None => break,
            Some((note, len_taken)) => {
                notes.push(note);
                start += len_taken;
            }
        }
    }

    notes
}
