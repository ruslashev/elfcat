use super::defs::*;
use super::parser::*;
use std::mem::size_of;

pub trait ElfHeader: Sized {
    fn from_le_bytes(buf: &[u8]) -> Result<Self, ReadErr>;
    fn from_be_bytes(buf: &[u8]) -> Result<Self, ReadErr>;
    fn describe() -> String;

    fn from_bytes(buf: &[u8], endianness: u8) -> Result<Self, String> {
        if endianness == ELF_DATA2LSB {
            Self::from_le_bytes(buf)
        } else {
            Self::from_be_bytes(buf)
        }
        .map_err(|a| format!("failed to read {}: {}", Self::describe(), a))
    }
}

// We do this because we can't access struct fields of a generic type
pub trait ElfXXEhdr<ElfXXAddr, ElfXXHalf, ElfXXWord, ElfXXOff>: ElfHeader {
    fn e_ident(&self) -> [u8; 16];
    fn e_type(&self) -> ElfXXHalf;
    fn e_machine(&self) -> ElfXXHalf;
    fn e_version(&self) -> ElfXXWord;
    fn e_entry(&self) -> ElfXXAddr;
    fn e_phoff(&self) -> ElfXXOff;
    fn e_shoff(&self) -> ElfXXOff;
    fn e_flags(&self) -> ElfXXWord;
    fn e_ehsize(&self) -> ElfXXHalf;
    fn e_phentsize(&self) -> ElfXXHalf;
    fn e_phnum(&self) -> ElfXXHalf;
    fn e_shentsize(&self) -> ElfXXHalf;
    fn e_shnum(&self) -> ElfXXHalf;
    fn e_shstrndx(&self) -> ElfXXHalf;
}

pub trait ElfXXPhdr<ElfXXAddr, ElfXXWord, ElfXXOff, ElfXXXword>: ElfHeader {
    fn p_type(&self) -> ElfXXWord;
    fn p_flags(&self) -> ElfXXWord;
    fn p_offset(&self) -> ElfXXOff;
    fn p_vaddr(&self) -> ElfXXAddr;
    fn p_paddr(&self) -> ElfXXAddr;
    fn p_filesz(&self) -> ElfXXXword;
    fn p_memsz(&self) -> ElfXXXword;
    fn p_align(&self) -> ElfXXXword;
}

pub trait ElfXXShdr<ElfXXAddr, ElfXXWord, ElfXXOff, ElfXXXword>: ElfHeader {
    fn sh_name(&self) -> ElfXXWord;
    fn sh_type(&self) -> ElfXXWord;
    fn sh_flags(&self) -> ElfXXXword;
    fn sh_addr(&self) -> ElfXXAddr;
    fn sh_offset(&self) -> ElfXXOff;
    fn sh_size(&self) -> ElfXXXword;
    fn sh_link(&self) -> ElfXXWord;
    fn sh_info(&self) -> ElfXXWord;
    fn sh_addralign(&self) -> ElfXXXword;
    fn sh_entsize(&self) -> ElfXXXword;
}

macro_rules! read_field {
    ($name:ident, $field:ident) => {
        $name
            .$field()
            .try_into()
            .map_err(|_| format!("failed to read {}", stringify!($field)))
    };
}

