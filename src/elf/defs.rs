#![cfg_attr(debug_assertions, allow(dead_code))]

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
