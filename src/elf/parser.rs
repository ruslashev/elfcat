use super::defs::*;
use super::types::*;
use std::convert::TryInto;

#[allow(dead_code)] // REMOVEME
struct ElfEhdr {
    e_ident: [u8; 16],
    e_type: Elf64Half,
    e_machine: Elf64Half,
    e_version: Elf64Word,
    e_entry: Elf64Addr,
    e_phoff: Elf64Off,
    e_shoff: Elf64Off,
    e_flags: Elf64Word,
    e_ehsize: Elf64Half,
    e_phentsize: Elf64Half,
    e_phnum: Elf64Half,
    e_shentsize: Elf64Half,
    e_shnum: Elf64Half,
    e_shstrndx: Elf64Half,
}

// All this just to avoid unsafe. This should be improved.
impl ElfEhdr {
    pub fn from_le_bytes(buf: &[u8]) -> ElfEhdr {
        ElfEhdr {
            e_ident: buf[0..16].try_into().unwrap(),
            e_type: Elf64Half::from_le_bytes(buf[16..18].try_into().unwrap()),
            e_machine: Elf64Half::from_le_bytes(buf[18..20].try_into().unwrap()),
            e_version: Elf64Word::from_le_bytes(buf[20..24].try_into().unwrap()),
            e_entry: Elf64Addr::from_le_bytes(buf[24..32].try_into().unwrap()),
            e_phoff: Elf64Off::from_le_bytes(buf[32..40].try_into().unwrap()),
            e_shoff: Elf64Off::from_le_bytes(buf[40..48].try_into().unwrap()),
            e_flags: Elf64Word::from_le_bytes(buf[48..52].try_into().unwrap()),
            e_ehsize: Elf64Half::from_le_bytes(buf[52..54].try_into().unwrap()),
            e_phentsize: Elf64Half::from_le_bytes(buf[54..56].try_into().unwrap()),
            e_phnum: Elf64Half::from_le_bytes(buf[56..58].try_into().unwrap()),
            e_shentsize: Elf64Half::from_le_bytes(buf[58..60].try_into().unwrap()),
            e_shnum: Elf64Half::from_le_bytes(buf[60..62].try_into().unwrap()),
            e_shstrndx: Elf64Half::from_le_bytes(buf[62..64].try_into().unwrap()),
        }
    }
    pub fn from_be_bytes(buf: &[u8]) -> ElfEhdr {
        ElfEhdr {
            e_ident: buf[0..16].try_into().unwrap(),
            e_type: Elf64Half::from_be_bytes(buf[16..18].try_into().unwrap()),
            e_machine: Elf64Half::from_be_bytes(buf[18..20].try_into().unwrap()),
            e_version: Elf64Word::from_be_bytes(buf[20..24].try_into().unwrap()),
            e_entry: Elf64Addr::from_be_bytes(buf[24..32].try_into().unwrap()),
            e_phoff: Elf64Off::from_be_bytes(buf[32..40].try_into().unwrap()),
            e_shoff: Elf64Off::from_be_bytes(buf[40..48].try_into().unwrap()),
            e_flags: Elf64Word::from_be_bytes(buf[48..52].try_into().unwrap()),
            e_ehsize: Elf64Half::from_be_bytes(buf[52..54].try_into().unwrap()),
            e_phentsize: Elf64Half::from_be_bytes(buf[54..56].try_into().unwrap()),
            e_phnum: Elf64Half::from_be_bytes(buf[56..58].try_into().unwrap()),
            e_shentsize: Elf64Half::from_be_bytes(buf[58..60].try_into().unwrap()),
            e_shnum: Elf64Half::from_be_bytes(buf[60..62].try_into().unwrap()),
            e_shstrndx: Elf64Half::from_be_bytes(buf[62..64].try_into().unwrap()),
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum RangeTypes {
    None,
    End,
    FileHeader,
}

impl RangeTypes {
    pub fn class(&self) -> &str {
        match self {
            RangeTypes::FileHeader => "ehdr",
            _ => "",
        }
    }
}

struct ParsedIdent {
    magic: [u8; 4],
    class: u8,
    endianness: u8,
    version: u8,
    abi: u8,
    abi_ver: u8,
}

impl ParsedIdent {
    fn from_bytes(buf: &Vec<u8>) -> ParsedIdent {
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

pub struct ParsedElf {
    pub filename: String,
    pub information: Vec<(&'static str, String)>,
    pub contents: Vec<u8>,
    pub ranges: Vec<RangeTypes>,
}

impl ParsedElf {
    pub fn from_bytes(filename: &String, buf: Vec<u8>) -> Result<ParsedElf, String> {
        if buf.len() < ELF_EI_NIDENT as usize {
            return Err(String::from("file is smaller than ELF header's e_ident"));
        }

        let ident = ParsedIdent::from_bytes(&buf);

        if ident.magic != [0x7f, 'E' as u8, 'L' as u8, 'F' as u8] {
            return Err(String::from("mismatched magic: not an ELF file"));
        }

        let mut information = vec![];

        information.push((
            "Object class",
            match ident.class {
                ELF_CLASS32 => String::from("32-bit"),
                ELF_CLASS64 => String::from("64-bit"),
                x => format!("Unknown: {}", x),
            },
        ));

        information.push((
            "Data encoding",
            match ident.endianness {
                ELF_DATA2LSB => String::from("Little endian"),
                ELF_DATA2MSB => String::from("Big endian"),
                x => return Err(format!("Unknown endianness: {}", x)),
            },
        ));

        if ident.version != ELF_EV_CURRENT {
            information.push(("Uncommon version(!)", format!("{}", ident.version)));
        }

        information.push(("ABI", abi_to_string(ident.abi)));

        if !(ident.abi == ELF_OSABI_SYSV && ident.abi_ver == 0) {
            information.push(("Uncommon ABI version(!)", format!("{}", ident.abi_ver)));
        }

        let ehdr_size = std::mem::size_of::<ElfEhdr>();

        if buf.len() < ehdr_size {
            return Err(String::from("file is smaller than ELF file header"));
        }

        let ehdr_slice = &buf[0..ehdr_size];

        let ehdr = if ident.endianness == ELF_DATA2LSB {
            ElfEhdr::from_le_bytes(ehdr_slice)
        } else {
            ElfEhdr::from_be_bytes(ehdr_slice)
        };

        information.push(("Type", type_to_string(ehdr.e_type)));
        information.push(("Architecture", machine_to_string(ehdr.e_machine)));
        information.push(("Entrypoint", format!("0x{:x}", ehdr.e_entry)));
        information.push((
            "Program headers",
            format!(
                "{} * 0x{:x} @ {}",
                ehdr.e_phnum, ehdr.e_phentsize, ehdr.e_phoff
            ),
        ));
        information.push((
            "Section headers",
            format!(
                "{} * 0x{:x} @ {}",
                ehdr.e_shnum, ehdr.e_shentsize, ehdr.e_shoff
            ),
        ));

        let mut ranges = vec![RangeTypes::None; buf.len()];

        ranges[0] = RangeTypes::FileHeader;
        ranges[ehdr_size - 1] = RangeTypes::End;

        Ok(ParsedElf {
            filename: filename.clone(),
            information,
            contents: buf,
            ranges,
        })
    }
}
