use crate::elf::{ElfClass, Endianness, FileType, Machine};
use crate::elf::ElfError;

pub(crate) fn read_struct<T>(bytes: &[u8]) -> Result<T, ElfError> {
    if bytes.len() < std::mem::size_of::<T>() {
        return Err(ElfError::UnexpectedEOF);
    }

    unsafe {
        Ok(std::ptr::read_unaligned(
            bytes.as_ptr() as *const T
        ))
    }
}

#[repr(C)]
pub(crate) struct Elf32_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u32,
    pub e_phoff: u32,
    pub e_shoff: u32,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

#[repr(C)]
pub(crate) struct Elf64_Ehdr {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}


impl Elf32_Ehdr {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

impl Elf64_Ehdr {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

pub trait RawElfHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError>
    where
        Self: Sized;

    fn e_ident(&self) -> [u8; 16];
    fn e_type(&self) -> u16;
    fn e_machine(&self) -> u16;
    fn e_version(&self) -> u32;
    fn e_entry(&self) -> u64;
    fn e_phoff(&self) -> u64;
    fn e_shoff(&self) -> u64;
    fn e_flags(&self) -> u32;
    fn e_ehsize(&self) -> u16;
    fn e_phentsize(&self) -> u16;
    fn e_phnum(&self) -> u16;
    fn e_shentsize(&self) -> u16;
    fn e_shnum(&self) -> u16;
    fn e_shstrndx(&self) -> u32;

    fn magic(&self) -> [u8; 4] {
        self.e_ident()[0..4]
            .try_into()
            .unwrap()
    }

    fn class(&self) -> ElfClass {
        ElfClass::from(self.e_ident()[4])
    }
    
    fn endianness(&self) -> Endianness {
        Endianness::from(self.e_ident()[5])
    }

    fn file_type(&self) -> FileType {
        FileType::from(self.e_type())
    }
    
    
    fn machine(&self) -> Machine {
        Machine::from(self.e_machine())
    }
    
    fn version(&self) -> u32 {
        self.e_version()
    }

    fn entry(&self) -> u64 {
        self.e_entry()
    }

    fn program_header_offset(&self) -> u64 {
        self.e_phoff()
    }

    fn section_header_offset(&self) -> u64 {
        self.e_shoff()
    }

    fn header_size(&self) -> u16 {
        self.e_ehsize()
    }

    fn program_header_size(&self) -> u16 {
        self.e_phentsize()
    }

    fn program_header_count(&self) -> u16 {
        self.e_phnum()
    }

    fn section_header_size(&self) -> u16 {
        self.e_shentsize()
    }

    fn section_header_count(&self) -> u16 {
        self.e_shnum()
    }

    fn section_name_table_index(&self) -> u32 {
        self.e_shstrndx()
    }
}

impl RawElfHeader for Elf32_Ehdr {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf32_Ehdr::from_bytes(bytes)
    }

    fn e_ident(&self) -> [u8; 16] {
        self.e_ident
    }

    fn e_type(&self) -> u16 {
        self.e_type
    }

    fn e_machine(&self) -> u16 {
        self.e_machine
    }

    fn e_version(&self) -> u32 {
        self.e_version
    }

    fn e_entry(&self) -> u64 {
        self.e_entry as u64
    }
    
    fn e_phoff(&self) -> u64 {
        self.e_phoff as u64
    }

    fn e_shoff(&self) -> u64 {
        self.e_shoff as u64
    }

    fn e_flags(&self) -> u32 {
        self.e_flags
    }

    fn e_ehsize(&self) -> u16 {
        self.e_ehsize
    }

    fn e_phentsize(&self) -> u16 {
        self.e_phentsize
    }

    fn e_phnum(&self) -> u16 {
        self.e_phnum
    }

    fn e_shentsize(&self) -> u16 {
        self.e_shentsize
    }

    fn e_shnum(&self) -> u16 {
        self.e_shnum
    }

    fn e_shstrndx(&self) -> u32 {
        self.e_shstrndx as u32
    }
}

impl RawElfHeader for Elf64_Ehdr {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf64_Ehdr::from_bytes(bytes)
    }

    fn e_ident(&self) -> [u8; 16] {
        self.e_ident
    }

    fn e_type(&self) -> u16 {
        self.e_type
    }

    fn e_machine(&self) -> u16 {
        self.e_machine
    }

    fn e_version(&self) -> u32 {
        self.e_version
    }

    fn e_entry(&self) -> u64 {
        self.e_entry
    }
    
    fn e_phoff(&self) -> u64 {
        self.e_phoff
    }

    fn e_shoff(&self) -> u64 {
        self.e_shoff
    }

    fn e_flags(&self) -> u32 {
        self.e_flags
    }

    fn e_ehsize(&self) -> u16 {
        self.e_ehsize
    }

    fn e_phentsize(&self) -> u16 {
        self.e_phentsize
    }

    fn e_phnum(&self) -> u16 {
        self.e_phnum
    }

    fn e_shentsize(&self) -> u16 {
        self.e_shentsize
    }

    fn e_shnum(&self) -> u16 {
        self.e_shnum
    }

    fn e_shstrndx(&self) -> u32 {
        self.e_shstrndx as u32
    }
}