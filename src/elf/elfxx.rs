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
