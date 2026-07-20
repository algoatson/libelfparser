use crate::elf::enums::{SegmentFlags, SegmentType};
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
pub(crate) struct Elf32_Phdr {
    pub p_type: u32,
    pub p_offset: u32,
    pub p_vaddr: u32,
    pub p_paddr: u32,
    pub p_filesz: u32,
    pub p_memsz: u32,
    pub p_flags: u32,
    pub p_align: u32,
}

#[repr(C)]
pub(crate) struct Elf64_Phdr {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

impl Elf32_Phdr {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

impl Elf64_Phdr {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

pub trait RawProgramHeader {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError>
    where
        Self: Sized;

    // Required
    fn p_type(&self) -> u32;
    fn p_offset(&self) -> u64;
    fn p_vaddr(&self) -> u64;
    fn p_paddr(&self) -> u64;
    fn p_filesz(&self) -> u64;
    fn p_memsz(&self) -> u64;
    fn p_flags(&self) -> u32;
    fn p_align(&self) -> u64;

    // Default implementations
    fn segment_type(&self) -> SegmentType {
        SegmentType::from(self.p_type())
    }

    fn file_offset(&self) -> u64 {
        self.p_offset()
    }

    fn virtual_address(&self) -> u64 {
        self.p_vaddr()
    }

    fn physical_address(&self) -> u64 {
        self.p_paddr()
    }

    fn file_size(&self) -> u64 {
        self.p_filesz()
    }

    fn memory_size(&self) -> u64 {
        self.p_memsz()
    }

    fn flags(&self) -> SegmentFlags {
        SegmentFlags::from(self.p_flags())
    }

    fn alignment(&self) -> u64 {
        self.p_align()
    }
}

impl RawProgramHeader for Elf32_Phdr {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf32_Phdr::from_bytes(bytes)
    }

    fn p_type(&self) -> u32 { self.p_type }
    fn p_offset(&self) -> u64 { self.p_offset as u64 }
    fn p_vaddr(&self) -> u64 { self.p_vaddr as u64 }
    fn p_paddr(&self) -> u64 { self.p_paddr as u64 }
    fn p_filesz(&self) -> u64 { self.p_filesz as u64 }
    fn p_memsz(&self) -> u64 { self.p_memsz as u64 }
    fn p_flags(&self) -> u32 { self.p_flags }
    fn p_align(&self) -> u64 { self.p_align as u64 }
}

impl RawProgramHeader for Elf64_Phdr {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf64_Phdr::from_bytes(bytes)
    }

    fn p_type(&self) -> u32 { self.p_type }
    fn p_offset(&self) -> u64 { self.p_offset as u64 }
    fn p_vaddr(&self) -> u64 { self.p_vaddr as u64 }
    fn p_paddr(&self) -> u64 { self.p_paddr as u64 }
    fn p_filesz(&self) -> u64 { self.p_filesz as u64 }
    fn p_memsz(&self) -> u64 { self.p_memsz as u64 }
    fn p_flags(&self) -> u32 { self.p_flags }
    fn p_align(&self) -> u64 { self.p_align as u64 }
}