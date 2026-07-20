use crate::elf::enums::DynamicTag;
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
pub struct Elf32_Dyn {
    pub d_tag: i32,
    pub d_un: u32,
}

#[repr(C)]
pub struct Elf64_Dyn {
    pub d_tag: i64,
    pub d_un: u64,
}

impl Elf32_Dyn {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

impl Elf64_Dyn {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

pub trait RawDynamic {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError>
    where
        Self: Sized;

    fn tag(&self) -> DynamicTag;
    fn value(&self) -> u64;
}

impl RawDynamic for Elf32_Dyn {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf32_Dyn::from_bytes(bytes)
    }

    fn tag(&self) -> DynamicTag {
        DynamicTag::from(self.d_tag as i64)
    }

    fn value(&self) -> u64 {
        self.d_un as u64
    }
}

impl RawDynamic for Elf64_Dyn {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf64_Dyn::from_bytes(bytes)
    }

    fn tag(&self) -> DynamicTag {
        DynamicTag::from(self.d_tag)
    }

    fn value(&self) -> u64 {
        self.d_un
    }
}