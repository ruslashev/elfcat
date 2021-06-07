use super::defs::ELF_DATA2LSB;
use super::parser::*;

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
