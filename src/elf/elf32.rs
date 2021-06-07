use super::elfxx::*;
use super::parser::*;
use std::convert::TryInto;

type Elf32Addr = u32;
type Elf32Half = u16;
type Elf32Off = u32;
type Elf32Word = u32;

pub struct Elf32Ehdr {
    e_ident: [u8; 16],
    e_type: Elf32Half,
    e_machine: Elf32Half,
    e_version: Elf32Word,
    e_entry: Elf32Addr,
    e_phoff: Elf32Off,
    e_shoff: Elf32Off,
    e_flags: Elf32Word,
    e_ehsize: Elf32Half,
    e_phentsize: Elf32Half,
    e_phnum: Elf32Half,
    e_shentsize: Elf32Half,
    e_shnum: Elf32Half,
    e_shstrndx: Elf32Half,
}

pub struct Elf32Phdr {
    p_type: Elf32Word,
    p_offset: Elf32Off,
    p_vaddr: Elf32Addr,
    p_paddr: Elf32Addr,
    p_filesz: Elf32Word,
    p_memsz: Elf32Word,
    p_flags: Elf32Word,
    p_align: Elf32Word,
}

pub struct Elf32Shdr {
    sh_name: Elf32Word,
    sh_type: Elf32Word,
    sh_flags: Elf32Word,
    sh_addr: Elf32Addr,
    sh_offset: Elf32Off,
    sh_size: Elf32Word,
    sh_link: Elf32Word,
    sh_info: Elf32Word,
    sh_addralign: Elf32Word,
    sh_entsize: Elf32Word,
}

pub struct Elf32;

#[rustfmt::skip]
impl ElfHeader for Elf32Ehdr {
    fn describe() -> String {
        String::from("file header")
    }
    fn from_le_bytes(buf: &[u8]) -> Result<Elf32Ehdr, ReadErr> {
        Ok(Elf32Ehdr {
            e_ident:     buf[0..16].try_into()?,
            e_type:      Elf32Half::from_le_bytes(buf[16..18].try_into()?),
            e_machine:   Elf32Half::from_le_bytes(buf[18..20].try_into()?),
            e_version:   Elf32Word::from_le_bytes(buf[20..24].try_into()?),
            e_entry:     Elf32Addr::from_le_bytes(buf[24..28].try_into()?),
            e_phoff:     Elf32Off:: from_le_bytes(buf[28..32].try_into()?),
            e_shoff:     Elf32Off:: from_le_bytes(buf[32..36].try_into()?),
            e_flags:     Elf32Word::from_le_bytes(buf[36..40].try_into()?),
            e_ehsize:    Elf32Half::from_le_bytes(buf[40..42].try_into()?),
            e_phentsize: Elf32Half::from_le_bytes(buf[42..44].try_into()?),
            e_phnum:     Elf32Half::from_le_bytes(buf[44..46].try_into()?),
            e_shentsize: Elf32Half::from_le_bytes(buf[46..48].try_into()?),
            e_shnum:     Elf32Half::from_le_bytes(buf[48..50].try_into()?),
            e_shstrndx:  Elf32Half::from_le_bytes(buf[50..52].try_into()?),
        })
    }
    fn from_be_bytes(buf: &[u8]) -> Result<Elf32Ehdr, ReadErr> {
        Ok(Elf32Ehdr {
            e_ident:     buf[0..16].try_into()?,
            e_type:      Elf32Half::from_be_bytes(buf[16..18].try_into()?),
            e_machine:   Elf32Half::from_be_bytes(buf[18..20].try_into()?),
            e_version:   Elf32Word::from_be_bytes(buf[20..24].try_into()?),
            e_entry:     Elf32Addr::from_be_bytes(buf[24..28].try_into()?),
            e_phoff:     Elf32Off:: from_be_bytes(buf[28..32].try_into()?),
            e_shoff:     Elf32Off:: from_be_bytes(buf[32..36].try_into()?),
            e_flags:     Elf32Word::from_be_bytes(buf[36..40].try_into()?),
            e_ehsize:    Elf32Half::from_be_bytes(buf[40..42].try_into()?),
            e_phentsize: Elf32Half::from_be_bytes(buf[42..44].try_into()?),
            e_phnum:     Elf32Half::from_be_bytes(buf[44..46].try_into()?),
            e_shentsize: Elf32Half::from_be_bytes(buf[46..48].try_into()?),
            e_shnum:     Elf32Half::from_be_bytes(buf[48..50].try_into()?),
            e_shstrndx:  Elf32Half::from_be_bytes(buf[50..52].try_into()?),
        })
    }
}

