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

pub trait RawSym {
    fn name(&self) -> u32;
    fn value(&self) -> u64;
    fn size(&self) -> u64;
    fn info(&self) -> u8;
    fn section_index(&self) -> u32;
}

impl RawSym for Elf32_Sym {
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

impl RawSym for Elf64_Sym {
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
