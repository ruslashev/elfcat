#![allow(dead_code)]

pub const ELF_EI_MAG0: u8 = 0;
pub const ELF_EI_MAG1: u8 = 1;
pub const ELF_EI_MAG2: u8 = 2;
pub const ELF_EI_MAG3: u8 = 3;
pub const ELF_EI_CLASS: u8 = 4;
pub const ELF_EI_DATA: u8 = 5;
pub const ELF_EI_VERSION: u8 = 6;
pub const ELF_EI_OSABI: u8 = 7;
pub const ELF_EI_ABIVERSION: u8 = 8;
pub const ELF_EI_PAD: u8 = 9;
pub const ELF_EI_NIDENT: u8 = 16;

pub const ELF_CLASS32: u8 = 1;
pub const ELF_CLASS64: u8 = 2;

pub const ELF_DATA2LSB: u8 = 1;
pub const ELF_DATA2MSB: u8 = 2;

pub const ELF_EV_CURRENT: u8 = 1;

pub const ELF_OSABI_SYSV: u8 = 0;
pub const ELF_OSABI_HPUX: u8 = 1;

pub const ELF_ET_NONE: u16 = 0;
pub const ELF_ET_REL: u16 = 1;
pub const ELF_ET_EXEC: u16 = 2;
pub const ELF_ET_DYN: u16 = 3;
pub const ELF_ET_CORE: u16 = 4;
pub const ELF_ET_LOOS: u16 = 0xfe00;
pub const ELF_ET_HIOS: u16 = 0xfeff;
pub const ELF_ET_LOPROC: u16 = 0xff00;
pub const ELF_ET_HIPROC: u16 = 0xffff;

pub const PT_NULL: u32 = 0;
pub const PT_LOAD: u32 = 1;
pub const PT_DYNAMIC: u32 = 2;
pub const PT_INTERP: u32 = 3;
pub const PT_NOTE: u32 = 4;
pub const PT_SHLIB: u32 = 5;
pub const PT_PHDR: u32 = 6;
pub const PT_TLS: u32 = 7;
pub const PT_LOOS: u32 = 0x6000_0000;
pub const PT_GNU_EH_FRAME: u32 = 0x6474_e550;
pub const PT_GNU_STACK: u32 = 0x6474_e551;
pub const PT_GNU_RELRO: u32 = 0x6474_e552;
pub const PT_HIOS: u32 = 0x6fff_ffff;
pub const PT_LOPROC: u32 = 0x7000_0000;
pub const PT_HIPROC: u32 = 0x7fff_ffff;

pub const PF_X: u32 = 0b001;
pub const PF_W: u32 = 0b010;
pub const PF_R: u32 = 0b100;
pub const PF_MASKOS: u32 = 0x00ff_0000;
pub const PF_MASKPROC: u32 = 0xff00_0000;

pub const NT_GNU_BUILD_ID: u32 = 0x3;

pub const SHN_UNDEF: u16 = 0;

pub const SHT_NULL: u32 = 0;
pub const SHT_PROGBITS: u32 = 1;
pub const SHT_SYMTAB: u32 = 2;
pub const SHT_STRTAB: u32 = 3;
pub const SHT_RELA: u32 = 4;
pub const SHT_HASH: u32 = 5;
pub const SHT_DYNAMIC: u32 = 6;
pub const SHT_NOTE: u32 = 7;
pub const SHT_NOBITS: u32 = 8;
pub const SHT_REL: u32 = 9;
pub const SHT_SHLIB: u32 = 10;
pub const SHT_DYNSYM: u32 = 11;
pub const SHT_INIT_ARRAY: u32 = 14;
pub const SHT_FINI_ARRAY: u32 = 15;
pub const SHT_LOOS: u32 = 0x6000_0000;
pub const SHT_GNU_HASH: u32 = 0x6fff_fff6;
pub const SHT_VER_NEED: u32 = 0x6fff_fffe;
pub const SHT_HIOS: u32 = 0x6fff_ffff;
pub const SHT_LOPROC: u32 = 0x7000_0000;
pub const SHT_HIPROC: u32 = 0x7fff_ffff;

pub const SHF_WRITE: u64 = 0b001;
pub const SHF_ALLOC: u64 = 0b010;
pub const SHF_EXECINSTR: u64 = 0b100;
pub const SHF_MASKOS: u64 = 0x0f00_0000;
pub const SHF_MASKPROC: u64 = 0xf000_0000;

pub fn type_to_string(e_type: u16) -> String {
    match e_type {
        ELF_ET_NONE => String::from("None (NONE)"),
        ELF_ET_REL => String::from("Relocatable object file (REL)"),
        ELF_ET_EXEC => String::from("Executable file (EXEC)"),
        ELF_ET_DYN => String::from("Shared object file (DYN)"),
        ELF_ET_CORE => String::from("Core file (CORE)"),
        ELF_ET_LOOS | ELF_ET_HIOS => String::from("Environment-specific use"),
        ELF_ET_LOPROC | ELF_ET_HIPROC => String::from("Processor-specific use"),
        x => format!("Unknown {}", x),
    }
}

