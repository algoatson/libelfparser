use bitflags::bitflags;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Endianness {
    Little,
    Big,
    Unknown(u8),
}

impl From<u8> for Endianness {
    fn from(value: u8) -> Self {
        match value {
            1 => Endianness::Little,
            2 => Endianness::Big,
            other => Endianness::Unknown(other),
        }
    }
}

impl From<Endianness> for u8 {
    fn from(value: Endianness) -> Self {
        match value {
            Endianness::Little => 1,
            Endianness::Big => 2,
            Endianness::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElfClass {
    None,
    Elf32,
    Elf64,
    Unknown(u8)
}

impl From<u8> for ElfClass {
    fn from(value: u8) -> Self {
        match value {
            0 => ElfClass::None,
            1 => ElfClass::Elf32,
            2 => ElfClass::Elf64,
            other => ElfClass::Unknown(other),
        }
    }
}

impl From<ElfClass> for u8 {
    fn from(value: ElfClass) -> u8 {
        match value {
            ElfClass::None => 0,
            ElfClass::Elf32 => 1,
            ElfClass::Elf64 => 2,
            ElfClass::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Machine {
    None,
    X86,
    X86_64,
    Arm,
    AArch64,
    Unknown(u16),
}

impl From<u16> for Machine {
    fn from(value: u16) -> Self {
        match value {
            0 => Machine::None,
            3 => Machine::X86,
            40 => Machine::Arm,
            62 => Machine::X86_64,
            183 => Machine::AArch64,
            other => Machine::Unknown(other),
        }
    }
}

impl From<Machine> for u16 {
    fn from(value: Machine) -> u16 {
        match value {
            Machine::None => 0,
            Machine::X86 => 3,
            Machine::Arm => 40,
            Machine::X86_64 => 62,
            Machine::AArch64 => 183,
            Machine::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    None,
    Relocatable,
    Executable,
    Dynamic,
    Core,
    Unknown(u16),
}

impl From<u16> for FileType {
    fn from(value: u16) -> Self {
        match value {
            0 => FileType::None,
            1 => FileType::Relocatable,
            2 => FileType::Executable,
            3 => FileType::Dynamic,
            4 => FileType::Core,
            other => FileType::Unknown(other),
        }
    }
}

impl From<FileType> for u16 {
    fn from(value: FileType) -> u16 {
        match value {
            FileType::None => 0,
            FileType::Relocatable => 1,
            FileType::Executable => 2,
            FileType::Dynamic => 3,
            FileType::Core => 4,
            FileType::Unknown(other) => other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SegmentType {
    Null,                    // PT_NULL        = 0
    Load,                    // PT_LOAD        = 1
    Dynamic,                 // PT_DYNAMIC     = 2
    Interpreter,             // PT_INTERP      = 3
    Note,                    // PT_NOTE        = 4
    Shlib,                   // PT_SHLIB       = 5
    ProgramHeader,           // PT_PHDR        = 6
    ThreadLocalStorage,      // PT_TLS         = 7

    OsSpecific(u32),         // PT_LOOS   - PT_HIOS
    ProcessorSpecific(u32),  // PT_LOPROC - PT_HIPROC
    Unknown(u32),
}

impl From<u32> for SegmentType {
    fn from(value: u32) -> SegmentType {
        match value {
            0 => SegmentType::Null,
            1 => SegmentType::Load,
            2 => SegmentType::Dynamic,
            3 => SegmentType::Interpreter,
            4 => SegmentType::Note,
            5 => SegmentType::Shlib,
            6 => SegmentType::ProgramHeader,
            7 => SegmentType::ThreadLocalStorage,

            0x60000000..=0x6fffffff => {
                SegmentType::OsSpecific(value)
            }

            0x70000000..=0x7fffffff => {
                SegmentType::ProcessorSpecific(value)
            }

            other => SegmentType::Unknown(other),
        }
    }
}

impl From<SegmentType> for u32 {
    fn from(value: SegmentType) -> u32 {
        match value {
            SegmentType::Null => 0,
            SegmentType::Load => 1,
            SegmentType::Dynamic => 2,
            SegmentType::Interpreter => 3,
            SegmentType::Note => 4,
            SegmentType::Shlib => 5,
            SegmentType::ProgramHeader => 6,
            SegmentType::ThreadLocalStorage => 7,

            SegmentType::OsSpecific(value) => value,
            SegmentType::ProcessorSpecific(value) => value,
            SegmentType::Unknown(other) => other,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct SegmentFlags(u32);

impl SegmentFlags {
    pub const EXECUTE: u32 = 0x1;
    pub const WRITE: u32 = 0x2;
    pub const READ: u32 = 0x4;

    pub fn readable(self) -> bool {
        self.0 & SegmentFlags::READ != 0
    }

    pub fn writable(self) -> bool {
        self.0 & SegmentFlags::WRITE != 0
    }

    pub fn executable(self) -> bool {
        self.0 & SegmentFlags::EXECUTE != 0
    }

    pub fn bits(self) -> u32 {
        self.0
    }
}

impl From<u32> for SegmentFlags {
    fn from(value: u32) -> Self {
        SegmentFlags(value)
    }
}

impl std::fmt::Display for SegmentFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let r = if self.readable() { 'R' } else { '-' };
        let w = if self.writable() { 'W' } else { '-' };
        let x = if self.executable() { 'X' } else { '-' };

        write!(f, "{}{}{}", r, w, x)
    }
}

impl std::fmt::Debug for SegmentFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SegmentFlags({:#x}, {})", self.0, self)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SectionType {
    Null,
    ProgramBits,
    SymbolTable,
    StringTable,
    Rela,
    Hash,
    Dynamic,
    Note,
    NoBits,
    Rel,
    ShLib,
    DynSym,
    Unknown(u32),
}

impl From<u32> for SectionType {
    fn from(value: u32) -> Self {
        match value {
            0 => SectionType::Null,
            1 => SectionType::ProgramBits,
            2 => SectionType::SymbolTable,
            3 => SectionType::StringTable,
            4 => SectionType::Rela,
            5 => SectionType::Hash,
            6 => SectionType::Dynamic,
            7 => SectionType::Note,
            8 => SectionType::NoBits,
            9 => SectionType::Rel,
            10 => SectionType::ShLib,
            11 => SectionType::DynSym,
            other => SectionType::Unknown(other),
        }
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct SectionFlags: u64 {
        const WRITE            = 0x1;
        const ALLOC            = 0x2;
        const EXECINSTR        = 0x4;
        const MERGE            = 0x10;
        const STRINGS          = 0x20;
        const INFO_LINK        = 0x40;
        const LINK_ORDER       = 0x80;
        const OS_NONCONFORMING = 0x100;
        const GROUP            = 0x200;
        const TLS              = 0x400;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SymbolBinding {
    Local,
    Global,
    Weak,

    // OS specific range
    OsSpecific(u8),

    // Processor specific range
    ProcessorSpecific(u8),

    Unknown(u8),
}

impl From<u8> for SymbolBinding {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Local,
            1 => Self::Global,
            2 => Self::Weak,

            10..=12 => Self::OsSpecific(value),
            13..=15 => Self::ProcessorSpecific(value),

            other => Self::Unknown(other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SymbolType {
    NoType,
    Object,
    Function,
    Section,
    File,
    Common,
    Tls,

    OsSpecific(u8),
    ProcessorSpecific(u8),

    Unknown(u8),
}

impl From<u8> for SymbolType {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::NoType,
            1 => Self::Object,
            2 => Self::Function,
            3 => Self::Section,
            4 => Self::File,
            5 => Self::Common,
            6 => Self::Tls,

            10..=12 => Self::OsSpecific(value),
            13..=15 => Self::ProcessorSpecific(value),

            other => Self::Unknown(other),
        }
    }
}