#![cfg_attr(debug_assertions, allow(dead_code))]

use super::defs::*;
use super::parser::*;
use std::convert::TryInto;

type Elf64Addr = u64;
type Elf64Off = u64;
type Elf64Half = u16;
type Elf64Word = u32;
type Elf64Sword = i32;
type Elf64Xword = u64;
type Elf64Sxword = i64;

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

// All this just to avoid unsafe. This should be improved.
impl Elf64Ehdr {
    fn from_le_bytes(buf: &[u8]) -> Result<Elf64Ehdr, std::array::TryFromSliceError> {
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
    fn from_be_bytes(buf: &[u8]) -> Result<Elf64Ehdr, std::array::TryFromSliceError> {
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

fn add_ehdr_info(ehdr: &Elf64Ehdr, information: &mut Vec<InfoTuple>, ranges: &mut Ranges) {
    ranges.add_range(0, ehdr.e_ehsize as usize, RangeType::FileHeader);

    information.push(("e_type", "Type", type_to_string(ehdr.e_type)));
    ranges.add_range(16, 2, RangeType::HeaderDetail("e_type"));

    information.push((
        "e_machine",
        "Architecture",
        machine_to_string(ehdr.e_machine),
    ));
    ranges.add_range(18, 2, RangeType::HeaderDetail("e_machine"));

    ranges.add_range(20, 4, RangeType::HeaderDetail("e_version"));

    information.push(("e_entry", "Entrypoint", format!("0x{:x}", ehdr.e_entry)));
    ranges.add_range(24, 8, RangeType::HeaderDetail("e_entry"));

    information.push((
        "ph",
        "Program headers",
        format!(
            "<span id='info_e_phnum'>{}</span> * \
             <span id='info_e_phentsize'>0x{:x}</span> @ \
             <span id='info_e_phoff'>{}</span>",
            ehdr.e_phnum, ehdr.e_phentsize, ehdr.e_phoff
        ),
    ));

    information.push((
        "sh",
        "Section headers",
        format!(
            "<span id='info_e_shnum'>{}</span> * \
             <span id='info_e_shentsize'>0x{:x}</span> @ \
             <span id='info_e_shoff'>{}</span>",
            ehdr.e_shnum, ehdr.e_shentsize, ehdr.e_shoff
        ),
    ));

    ranges.add_range(32, 8, RangeType::HeaderDetail("e_phoff"));
    ranges.add_range(40, 8, RangeType::HeaderDetail("e_shoff"));

    if ehdr.e_flags != 0 {
        information.push(("e_flags", "Flags", format!("0x{:x}", ehdr.e_flags)));
    }
    ranges.add_range(48, 4, RangeType::HeaderDetail("e_flags"));
    ranges.add_range(52, 2, RangeType::HeaderDetail("e_ehsize"));

    ranges.add_range(54, 2, RangeType::HeaderDetail("e_phentsize"));
    ranges.add_range(56, 2, RangeType::HeaderDetail("e_phnum"));
    ranges.add_range(58, 2, RangeType::HeaderDetail("e_shentsize"));
    ranges.add_range(60, 2, RangeType::HeaderDetail("e_shnum"));
    ranges.add_range(62, 2, RangeType::HeaderDetail("e_shstrndx"));
}

pub fn parse(
    buf: &Vec<u8>,
    ident: &ParsedIdent,
    information: &mut Vec<InfoTuple>,
    ranges: &mut Ranges,
) -> Result<(), String> {
    let ehdr_size = std::mem::size_of::<Elf64Ehdr>();

    if buf.len() < ehdr_size {
        return Err(String::from("file is smaller than ELF file header"));
    }

    let ehdr_slice = &buf[0..ehdr_size];

    let ehdr = if ident.endianness == ELF_DATA2LSB {
        Elf64Ehdr::from_le_bytes(ehdr_slice)
    } else {
        Elf64Ehdr::from_be_bytes(ehdr_slice)
    }
    .map_err(|a| String::from(format!("failed to read file header: {}", a)))?;

    add_ehdr_info(&ehdr, information, ranges);

    Ok(())
}
