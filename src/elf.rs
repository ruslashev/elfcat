type Elf64Addr = u64;
type Elf64Off = u64;
type Elf64Half = u16;
type Elf64Word = u32;
type Elf64Sword = i32;
type Elf64Xword = u64;
type Elf64Sxword = i64;

const ELF_EI_MAG0: u8 = 0;
const ELF_EI_MAG1: u8 = 1;
const ELF_EI_MAG2: u8 = 2;
const ELF_EI_MAG3: u8 = 3;
const ELF_EI_CLASS: u8 = 4;
const ELF_EI_DATA: u8 = 5;
const ELF_EI_VERSION: u8 = 6;
const ELF_EI_OSABI: u8 = 7;
const ELF_EI_ABIVERSION: u8 = 8;
const ELF_EI_PAD: u8 = 9;
const ELF_EI_NIDENT: u8 = 16;

#[repr(packed)]
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

pub struct ParsedElf {
    pub filename: String,
    pub contents: Vec<u8>,
}

impl ParsedElf {
    pub fn from_bytes(filename: &String, buf: Vec<u8>) -> Result<ParsedElf, &str> {
        if buf.len() < ELF_EI_NIDENT as usize {
            return Err("file smaller than ELF file header");
        }

        if buf[0..=ELF_EI_MAG3 as usize] != [0x7f, 'E' as u8, 'L' as u8, 'F' as u8] {
            return Err("mismatched magic: not an ELF file");
        }

        Ok(ParsedElf {
            filename: filename.clone(),
            contents: buf,
        })
    }
}
