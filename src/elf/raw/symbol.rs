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
pub struct Elf32_Sym {
    pub st_name: u32,
    pub st_value: u32,
    pub st_size: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
}

impl Elf32_Sym {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

impl Elf64_Sym {
    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        read_struct(bytes)
    }
}

pub trait RawSymbol {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError>
    where
        Self: Sized;

    fn name_offset(&self) -> u32;
    fn value(&self) -> u64;
    fn size(&self) -> u64;
    fn info(&self) -> u8;
    fn section_index(&self) -> u32;
}

impl RawSymbol for Elf32_Sym {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf32_Sym::from_bytes(bytes)
    }

    fn name_offset(&self) -> u32 {
        self.st_name
    }

    fn value(&self) -> u64 {
        self.st_value as u64
    }

    fn size(&self) -> u64 {
        self.st_size as u64
    }

    fn info(&self) -> u8 {
        self.st_info
    }

    fn section_index(&self) -> u32 {
        self.st_shndx as u32
    }
}

#[repr(C)]
pub struct Elf64_Sym {
    pub st_name: u32,
    pub st_info: u8,
    pub st_other: u8,
    pub st_shndx: u16,
    pub st_value: u64,
    pub st_size: u64,
}

impl RawSymbol for Elf64_Sym {
    fn from_bytes(bytes: &[u8]) -> Result<Self, ElfError> {
        Elf64_Sym::from_bytes(bytes)
    }

    fn name_offset(&self) -> u32 {
        self.st_name
    }

    fn value(&self) -> u64 {
        self.st_value
    }

    fn size(&self) -> u64 {
        self.st_size
    }

    fn info(&self) -> u8 {
        self.st_info
    }

    fn section_index(&self) -> u32 {
        self.st_shndx as u32
    }
}