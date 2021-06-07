use super::defs::ELF_DATA2LSB;
use super::parser::*;
use std::mem::size_of;

pub trait ElfHeader {
    fn from_le_bytes(buf: &[u8]) -> Result<Self, ReadErr>
    where
        Self: Sized;
    fn from_be_bytes(buf: &[u8]) -> Result<Self, ReadErr>
    where
        Self: Sized;
    fn describe() -> String;

    fn from_bytes(buf: &[u8], endianness: u8) -> Result<Self, String>
    where
        Self: Sized,
    {
        if endianness == ELF_DATA2LSB {
            Self::from_le_bytes(buf)
        } else {
            Self::from_be_bytes(buf)
        }
        .map_err(|a| format!("failed to read {}: {}", Self::describe(), a))
    }
}

// We do this because we can't access struct fields of a generic type
pub trait ElfXXEhdr<ElfXXAddr, ElfXXHalf, ElfXXWord, ElfXXOff> {
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

pub trait ElfXXPhdr<ElfXXAddr, ElfXXWord, ElfXXOff, ElfXXXword> {
    fn p_type(&self) -> ElfXXWord;
    fn p_flags(&self) -> ElfXXWord;
    fn p_offset(&self) -> ElfXXOff;
    fn p_vaddr(&self) -> ElfXXAddr;
    fn p_paddr(&self) -> ElfXXAddr;
    fn p_filesz(&self) -> ElfXXXword;
    fn p_memsz(&self) -> ElfXXXword;
    fn p_align(&self) -> ElfXXXword;
}

pub trait ElfXXShdr<ElfXXAddr, ElfXXWord, ElfXXOff, ElfXXXword> {
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

pub trait ElfXX<EhdrT,PhdrT,ShdrT,ElfXXAddr,ElfXXHalf,ElfXXWord,ElfXXOff,ElfXXXword>
{
    pub fn parse(buf: &[u8], ident: &ParsedIdent, elf: &mut ParsedElf) -> Result<(), String> {
        let ehdr_size = size_of::<Elf64Ehdr>();

        if buf.len() < ehdr_size {
            return Err(String::from("file is smaller than ELF file header"));
        }

        let ehdr = Elf64Ehdr::from_bytes(&buf[0..ehdr_size], ident.endianness)?;

        elf.shstrndx = ehdr.e_shstrndx;

        parse_ehdr(&ehdr, elf);

        parse_phdrs(buf, ident.endianness, &ehdr, elf)?;

        parse_shdrs(buf, ident.endianness, &ehdr, elf)?;

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

    fn add_ehdr_ranges(ehdr: &Elf64Ehdr, ranges: &mut Ranges);

    fn parse_phdrs(
        buf: &[u8],
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
                ranges.add_range(parsed.file_offset, parsed.file_size, RangeType::Segment(i));
            }

            ranges.add_range(start, phsize, RangeType::ProgramHeader(i as u32));

            add_phdr_ranges(start, ranges);

            elf.phdrs.push(parsed);

            start += phsize;
        }

        Ok(())
    }

    fn parse_phdr(phdr: &Elf64Phdr) -> ParsedPhdr {
        let ptype = phdr.p_type;
        let file_offset = phdr.p_offset as usize;
        let file_size = phdr.p_filesz as usize;

        ParsedPhdr {
            ptype,
            flags: pflags_to_string(phdr.p_flags),
            file_offset,
            file_size,
            vaddr: phdr.p_vaddr as usize,
            memsz: phdr.p_memsz as usize,
            alignment: phdr.p_align as usize,
        }
    }

    fn add_phdr_ranges(start: usize, ranges: &mut Ranges);

    fn parse_shdrs(
        buf: &[u8],
        endianness: u8,
        ehdr: &Elf64Ehdr,
        elf: &mut ParsedElf,
    ) -> Result<(), String> {
        let mut start = ehdr.e_shoff as usize;
        let shsize = size_of::<Elf64Shdr>();

        for i in 0..ehdr.e_shnum {
            let shdr = Elf64Shdr::from_bytes(&buf[start..start + shsize], endianness)?;
            let parsed = parse_shdr(buf, endianness, &shdr);
            let ranges = &mut elf.ranges;

            if parsed.file_offset != 0 && parsed.size != 0 {
                ranges.add_range(parsed.file_offset, parsed.size, RangeType::Section(i));
            }

            ranges.add_range(start, shsize, RangeType::SectionHeader(i as u32));

            add_shdr_ranges(start, ranges);

            elf.shdrs.push(parsed);

            start += shsize;
        }

        Ok(())
    }

    fn parse_shdr(_buf: &[u8], _endianness: u8, shdr: &Elf64Shdr) -> ParsedShdr {
        ParsedShdr {
            name: shdr.sh_name as usize,
            shtype: shdr.sh_type,
            flags: shdr.sh_flags,
            addr: shdr.sh_addr as usize,
            file_offset: shdr.sh_offset as usize,
            size: shdr.sh_size as usize,
            link: shdr.sh_link as usize,
            info: shdr.sh_info as usize,
            addralign: shdr.sh_addralign as usize,
            entsize: shdr.sh_entsize as usize,
        }
    }

    fn add_shdr_ranges(start: usize, ranges: &mut Ranges);
}
