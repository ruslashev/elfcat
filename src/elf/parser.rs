use super::defs::*;
use super::elf32;
use super::elf64;

pub type InfoTuple = (&'static str, &'static str, String);

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum RangeType {
    End,
    Ident,
    FileHeader,
    HeaderDetail(&'static str),
}

impl RangeType {
    pub fn span_id(&self) -> &str {
        match self {
            RangeType::Ident => "ident",
            RangeType::FileHeader => "ehdr",
            RangeType::HeaderDetail(class) => class,
            _ => "",
        }
    }
}

// Interval tree that allows querying point for all intervals that intersect it should be better
pub struct Ranges {
    pub data: Vec<Vec<RangeType>>,
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

pub struct ParsedIdent {
    pub magic: [u8; 4],
    pub class: u8,
    pub endianness: u8,
    pub version: u8,
    pub abi: u8,
    pub abi_ver: u8,
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
    pub information: Vec<(&'static str, &'static str, String)>,
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

        let mut ranges = Ranges::new(buf.len());

        let mut information = vec![];

        ParsedElf::push_ident_info(&ident, &mut information)?;

        if ident.class == ELF_CLASS32 {
            elf32::parse(&buf, &ident, &mut information, &mut ranges)?;
        } else {
            elf64::parse(&buf, &ident, &mut information, &mut ranges)?;
        }

        ranges.add_range(0, ELF_EI_NIDENT as usize, RangeType::Ident);

        ranges.add_range(0, 4, RangeType::HeaderDetail("magic"));
        ranges.add_range(4, 1, RangeType::HeaderDetail("class"));
        ranges.add_range(5, 1, RangeType::HeaderDetail("data"));
        ranges.add_range(6, 1, RangeType::HeaderDetail("ver"));
        ranges.add_range(7, 1, RangeType::HeaderDetail("abi"));
        ranges.add_range(8, 1, RangeType::HeaderDetail("abi_ver"));
        ranges.add_range(9, 7, RangeType::HeaderDetail("pad"));

        Ok(ParsedElf {
            filename: filename.clone(),
            information,
            contents: buf,
            ranges,
        })
    }

    fn push_ident_info(
        ident: &ParsedIdent,
        information: &mut Vec<InfoTuple>,
    ) -> Result<(), String> {
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
}