#[rustfmt::skip]
impl ElfHeader for Elf32Phdr {
    fn describe() -> String {
        String::from("program header")
    }
    fn from_le_bytes(buf: &[u8]) -> Result<Elf32Phdr, ReadErr> {
        Ok(Elf32Phdr {
            p_type:   Elf32Word::from_le_bytes(buf[ 0.. 4].try_into()?),
            p_offset: Elf32Off:: from_le_bytes(buf[ 4.. 8].try_into()?),
            p_vaddr:  Elf32Addr::from_le_bytes(buf[ 8..12].try_into()?),
            p_paddr:  Elf32Addr::from_le_bytes(buf[12..16].try_into()?),
            p_filesz: Elf32Word::from_le_bytes(buf[16..20].try_into()?),
            p_memsz:  Elf32Word::from_le_bytes(buf[20..24].try_into()?),
            p_flags:  Elf32Word::from_le_bytes(buf[24..28].try_into()?),
            p_align:  Elf32Word::from_le_bytes(buf[28..32].try_into()?),
        })
    }
    fn from_be_bytes(buf: &[u8]) -> Result<Elf32Phdr, ReadErr> {
        Ok(Elf32Phdr {
            p_type:   Elf32Word::from_be_bytes(buf[ 0.. 4].try_into()?),
            p_offset: Elf32Off:: from_be_bytes(buf[ 4.. 8].try_into()?),
            p_vaddr:  Elf32Addr::from_be_bytes(buf[ 8..12].try_into()?),
            p_paddr:  Elf32Addr::from_be_bytes(buf[12..16].try_into()?),
            p_filesz: Elf32Word::from_be_bytes(buf[16..20].try_into()?),
            p_memsz:  Elf32Word::from_be_bytes(buf[20..24].try_into()?),
            p_flags:  Elf32Word::from_be_bytes(buf[24..28].try_into()?),
            p_align:  Elf32Word::from_be_bytes(buf[28..32].try_into()?),
        })
    }
}

#[rustfmt::skip]
impl ElfHeader for Elf32Shdr {
    fn describe() -> String {
        String::from("section header")
    }
    fn from_le_bytes(buf: &[u8]) -> Result<Elf32Shdr, ReadErr> {
        Ok(Elf32Shdr {
            sh_name:      Elf32Word::from_le_bytes(buf[ 0.. 4].try_into()?),
            sh_type:      Elf32Word::from_le_bytes(buf[ 4.. 8].try_into()?),
            sh_flags:     Elf32Word::from_le_bytes(buf[ 8..12].try_into()?),
            sh_addr:      Elf32Addr::from_le_bytes(buf[12..16].try_into()?),
            sh_offset:    Elf32Off:: from_le_bytes(buf[16..20].try_into()?),
            sh_size:      Elf32Word::from_le_bytes(buf[20..24].try_into()?),
            sh_link:      Elf32Word::from_le_bytes(buf[24..28].try_into()?),
            sh_info:      Elf32Word::from_le_bytes(buf[28..32].try_into()?),
            sh_addralign: Elf32Word::from_le_bytes(buf[32..36].try_into()?),
            sh_entsize:   Elf32Word::from_le_bytes(buf[36..40].try_into()?),
        })
    }
    fn from_be_bytes(buf: &[u8]) -> Result<Elf32Shdr, ReadErr> {
        Ok(Elf32Shdr {
            sh_name:      Elf32Word::from_be_bytes(buf[ 0.. 4].try_into()?),
            sh_type:      Elf32Word::from_be_bytes(buf[ 4.. 8].try_into()?),
            sh_flags:     Elf32Word::from_be_bytes(buf[ 8..12].try_into()?),
            sh_addr:      Elf32Addr::from_be_bytes(buf[12..16].try_into()?),
            sh_offset:    Elf32Off:: from_be_bytes(buf[16..20].try_into()?),
            sh_size:      Elf32Word::from_be_bytes(buf[20..24].try_into()?),
            sh_link:      Elf32Word::from_be_bytes(buf[24..28].try_into()?),
            sh_info:      Elf32Word::from_be_bytes(buf[28..32].try_into()?),
            sh_addralign: Elf32Word::from_be_bytes(buf[32..36].try_into()?),
            sh_entsize:   Elf32Word::from_be_bytes(buf[36..40].try_into()?),
        })
    }
}

#[rustfmt::skip]
impl ElfXXEhdr<Elf32Addr, Elf32Half, Elf32Word, Elf32Off> for Elf32Ehdr {
    fn e_ident(&self)     -> [u8; 16]  { self.e_ident     }
    fn e_type(&self)      -> Elf32Half { self.e_type      }
    fn e_machine(&self)   -> Elf32Half { self.e_machine   }
    fn e_version(&self)   -> Elf32Word { self.e_version   }
    fn e_entry(&self)     -> Elf32Addr { self.e_entry     }
    fn e_phoff(&self)     -> Elf32Off  { self.e_phoff     }
    fn e_shoff(&self)     -> Elf32Off  { self.e_shoff     }
    fn e_flags(&self)     -> Elf32Word { self.e_flags     }
    fn e_ehsize(&self)    -> Elf32Half { self.e_ehsize    }
    fn e_phentsize(&self) -> Elf32Half { self.e_phentsize }
    fn e_phnum(&self)     -> Elf32Half { self.e_phnum     }
    fn e_shentsize(&self) -> Elf32Half { self.e_shentsize }
    fn e_shnum(&self)     -> Elf32Half { self.e_shnum     }
    fn e_shstrndx(&self)  -> Elf32Half { self.e_shstrndx  }
}

