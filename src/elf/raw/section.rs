use crate::elf::enums::{SectionFlags, SectionType};
use crate::elf::error::ElfError;

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
pub(crate) struct Elf32_Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u32,
    pub sh_addr: u32,
    pub sh_offset: u32,
    pub sh_size: u32,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u32,
    pub sh_entsize: u32,
}

#[repr(C)]
pub(crate) struct Elf64_Shdr {
    pub sh_name: u32,
    pub sh_type: u32,
    pub sh_flags: u64,
    pub sh_addr: u64,
    pub sh_offset: u64,
    pub sh_size: u64,
    pub sh_link: u32,
    pub sh_info: u32,
    pub sh_addralign: u64,
    pub sh_entsize: u64,
}

impl Elf32_Shdr {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

impl Elf64_Shdr {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

pub trait RawSectionHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError>
    where
        Self: Sized;

    fn name_offset(&self) -> u32;
    fn section_type(&self) -> SectionType;
    fn flags(&self) -> SectionFlags;
    fn virtual_address(&self) -> u64;
    fn file_offset(&self) -> u64;
    fn size(&self) -> u64;
    fn link(&self) -> u32;
    fn info(&self) -> u32;
    fn alignment(&self) -> u64;
    fn entry_size(&self) -> u64;
}

impl RawSectionHeader for Elf32_Shdr {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf32_Shdr::from_bytes(bytes)
    }

    fn name_offset(&self) -> u32 {
        self.sh_name
    }

    fn section_type(&self) -> SectionType {
        SectionType::from(self.sh_type)
    }

    fn flags(&self) -> SectionFlags {
        SectionFlags::from_bits_truncate(self.sh_flags as u64)
    }

    fn virtual_address(&self) -> u64 {
        self.sh_addr as u64
    }

    fn file_offset(&self) -> u64 {
        self.sh_offset as u64
    }

    fn size(&self) -> u64 {
        self.sh_size as u64
    }

    fn link(&self) -> u32 {
        self.sh_link
    }

    fn info(&self) -> u32 {
        self.sh_info
    }
    
    fn alignment(&self) -> u64 {
        self.sh_addralign as u64
    }

    fn entry_size(&self) -> u64 {
        self.sh_entsize as u64
    }
}

impl RawSectionHeader for Elf64_Shdr {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf64_Shdr::from_bytes(bytes)
    }

    fn name_offset(&self) -> u32 {
        self.sh_name
    }

    fn section_type(&self) -> SectionType {
        SectionType::from(self.sh_type)
    }

    fn flags(&self) -> SectionFlags {
        SectionFlags::from_bits_truncate(self.sh_flags as u64)
    }

    fn virtual_address(&self) -> u64 {
        self.sh_addr as u64
    }

    fn file_offset(&self) -> u64 {
        self.sh_offset as u64
    }

    fn size(&self) -> u64 {
        self.sh_size as u64
    }

    fn link(&self) -> u32 {
        self.sh_link
    }

    fn info(&self) -> u32 {
        self.sh_info
    }
    
    fn alignment(&self) -> u64 {
        self.sh_addralign as u64
    }

    fn entry_size(&self) -> u64 {
        self.sh_entsize
    }
}