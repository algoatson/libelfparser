use super::enums::{Endianness, Machine, ElfClass, FileType, SegmentType, SegmentFlags, SectionType, SectionFlags, SymbolBinding, SymbolType, RelocationType};
use super::error::ElfError;

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

// this one uses the old version for demonstration,
// im trying to fully understand whats going on
// so im leaving it here for now until i understand
// everything that is happening.
// impl Elf32_Phdr {
//     pub(crate) fn from_bytes(bytes: &[u8]) -> &Self {
//         assert!(bytes.len() >= std::mem::size_of::<Self>());

//         unsafe {
//             &*(bytes.as_ptr() as *const Self)
//         }
//     }
// }

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

pub trait RawProgramHeader {
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
    fn p_type(&self) -> u32 { self.p_type }
    fn p_offset(&self) -> u64 { self.p_offset as u64 }
    fn p_vaddr(&self) -> u64 { self.p_vaddr as u64 }
    fn p_paddr(&self) -> u64 { self.p_paddr as u64 }
    fn p_filesz(&self) -> u64 { self.p_filesz as u64 }
    fn p_memsz(&self) -> u64 { self.p_memsz as u64 }
    fn p_flags(&self) -> u32 { self.p_flags }
    fn p_align(&self) -> u64 { self.p_align as u64 }
}

pub trait RawSectionHeader {
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

pub trait RawSymbol {
    fn name(&self) -> u32;
    fn value(&self) -> u64;
    fn size(&self) -> u64;
    fn info(&self) -> u8;
    fn section_index(&self) -> u32;
}

impl RawSymbol for Elf32_Sym {
    fn name(&self) -> u32 {
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
    fn name(&self) -> u32 {
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