pub fn abi_to_string(abi: u8) -> String {
    match abi {
        ELF_OSABI_SYSV => String::from("SysV"),
        ELF_OSABI_HPUX => String::from("HP-UX"),
        2 => String::from("NetBSD"),
        3 => String::from("Linux"),
        4 => String::from("Hurd"),
        6 => String::from("Solaris"),
        7 => String::from("AIX"),
        8 => String::from("IRIX"),
        9 => String::from("FreeBSD"),
        10 => String::from("Tru64"),
        11 => String::from("Modesto"),
        12 => String::from("OpenBSD"),
        13 => String::from("OpenVMS"),
        255 => String::from("Standalone"),
        x => format!("Unknown: {}", x),
    }
}

pub fn machine_to_string(e_machine: u16) -> String {
    match e_machine {
        0 => String::from("None"),
        1 => String::from("AT&T WE 32100"),
        2 => String::from("SPARC"),
        3 => String::from("x86"),
        4 => String::from("Motorolla 68000"),
        5 => String::from("Motorolla 88000"),
        6 => String::from("Intel MCU"),
        7 => String::from("Intel 80860"),
        8 => String::from("MIPS"),
        9 => String::from("IBM System/370"),
        10 => String::from("MIPS RS3000 little-endian"),
        14 => String::from("HP PA-RISC"),
        19 => String::from("Intel 80960"),
        20 => String::from("PowerPC"),
        21 => String::from("PowerPC 64-bit"),
        22 => String::from("S390"),
        40 => String::from("ARM Aarch32"),
        50 => String::from("Itanium IA-64"),
        62 => String::from("x86-64"),
        183 => String::from("ARM Aarch64"),
        190 => String::from("CUDA"),
        224 => String::from("AMDGPU"),
        243 => String::from("RISC-V"),
        x => format!("Unknown: {}", x),
    }
}

pub fn ptype_to_string(ptype: u32) -> String {
    match ptype {
        PT_NULL => String::from("NULL"),
        PT_LOAD => String::from("LOAD"),
        PT_DYNAMIC => String::from("DYNAMIC"),
        PT_INTERP => String::from("INTERP"),
        PT_NOTE => String::from("NOTE"),
        PT_SHLIB => String::from("SHLIB"),
        PT_PHDR => String::from("PHDR"),
        PT_TLS => String::from("TLS"),
        PT_LOOS => String::from("LOOS"),
        PT_GNU_EH_FRAME => String::from("GNU_EH_FRAME (OS-specific)"),
        PT_GNU_STACK => String::from("GNU_STACK (OS-specific)"),
        PT_GNU_RELRO => String::from("GNU_RELRO (OS-specific)"),
        PT_HIOS => String::from("HIOS"),
        PT_LOPROC => String::from("LOPROC"),
        PT_HIPROC => String::from("HIPROC"),
        x => format!("Unknown: {}", x),
    }
}

pub fn pflags_to_string(flags: u32) -> String {
    let mut s = String::new();

    if flags & PF_R != 0 {
        s.push('R');
    }

    if flags & PF_W != 0 {
        s.push('W');
    }

    if flags & PF_X != 0 {
        s.push('X');
    }

    s
}

pub fn shtype_to_string(shtype: u32) -> String {
    match shtype {
        SHT_NULL => String::from("NULL"),
        SHT_PROGBITS => String::from("PROGBITS"),
        SHT_SYMTAB => String::from("SYMTAB"),
        SHT_STRTAB => String::from("STRTAB"),
        SHT_RELA => String::from("RELA"),
        SHT_HASH => String::from("HASH"),
        SHT_DYNAMIC => String::from("DYNAMIC"),
        SHT_NOTE => String::from("NOTE"),
        SHT_NOBITS => String::from("NOBITS"),
        SHT_REL => String::from("REL"),
        SHT_SHLIB => String::from("SHLIB"),
        SHT_INIT_ARRAY => String::from("INIT_ARRAY"),
        SHT_FINI_ARRAY => String::from("FINI_ARRAY"),
        SHT_DYNSYM => String::from("DYNSYM"),
        SHT_LOOS => String::from("LOOS"),
        SHT_GNU_HASH => String::from("GNU_HASH (OS-specific)"),
        SHT_VER_NEED => String::from("VER_NEED (OS-specific)"),
        SHT_HIOS => String::from("HIOS"),
        SHT_LOPROC => String::from("LOPROC"),
        SHT_HIPROC => String::from("HIPROC"),
        x => format!("Unknown: {}", x),
    }
}

pub fn shflags_to_string(flags: u64) -> String {
    let mut s = String::new();

    if flags & SHF_WRITE != 0 {
        s.push('W');
    }

    if flags & SHF_ALLOC != 0 {
        s.push('A');
    }

    if flags & SHF_EXECINSTR != 0 {
        s.push('X');
    }

    if s.is_empty() {
        s.push('0');
    }

    s
}
