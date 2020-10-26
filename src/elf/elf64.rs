#![cfg_attr(debug_assertions, allow(dead_code))]

use super::defs::*;
use super::parser::*;
use std::convert::TryInto;
use std::mem::size_of;

type Elf64Addr = u64;
type Elf64Off = u64;
type Elf64Half = u16;
type Elf64Word = u32;
type Elf64Sword = i32;
type Elf64Xword = u64;
type Elf64Sxword = i64;

type ReadErr = std::array::TryFromSliceError;

#[allow(dead_code)] // REMOVEME
struct Elf64Ehdr {
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

#[allow(dead_code)]
struct Elf64Phdr {
    p_type: Elf64Word,
    p_flags: Elf64Word,
    p_offset: Elf64Off,
    p_vaddr: Elf64Addr,
    p_paddr: Elf64Addr,
    p_filesz: Elf64Xword,
    p_memsz: Elf64Xword,
    p_align: Elf64Xword,
}

#[allow(dead_code)]
struct Elf64Shdr {
    sh_name: Elf64Word,
    sh_type: Elf64Word,
    sh_flags: Elf64Xword,
    sh_addr: Elf64Addr,
    sh_offset: Elf64Off,
    sh_size: Elf64Xword,
    sh_link: Elf64Word,
    sh_info: Elf64Word,
    sh_addralign: Elf64Xword,
    sh_entsize: Elf64Xword,
}

// All this just to avoid unsafe. This should be improved.
impl Elf64Ehdr {
    fn from_bytes(buf: &[u8], endianness: u8) -> Result<Elf64Ehdr, String> {
        if endianness == ELF_DATA2LSB {
            Elf64Ehdr::from_le_bytes(buf)
        } else {
            Elf64Ehdr::from_be_bytes(buf)
        }
        .map_err(|a| String::from(format!("failed to read file header: {}", a)))
    }
    fn from_le_bytes(buf: &[u8]) -> Result<Elf64Ehdr, ReadErr> {
        Ok(Elf64Ehdr {
            e_ident: buf[0..16].try_into()?,
            e_type: Elf64Half::from_le_bytes(buf[16..18].try_into()?),
            e_machine: Elf64Half::from_le_bytes(buf[18..20].try_into()?),
            e_version: Elf64Word::from_le_bytes(buf[20..24].try_into()?),
            e_entry: Elf64Addr::from_le_bytes(buf[24..32].try_into()?),
            e_phoff: Elf64Off::from_le_bytes(buf[32..40].try_into()?),
            e_shoff: Elf64Off::from_le_bytes(buf[40..48].try_into()?),
            e_flags: Elf64Word::from_le_bytes(buf[48..52].try_into()?),
            e_ehsize: Elf64Half::from_le_bytes(buf[52..54].try_into()?),
            e_phentsize: Elf64Half::from_le_bytes(buf[54..56].try_into()?),
            e_phnum: Elf64Half::from_le_bytes(buf[56..58].try_into()?),
            e_shentsize: Elf64Half::from_le_bytes(buf[58..60].try_into()?),
            e_shnum: Elf64Half::from_le_bytes(buf[60..62].try_into()?),
            e_shstrndx: Elf64Half::from_le_bytes(buf[62..64].try_into()?),
        })
    }
    fn from_be_bytes(buf: &[u8]) -> Result<Elf64Ehdr, ReadErr> {
        Ok(Elf64Ehdr {
            e_ident: buf[0..16].try_into()?,
            e_type: Elf64Half::from_be_bytes(buf[16..18].try_into()?),
            e_machine: Elf64Half::from_be_bytes(buf[18..20].try_into()?),
            e_version: Elf64Word::from_be_bytes(buf[20..24].try_into()?),
            e_entry: Elf64Addr::from_be_bytes(buf[24..32].try_into()?),
            e_phoff: Elf64Off::from_be_bytes(buf[32..40].try_into()?),
            e_shoff: Elf64Off::from_be_bytes(buf[40..48].try_into()?),
            e_flags: Elf64Word::from_be_bytes(buf[48..52].try_into()?),
            e_ehsize: Elf64Half::from_be_bytes(buf[52..54].try_into()?),
            e_phentsize: Elf64Half::from_be_bytes(buf[54..56].try_into()?),
            e_phnum: Elf64Half::from_be_bytes(buf[56..58].try_into()?),
            e_shentsize: Elf64Half::from_be_bytes(buf[58..60].try_into()?),
            e_shnum: Elf64Half::from_be_bytes(buf[60..62].try_into()?),
            e_shstrndx: Elf64Half::from_be_bytes(buf[62..64].try_into()?),
        })
    }
}

impl Elf64Phdr {
    fn from_bytes(buf: &[u8], endianness: u8) -> Result<Elf64Phdr, String> {
        if endianness == ELF_DATA2LSB {
            Elf64Phdr::from_le_bytes(buf)
        } else {
            Elf64Phdr::from_be_bytes(buf)
        }
        .map_err(|a| String::from(format!("failed to read program header: {}", a)))
    }
    fn from_le_bytes(buf: &[u8]) -> Result<Elf64Phdr, ReadErr> {
        Ok(Elf64Phdr {
            p_type: Elf64Word::from_le_bytes(buf[0..4].try_into()?),
            p_flags: Elf64Word::from_le_bytes(buf[4..8].try_into()?),
            p_offset: Elf64Off::from_le_bytes(buf[8..16].try_into()?),
            p_vaddr: Elf64Addr::from_le_bytes(buf[16..24].try_into()?),
            p_paddr: Elf64Addr::from_le_bytes(buf[24..32].try_into()?),
            p_filesz: Elf64Xword::from_le_bytes(buf[32..40].try_into()?),
            p_memsz: Elf64Xword::from_le_bytes(buf[40..48].try_into()?),
            p_align: Elf64Xword::from_le_bytes(buf[48..56].try_into()?),
        })
    }
    fn from_be_bytes(buf: &[u8]) -> Result<Elf64Phdr, ReadErr> {
        Ok(Elf64Phdr {
            p_type: Elf64Word::from_be_bytes(buf[0..4].try_into()?),
            p_flags: Elf64Word::from_be_bytes(buf[4..8].try_into()?),
            p_offset: Elf64Off::from_be_bytes(buf[8..16].try_into()?),
            p_vaddr: Elf64Addr::from_be_bytes(buf[16..24].try_into()?),
            p_paddr: Elf64Addr::from_be_bytes(buf[24..32].try_into()?),
            p_filesz: Elf64Xword::from_be_bytes(buf[32..40].try_into()?),
            p_memsz: Elf64Xword::from_be_bytes(buf[40..48].try_into()?),
            p_align: Elf64Xword::from_be_bytes(buf[48..56].try_into()?),
        })
    }
}

impl Elf64Shdr {
    fn from_bytes(buf: &[u8], endianness: u8) -> Result<Elf64Shdr, String> {
        if endianness == ELF_DATA2LSB {
            Elf64Shdr::from_le_bytes(buf)
        } else {
            Elf64Shdr::from_be_bytes(buf)
        }
        .map_err(|a| String::from(format!("failed to read section header: {}", a)))
    }
    fn from_le_bytes(buf: &[u8]) -> Result<Elf64Shdr, ReadErr> {
        Ok(Elf64Shdr {
            sh_name: Elf64Word::from_le_bytes(buf[0..4].try_into()?),
            sh_type: Elf64Word::from_le_bytes(buf[4..8].try_into()?),
            sh_flags: Elf64Xword::from_le_bytes(buf[8..16].try_into()?),
            sh_addr: Elf64Addr::from_le_bytes(buf[16..24].try_into()?),
            sh_offset: Elf64Off::from_le_bytes(buf[24..32].try_into()?),
            sh_size: Elf64Xword::from_le_bytes(buf[32..40].try_into()?),
            sh_link: Elf64Word::from_le_bytes(buf[40..44].try_into()?),
            sh_info: Elf64Word::from_le_bytes(buf[44..48].try_into()?),
            sh_addralign: Elf64Xword::from_le_bytes(buf[48..56].try_into()?),
            sh_entsize: Elf64Xword::from_le_bytes(buf[56..64].try_into()?),
        })
    }
    fn from_be_bytes(buf: &[u8]) -> Result<Elf64Shdr, ReadErr> {
        Ok(Elf64Shdr {
            sh_name: Elf64Word::from_be_bytes(buf[0..4].try_into()?),
            sh_type: Elf64Word::from_be_bytes(buf[4..8].try_into()?),
            sh_flags: Elf64Xword::from_be_bytes(buf[8..16].try_into()?),
            sh_addr: Elf64Addr::from_be_bytes(buf[16..24].try_into()?),
            sh_offset: Elf64Off::from_be_bytes(buf[24..32].try_into()?),
            sh_size: Elf64Xword::from_be_bytes(buf[32..40].try_into()?),
            sh_link: Elf64Word::from_be_bytes(buf[40..44].try_into()?),
            sh_info: Elf64Word::from_be_bytes(buf[44..48].try_into()?),
            sh_addralign: Elf64Xword::from_be_bytes(buf[48..56].try_into()?),
            sh_entsize: Elf64Xword::from_be_bytes(buf[56..64].try_into()?),
        })
    }
}

pub fn parse(buf: &Vec<u8>, ident: &ParsedIdent, elf: &mut ParsedElf) -> Result<(), String> {
    let ehdr_size = size_of::<Elf64Ehdr>();

    if buf.len() < ehdr_size {
        return Err(String::from("file is smaller than ELF file header"));
    }

    let ehdr = Elf64Ehdr::from_bytes(&buf[0..ehdr_size], ident.endianness)?;

    parse_ehdr(&ehdr, elf);

    parse_phdrs(buf, ident.endianness, &ehdr, elf)?;

    Ok(())
}

fn parse_ehdr(ehdr: &Elf64Ehdr, elf: &mut ParsedElf) {
    push_ehdr_info(ehdr, &mut elf.information);

    add_ehdr_ranges(ehdr, &mut elf.ranges);
}

fn push_ehdr_info(ehdr: &Elf64Ehdr, information: &mut Vec<InfoTuple>) {
    information.push(("e_type", "Type", type_to_string(ehdr.e_type)));

    information.push((
        "e_machine",
        "Architecture",
        machine_to_string(ehdr.e_machine),
    ));

    information.push(("e_entry", "Entrypoint", format!("0x{:x}", ehdr.e_entry)));

    information.push((
        "ph",
        "Program headers",
        format!(
            "<span id='info_e_phnum'>{}</span> * \
             <span id='info_e_phentsize'>{}</span> @ \
             <span id='info_e_phoff'>{}</span>",
            ehdr.e_phnum, ehdr.e_phentsize, ehdr.e_phoff
        ),
    ));

    information.push((
        "sh",
        "Section headers",
        format!(
            "<span id='info_e_shnum'>{}</span> * \
             <span id='info_e_shentsize'>{}</span> @ \
             <span id='info_e_shoff'>{}</span>",
            ehdr.e_shnum, ehdr.e_shentsize, ehdr.e_shoff
        ),
    ));

    if ehdr.e_flags != 0 {
        information.push(("e_flags", "Flags", format!("0x{:x}", ehdr.e_flags)));
    }
}

fn add_ehdr_ranges(ehdr: &Elf64Ehdr, ranges: &mut Ranges) {
    ranges.add_range(0, ehdr.e_ehsize as usize, RangeType::FileHeader);
    ranges.add_range(16, 2, RangeType::HeaderField("e_type"));
    ranges.add_range(18, 2, RangeType::HeaderField("e_machine"));
    ranges.add_range(20, 4, RangeType::HeaderField("e_version"));
    ranges.add_range(24, 8, RangeType::HeaderField("e_entry"));
    ranges.add_range(32, 8, RangeType::HeaderField("e_phoff"));
    ranges.add_range(40, 8, RangeType::HeaderField("e_shoff"));
    ranges.add_range(48, 4, RangeType::HeaderField("e_flags"));
    ranges.add_range(52, 2, RangeType::HeaderField("e_ehsize"));
    ranges.add_range(54, 2, RangeType::HeaderField("e_phentsize"));
    ranges.add_range(56, 2, RangeType::HeaderField("e_phnum"));
    ranges.add_range(58, 2, RangeType::HeaderField("e_shentsize"));
    ranges.add_range(60, 2, RangeType::HeaderField("e_shnum"));
    ranges.add_range(62, 2, RangeType::HeaderField("e_shstrndx"));
}

fn parse_phdrs(
    buf: &Vec<u8>,
    endianness: u8,
    ehdr: &Elf64Ehdr,
    elf: &mut ParsedElf,
) -> Result<(), String> {
    let mut start = ehdr.e_phoff as usize;
    let phsize = size_of::<Elf64Phdr>();

    for i in 0..ehdr.e_phnum {
        let phdr = Elf64Phdr::from_bytes(&buf[start..start + phsize], endianness)?;
        let parsed = parse_phdr(&phdr);
        let ranges = &mut elf.ranges;

        if parsed.file_offset != 0 && parsed.file_size != 0 {
            ranges.add_range(parsed.file_offset, parsed.file_size, RangeType::PhdrData(i));
        }

        ranges.add_range(start, phsize, RangeType::ProgramHeader(i as u32));

        add_phdr_ranges(start, ranges);

        elf.phdrs.push(parsed);

        start += phsize;
    }

    Ok(())
}

fn add_phdr_ranges(start: usize, ranges: &mut Ranges) {
    ranges.add_range(start + 0, 4, RangeType::PhdrField("p_type"));
    ranges.add_range(start + 4, 4, RangeType::PhdrField("p_flags"));
    ranges.add_range(start + 8, 8, RangeType::PhdrField("p_offset"));
    ranges.add_range(start + 16, 8, RangeType::PhdrField("p_vaddr"));
    ranges.add_range(start + 24, 8, RangeType::PhdrField("p_paddr"));
    ranges.add_range(start + 32, 8, RangeType::PhdrField("p_filesz"));
    ranges.add_range(start + 40, 8, RangeType::PhdrField("p_memsz"));
    ranges.add_range(start + 48, 8, RangeType::PhdrField("p_align"));
}

fn parse_phdr(phdr: &Elf64Phdr) -> ParsedPhdr {
    ParsedPhdr {
        ptype: phdr.p_type,
        flags: pflags_to_string(phdr.p_flags),
        file_offset: phdr.p_offset as usize,
        file_size: phdr.p_filesz as usize,
        vaddr: phdr.p_vaddr as usize,
        memsz: phdr.p_memsz as usize,
        alignment: phdr.p_align as usize,
    }
}
