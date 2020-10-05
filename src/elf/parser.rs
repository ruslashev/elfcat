use super::defs::*;
use super::types::*;
use bincode::Options;
use serde::Deserialize;

#[repr(packed)]
#[derive(Deserialize)]
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

        if buf[0..=ELF_EI_MAG3 as usize] != [0x7f, 'E' as u8, 'L' as u8, 'F' as u8] {
            return Err(String::from("mismatched magic: not an ELF file"));
        }

        let mut information = vec![];

        information.push((
            "Object class",
            match buf[ELF_EI_CLASS as usize] {
                ELF_CLASS32 => String::from("32-bit"),
                ELF_CLASS64 => String::from("64-bit"),
                x => format!("Unknown: {}", x),
            },
        ));

        let endianness = buf[ELF_EI_DATA as usize];
        information.push((
            "Data encoding",
            match endianness {
                ELF_DATA2LSB => String::from("Little endian"),
                ELF_DATA2MSB => String::from("Big endian"),
                x => return Err(format!("Unknown endianness: {}", x)),
            },
        ));

        let ver = buf[ELF_EI_VERSION as usize];
        if ver != ELF_EV_CURRENT {
            information.push(("Uncommon version(!)", format!("{}", ver)));
        }

        let abi = buf[ELF_EI_OSABI as usize];
        information.push(("ABI", abi_to_string(abi)));

        let abi_ver = buf[ELF_EI_ABIVERSION as usize];
        if !(abi == ELF_OSABI_SYSV && abi_ver == 0) {
            information.push(("Uncommon ABI version(!)", format!("{}", abi_ver)));
        }

        let ehdr_size = std::mem::size_of::<ElfEhdr>();

        if buf.len() < ehdr_size {
            return Err(String::from("file is smaller than ELF file header"));
        }

        let ehdr_slice = &buf[0..ehdr_size];

        let maybe: bincode::Result<ElfEhdr> = if endianness == ELF_DATA2LSB {
            bincode::DefaultOptions::new()
                .with_little_endian()
                .deserialize_from(ehdr_slice)
        } else {
            bincode::DefaultOptions::new()
                .with_big_endian()
                .deserialize_from(ehdr_slice)
        };

        let ehdr: ElfEhdr = match maybe {
            Ok(x) => x,
            Err(why) => return Err(format!("failed to deserialize into file header: {}", why)),
        };

        information.push((
            "Type",
            match ehdr.e_type {
                ELF_ET_NONE => String::from("None (NONE)"),
                ELF_ET_REL => String::from("Relocatable object file (REL)"),
                ELF_ET_EXEC => String::from("Executable file (EXEC)"),
                ELF_ET_DYN => String::from("Shared object file (DYN)"),
                ELF_ET_CORE => String::from("Core file (CORE)"),
                ELF_ET_LOOS | ELF_ET_HIOS => String::from("Environment-specific use"),
                ELF_ET_LOPROC | ELF_ET_HIPROC => String::from("Processor-specific use"),
                x => format!("Unknown {}", x),
            },
        ));

        let mut ranges = vec![RangeTypes::None; buf.len()];

        ranges[0] = RangeTypes::FileHeader;
        ranges[ehdr_size] = RangeTypes::End;

        Ok(ParsedElf {
            filename: filename.clone(),
            information,
            contents: buf,
            ranges,
        })
    }
}