pub trait ElfXX<EhdrT, PhdrT, ShdrT, ElfXXAddr, ElfXXHalf, ElfXXWord, ElfXXOff, ElfXXXword>
where
    EhdrT: ElfXXEhdr<ElfXXAddr, ElfXXHalf, ElfXXWord, ElfXXOff>,
    PhdrT: ElfXXPhdr<ElfXXAddr, ElfXXWord, ElfXXOff, ElfXXXword>,
    ShdrT: ElfXXShdr<ElfXXAddr, ElfXXWord, ElfXXOff, ElfXXXword>,
    u32: From<ElfXXWord>,
    u64: From<ElfXXXword>,
    // This is a bit of a mess
    ElfXXAddr: std::convert::TryInto<usize> + std::fmt::Display + std::fmt::LowerHex,
    ElfXXHalf: std::convert::Into<u16> + std::fmt::Display + std::fmt::LowerHex,
    ElfXXWord: std::convert::TryInto<usize> + std::fmt::LowerHex,
    ElfXXOff: std::convert::TryInto<usize> + std::fmt::Display + std::fmt::LowerHex,
    ElfXXXword: std::convert::TryInto<usize>,
{
    fn parse(buf: &[u8], ident: &ParsedIdent, elf: &mut ParsedElf) -> Result<(), String> {
        let ehdr_size = size_of::<EhdrT>();

        if buf.len() < ehdr_size {
            return Err(String::from("file is smaller than ELF file header"));
        }

        let ehdr = EhdrT::from_bytes(&buf[0..ehdr_size], ident.endianness)?;

        elf.shstrndx = ehdr.e_shstrndx().into();

        Self::parse_ehdr(&ehdr, elf);

        Self::parse_phdrs(buf, ident.endianness, &ehdr, elf)?;

        Self::parse_shdrs(buf, ident.endianness, &ehdr, elf)?;

        Ok(())
    }

    fn parse_ehdr(ehdr: &EhdrT, elf: &mut ParsedElf) {
        Self::push_ehdr_info(ehdr, &mut elf.information);

        Self::add_ehdr_ranges(ehdr, &mut elf.ranges);
    }

    fn push_ehdr_info(ehdr: &EhdrT, information: &mut Vec<InfoTuple>) {
        information.push(("e_type", "Type", type_to_string(ehdr.e_type().into())));

        information.push(("e_machine", "Architecture", machine_to_string(ehdr.e_machine().into())));

        information.push((
            "e_entry",
            "Entrypoint",
            format!("<span class='number' title='{}'>{:#x}</span>", ehdr.e_entry(), ehdr.e_entry()),
        ));

        information.push((
            "ph",
            "Program headers",
            format!(
                "<span title='{:#x}' class='number fileinfo_e_phnum'>{}</span> * \
                 <span title='{:#x}' class='number fileinfo_e_phentsize'>{}</span> @ \
                 <span title='{:#x}' class='number fileinfo_e_phoff'>{}</span>",
                ehdr.e_phnum(),
                ehdr.e_phnum(),
                ehdr.e_phentsize(),
                ehdr.e_phentsize(),
                ehdr.e_phoff(),
                ehdr.e_phoff()
            ),
        ));

        information.push((
            "sh",
            "Section headers",
            format!(
                "<span title='{:#x}' class='number fileinfo_e_shnum'>{}</span> * \
                 <span title='{:#x}' class='number fileinfo_e_shentsize'>{}</span> @ \
                 <span title='{:#x}' class='number fileinfo_e_shoff'>{}</span>",
                ehdr.e_shnum(),
                ehdr.e_shnum(),
                ehdr.e_shentsize(),
                ehdr.e_shentsize(),
                ehdr.e_shoff(),
                ehdr.e_shoff()
            ),
        ));

        if u32::from(ehdr.e_flags()) != 0 {
            information.push(("e_flags", "Flags", format!("{:#x}", ehdr.e_flags())));
        }
    }

    fn add_ehdr_ranges(ehdr: &EhdrT, ranges: &mut Ranges);

    fn parse_phdrs(
        buf: &[u8],
        endianness: u8,
        ehdr: &EhdrT,
        elf: &mut ParsedElf,
    ) -> Result<(), String> {
        let mut start = read_field!(ehdr, e_phoff)?;
        let phsize = size_of::<PhdrT>();

        for i in 0..ehdr.e_phnum().into() {
            let phdr = PhdrT::from_bytes(&buf[start..start + phsize], endianness)?;
            let parsed = Self::parse_phdr(&phdr)?;
            let ranges = &mut elf.ranges;

            if parsed.file_offset != 0 && parsed.file_size != 0 {
                ranges.add_range(parsed.file_offset, parsed.file_size, RangeType::Segment(i));
            }

            ranges.add_range(start, phsize, RangeType::ProgramHeader(i as u32));

            Self::add_phdr_ranges(start, ranges);

            elf.phdrs.push(parsed);

            start += phsize;
        }

        Ok(())
    }

    fn parse_phdr(phdr: &PhdrT) -> Result<ParsedPhdr, String> {
        let file_offset = read_field!(phdr, p_offset)?;
        let file_size = read_field!(phdr, p_filesz)?;
        let vaddr = read_field!(phdr, p_vaddr)?;
        let memsz = read_field!(phdr, p_memsz)?;
        let alignment = read_field!(phdr, p_align)?;

        Ok(ParsedPhdr {
            ptype: phdr.p_type().into(),
            flags: pflags_to_string(phdr.p_flags().into()),
            file_offset,
            file_size,
            vaddr,
            memsz,
            alignment,
        })
    }

    fn add_phdr_ranges(start: usize, ranges: &mut Ranges);

    fn parse_shdrs(
        buf: &[u8],
        endianness: u8,
        ehdr: &EhdrT,
        elf: &mut ParsedElf,
    ) -> Result<(), String> {
        let mut start = read_field!(ehdr, e_shoff)?;
        let shsize = size_of::<ShdrT>();

        for i in 0..ehdr.e_shnum().into() {
            let shdr = ShdrT::from_bytes(&buf[start..start + shsize], endianness)?;
            let parsed = Self::parse_shdr(buf, endianness, &shdr)?;
            let ranges = &mut elf.ranges;

            if parsed.file_offset != 0 && parsed.size != 0 && parsed.shtype != SHT_NOBITS {
                ranges.add_range(parsed.file_offset, parsed.size, RangeType::Section(i));
            }

            ranges.add_range(start, shsize, RangeType::SectionHeader(i as u32));

            Self::add_shdr_ranges(start, ranges);

            elf.shdrs.push(parsed);

            start += shsize;
        }

        Ok(())
    }

    fn parse_shdr(_buf: &[u8], _endianness: u8, shdr: &ShdrT) -> Result<ParsedShdr, String> {
        let name = read_field!(shdr, sh_name)?;
        let addr = read_field!(shdr, sh_addr)?;
        let file_offset = read_field!(shdr, sh_offset)?;
        let size = read_field!(shdr, sh_size)?;
        let link = read_field!(shdr, sh_link)?;
        let info = read_field!(shdr, sh_info)?;
        let addralign = read_field!(shdr, sh_addralign)?;
        let entsize = read_field!(shdr, sh_entsize)?;

        Ok(ParsedShdr {
            name,
            shtype: shdr.sh_type().into(),
            flags: shdr.sh_flags().into(),
            addr,
            file_offset,
            size,
            link,
            info,
            addralign,
            entsize,
        })
    }

    fn add_shdr_ranges(start: usize, ranges: &mut Ranges);
}
