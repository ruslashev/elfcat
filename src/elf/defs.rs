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
pub const ELF_OSABI_NETBSD: u8 = 2;
pub const ELF_OSABI_LINUX: u8 = 3;
pub const ELF_OSABI_HURD: u8 = 4;
pub const ELF_OSABI_SOLARIS: u8 = 6;
pub const ELF_OSABI_AIX: u8 = 7;
pub const ELF_OSABI_IRIX: u8 = 8;
pub const ELF_OSABI_FREEBSD: u8 = 9;
pub const ELF_OSABI_TRU64: u8 = 10;
pub const ELF_OSABI_MODESTO: u8 = 11;
pub const ELF_OSABI_OPENBSD: u8 = 12;
pub const ELF_OSABI_OPENVMS: u8 = 13;
pub const ELF_OSABI_STANDALONE: u8 = 255;

pub const ELF_ET_NONE: u16 = 0;
pub const ELF_ET_REL: u16 = 1;
pub const ELF_ET_EXEC: u16 = 2;
pub const ELF_ET_DYN: u16 = 3;
pub const ELF_ET_CORE: u16 = 4;
pub const ELF_ET_LOOS: u16 = 0xfe00;
pub const ELF_ET_HIOS: u16 = 0xfeff;
pub const ELF_ET_LOPROC: u16 = 0xff00;
pub const ELF_ET_HIPROC: u16 = 0xffff;

pub const ELF_EM_NONE: u16 = 0;
pub const ELF_EM_32K: u16 = 1;
pub const ELF_EM_SPARC: u16 = 2;
pub const ELF_EM_X86: u16 = 3;
pub const ELF_EM_M68K: u16 = 4;
pub const ELF_EM_M88K: u16 = 5;
pub const ELF_EM_INTEL_MCU: u16 = 6;
pub const ELF_EM_80860: u16 = 7;
pub const ELF_EM_MIPS: u16 = 8;
pub const ELF_EM_S370: u16 = 9;
pub const ELF_EM_MIPS_RS3K_LE: u16 = 10;
pub const ELF_EM_PA_RISC: u16 = 14;
pub const ELF_EM_80960: u16 = 19;
pub const ELF_EM_PPC: u16 = 20;
pub const ELF_EM_PPC64: u16 = 21;
pub const ELF_EM_S390: u16 = 22;
pub const ELF_EM_ARM32: u16 = 40;
pub const ELF_EM_IA64: u16 = 50;
pub const ELF_EM_AMD64: u16 = 62;
pub const ELF_EM_ARM64: u16 = 183;
pub const ELF_EM_CUDA: u16 = 190;
pub const ELF_EM_AMDGPU: u16 = 224;
pub const ELF_EM_RISCV: u16 = 243;

// This is messy.
pub fn abi_to_string(abi: u8) -> String {
    match abi {
        ELF_OSABI_SYSV => String::from("SysV"),
        ELF_OSABI_HPUX => String::from("HP-UX"),
        ELF_OSABI_NETBSD => String::from("NetBSD"),
        ELF_OSABI_LINUX => String::from("Linux"),
        ELF_OSABI_HURD => String::from("Hurd"),
        ELF_OSABI_SOLARIS => String::from("Solaris"),
        ELF_OSABI_AIX => String::from("AIX"),
        ELF_OSABI_IRIX => String::from("IRIX"),
        ELF_OSABI_FREEBSD => String::from("FreeBSD"),
        ELF_OSABI_TRU64 => String::from("Tru64"),
        ELF_OSABI_MODESTO => String::from("Modesto"),
        ELF_OSABI_OPENBSD => String::from("OpenBSD"),
        ELF_OSABI_OPENVMS => String::from("OpenVMS"),
        ELF_OSABI_STANDALONE => String::from("Standalone"),
        x => format!("Unknown: {}", x),
    }
}

pub fn machine_to_string(machine: u16) -> String {
    match machine {
        ELF_EM_NONE => String::from("None"),
        ELF_EM_32K => String::from("AT&T WE 32100"),
        ELF_EM_SPARC => String::from("SPARC"),
        ELF_EM_X86 => String::from("x86"),
        ELF_EM_M68K => String::from("Motorolla 68000"),
        ELF_EM_M88K => String::from("Motorolla 88000"),
        ELF_EM_INTEL_MCU => String::from("Intel MCU"),
        ELF_EM_80860 => String::from("Intel 80860"),
        ELF_EM_MIPS => String::from("MIPS"),
        ELF_EM_S370 => String::from("IBM System/370"),
        ELF_EM_MIPS_RS3K_LE => String::from("MIPS RS3000 little-endian"),
        ELF_EM_PA_RISC => String::from("HP PA-RISC"),
        ELF_EM_80960 => String::from("Intel 80960"),
        ELF_EM_PPC => String::from("PowerPC"),
        ELF_EM_PPC64 => String::from("PowerPC 64-bit"),
        ELF_EM_S390 => String::from("S390"),
        ELF_EM_ARM32 => String::from("ARM Aarch32"),
        ELF_EM_IA64 => String::from("Itanium IA-64"),
        ELF_EM_AMD64 => String::from("x86-64"),
        ELF_EM_ARM64 => String::from("ARM Aarch64"),
        ELF_EM_CUDA => String::from("CUDA"),
        ELF_EM_AMDGPU => String::from("AMDGPU"),
        ELF_EM_RISCV => String::from("RISC-V"),
        x => format!("Unknown: {}", x),
    }
}
