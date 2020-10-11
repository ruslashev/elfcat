#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]

use super::parser::*;

type Elf32Addr = u32;
type Elf32Half = u16;
type Elf32Off = u32;
type Elf32Sword = i32;
type Elf32Word = u32;

pub fn parse(buf: &Vec<u8>, ident: &ParsedIdent, elf: &mut ParsedElf) -> Result<(), String> {
    unimplemented!();
}
