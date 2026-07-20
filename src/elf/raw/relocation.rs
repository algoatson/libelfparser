use crate::elf::enums::RelocationType;
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
pub struct Elf32_Rel {
    r_offset: u32,
    r_info: u32,
}

#[repr(C)]
pub struct Elf64_Rel {
    r_offset: u64,
    r_info: u64,
}

#[repr(C)]
pub struct Elf32_Rela {
    r_offset: u32,
    r_info: u32,
    r_addend: i32, 
}

#[repr(C)]
pub struct Elf64_Rela {
    r_offset: u64,
    r_info: u64,
    r_addend: i64,
}

impl Elf32_Rel {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

impl Elf64_Rel {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

impl Elf32_Rela {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

impl Elf64_Rela {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

pub trait RawRelocation {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError>
    where
        Self: Sized;

    fn offset(&self) -> u64;
    
    fn info(&self) -> u64;
    
    fn symbol_index(&self) -> u32 {
        (self.info() >> 32) as u32
    }

    fn relocation_type(&self) -> RelocationType {
        ((self.info() & 0xffffffff) as u32).into()
    }
 
    fn addend(&self) -> Option<i64>;
}

impl RawRelocation for Elf32_Rel {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf32_Rel::from_bytes(bytes)
    }

    fn offset(&self) -> u64 {
        self.r_offset as u64
    }

    fn info(&self) -> u64 {
        self.r_info as u64
    }

    fn addend(&self) -> Option<i64> {
        None
    }
}

impl RawRelocation for Elf64_Rel {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf64_Rel::from_bytes(bytes)
    }

    fn offset(&self) -> u64 {
        self.r_offset
    }

    fn info(&self) -> u64 {
        self.r_info
    }

    fn addend(&self) -> Option<i64> {
        None
    }
}

impl RawRelocation for Elf32_Rela {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf32_Rela::from_bytes(bytes)
    }

    fn offset(&self) -> u64 {
        self.r_offset as u64
    }

    fn info(&self) -> u64 {
        self.r_info as u64
    }

    fn addend(&self) -> Option<i64> {
        Some(self.r_addend as i64)
    }
}

impl RawRelocation for Elf64_Rela {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf64_Rela::from_bytes(bytes)
    }

    fn offset(&self) -> u64 {
        self.r_offset
    }

    fn info(&self) -> u64 {
        self.r_info
    }

    fn addend(&self) -> Option<i64> {
        Some(self.r_addend)
    }
}