use super::defs::*;
use super::types::*;
use std::convert::TryInto;

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

#[derive(Copy, Clone, PartialEq)]
pub enum RangeType {
    End,
    FileHeader,
}

impl RangeType {
    pub fn span_class(&self) -> &str {
        match self {
            RangeType::FileHeader => "ehdr",
            _ => "",
        }
    }
}

// Interval tree that allows querying point for all intervals that intersect it should be better
pub struct Ranges {
    data: Vec<Vec<RangeType>>,
}

impl Ranges {
    fn new(capacity: usize) -> Ranges {
        Ranges {
            data: vec![vec![]; capacity],
        }
    }

    fn add_range(&mut self, start: usize, end: usize, range_type: RangeType) {
        self.data[start].push(range_type);
        self.data[start + end - 1].push(RangeType::End);
    }

    // `init' is a Haskell term for everything but the last element in a list (like head + tail, and
    // init + last). Used here because we are interested in looking up ranges but not their ends.
    pub fn lookup_range_inits(&self, point: usize) -> Vec<RangeType> {
        let mut result = vec![];

        for range_type in self.data[point].clone() {
            if range_type != RangeType::End {
                result.push(range_type);
            }
        }

        result
    }

    pub fn lookup_range_ends(&self, point: usize) -> usize {
        self.data[point]
            .iter()
            .filter(|&x| *x == RangeType::End)
            .count()
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
    pub ranges: Ranges,
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

        ParsedElf::push_ident_info(&ident, &mut information)?;

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

        let mut ranges = Ranges::new(buf.len());

        ranges.add_range(0, ehdr_size, RangeType::FileHeader);

        Ok(ParsedElf {
            filename: filename.clone(),
            information,
            contents: buf,
            ranges,
        })
    }

    fn push_ident_info(
        ident: &ParsedIdent,
        information: &mut Vec<(&'static str, String)>,
    ) -> Result<(), String> {
        information.push((
            "Object class",
            match ident.class {
                ELF_CLASS32 => String::from("32-bit"),
                ELF_CLASS64 => String::from("64-bit"),
                x => return Err(format!("Unknown bitness: {}", x)),
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

        Ok(())
    }
}