#[rustfmt::skip]
impl ElfXXPhdr<Elf32Addr, Elf32Word, Elf32Off, Elf32Word> for Elf32Phdr {
    fn p_type(&self)   -> Elf32Word  { self.p_type   }
    fn p_flags(&self)  -> Elf32Word  { self.p_flags  }
    fn p_offset(&self) -> Elf32Off   { self.p_offset }
    fn p_vaddr(&self)  -> Elf32Addr  { self.p_vaddr  }
    fn p_paddr(&self)  -> Elf32Addr  { self.p_paddr  }
    fn p_filesz(&self) -> Elf32Word  { self.p_filesz }
    fn p_memsz(&self)  -> Elf32Word  { self.p_memsz  }
    fn p_align(&self)  -> Elf32Word  { self.p_align  }
}

#[rustfmt::skip]
impl ElfXXShdr<Elf32Addr, Elf32Word, Elf32Off, Elf32Word> for Elf32Shdr {
    fn sh_name(&self)      -> Elf32Word  { self.sh_name      }
    fn sh_type(&self)      -> Elf32Word  { self.sh_type      }
    fn sh_flags(&self)     -> Elf32Word  { self.sh_flags     }
    fn sh_addr(&self)      -> Elf32Addr  { self.sh_addr      }
    fn sh_offset(&self)    -> Elf32Off   { self.sh_offset    }
    fn sh_size(&self)      -> Elf32Word  { self.sh_size      }
    fn sh_link(&self)      -> Elf32Word  { self.sh_link      }
    fn sh_info(&self)      -> Elf32Word  { self.sh_info      }
    fn sh_addralign(&self) -> Elf32Word  { self.sh_addralign }
    fn sh_entsize(&self)   -> Elf32Word  { self.sh_entsize   }
}

#[rustfmt::skip]
impl ElfXX<Elf32Ehdr, Elf32Phdr, Elf32Shdr, Elf32Addr, Elf32Half, Elf32Word, Elf32Off, Elf32Word>
    for Elf32
{
    fn add_ehdr_ranges(ehdr: &Elf32Ehdr, ranges: &mut Ranges) {
        ranges.add_range(0,  ehdr.e_ehsize as usize, RangeType::FileHeader);
        ranges.add_range(16, 2, RangeType::HeaderField("e_type"));
        ranges.add_range(18, 2, RangeType::HeaderField("e_machine"));
        ranges.add_range(20, 4, RangeType::HeaderField("e_version"));
        ranges.add_range(24, 4, RangeType::HeaderField("e_entry"));
        ranges.add_range(28, 4, RangeType::HeaderField("e_phoff"));
        ranges.add_range(32, 4, RangeType::HeaderField("e_shoff"));
        ranges.add_range(36, 4, RangeType::HeaderField("e_flags"));
        ranges.add_range(40, 2, RangeType::HeaderField("e_ehsize"));
        ranges.add_range(42, 2, RangeType::HeaderField("e_phentsize"));
        ranges.add_range(44, 2, RangeType::HeaderField("e_phnum"));
        ranges.add_range(46, 2, RangeType::HeaderField("e_shentsize"));
        ranges.add_range(48, 2, RangeType::HeaderField("e_shnum"));
        ranges.add_range(50, 2, RangeType::HeaderField("e_shstrndx"));
    }

    fn add_phdr_ranges(start: usize, ranges: &mut Ranges) {
        ranges.add_range(start +  0, 4, RangeType::PhdrField("p_type"));
        ranges.add_range(start +  4, 4, RangeType::PhdrField("p_offset"));
        ranges.add_range(start +  8, 4, RangeType::PhdrField("p_vaddr"));
        ranges.add_range(start + 12, 4, RangeType::PhdrField("p_paddr"));
        ranges.add_range(start + 16, 4, RangeType::PhdrField("p_filesz"));
        ranges.add_range(start + 20, 4, RangeType::PhdrField("p_memsz"));
        ranges.add_range(start + 24, 4, RangeType::PhdrField("p_flags"));
        ranges.add_range(start + 28, 4, RangeType::PhdrField("p_align"));
    }

    fn add_shdr_ranges(start: usize, ranges: &mut Ranges) {
        ranges.add_range(start +  0, 4, RangeType::ShdrField("sh_name"));
        ranges.add_range(start +  4, 4, RangeType::ShdrField("sh_type"));
        ranges.add_range(start +  8, 4, RangeType::ShdrField("sh_flags"));
        ranges.add_range(start + 12, 4, RangeType::ShdrField("sh_addr"));
        ranges.add_range(start + 16, 4, RangeType::ShdrField("sh_offset"));
        ranges.add_range(start + 20, 4, RangeType::ShdrField("sh_size"));
        ranges.add_range(start + 24, 4, RangeType::ShdrField("sh_link"));
        ranges.add_range(start + 28, 4, RangeType::ShdrField("sh_info"));
        ranges.add_range(start + 32, 4, RangeType::ShdrField("sh_addralign"));
        ranges.add_range(start + 36, 4, RangeType::ShdrField("sh_entsize"));
    }
}
