#![cfg_attr(debug_assertions, allow(dead_code))]

use std::convert::TryInto;

pub type Elf64Addr = u64;
pub type Elf64Off = u64;
pub type Elf64Half = u16;
pub type Elf64Word = u32;
pub type Elf64Sword = i32;
pub type Elf64Xword = u64;
pub type Elf64Sxword = i64;

#[allow(dead_code)] // REMOVEME
pub struct Elf64Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: Elf64Half,
    pub e_machine: Elf64Half,
    pub e_version: Elf64Word,
    pub e_entry: Elf64Addr,
    pub e_phoff: Elf64Off,
    pub e_shoff: Elf64Off,
    pub e_flags: Elf64Word,
    pub e_ehsize: Elf64Half,
    pub e_phentsize: Elf64Half,
    pub e_phnum: Elf64Half,
    pub e_shentsize: Elf64Half,
    pub e_shnum: Elf64Half,
    pub e_shstrndx: Elf64Half,
}

// All this just to avoid unsafe. This should be improved.
impl Elf64Ehdr {
    pub fn from_le_bytes(buf: &[u8]) -> Result<Elf64Ehdr, std::array::TryFromSliceError> {
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
    pub fn from_be_bytes(buf: &[u8]) -> Result<Elf64Ehdr, std::array::TryFromSliceError> {
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
