use super::enums::{Endianness, Machine, ElfClass, FileType, SegmentType, SegmentFlags, SectionType, SectionFlags};

pub struct ElfHeader {
    magic: [u8; 4],
    class: ElfClass,
    endianness: Endianness,
    file_type: FileType,
    machine: Machine,
    version: u32,

    entry: u64,
    program_header_offset: u64,
    section_header_offset: u64,

    header_size: u16,
    program_header_size: u16,
    program_header_count: u16,
    section_header_size: u16,
    section_header_count: u16,
    section_name_table_index: u32,
}

impl ElfHeader {
    pub(crate) fn from_32(raw: &crate::elf::raw::Elf32_Ehdr) -> Self {
        Self {
            magic: [
                raw.e_ident[0],
                raw.e_ident[1],
                raw.e_ident[2],
                raw.e_ident[3],
            ],

            class: ElfClass::from(raw.e_ident[4]),
            endianness: Endianness::from(raw.e_ident[5]),

            file_type: FileType::from(raw.e_type),
            machine: Machine::from(raw.e_machine),

            version: raw.e_version,

            entry: raw.e_entry as u64,
            program_header_offset: raw.e_phoff as u64,
            section_header_offset: raw.e_shoff as u64,

            header_size: raw.e_ehsize,
            program_header_size: raw.e_phentsize,
            program_header_count: raw.e_phnum,

            section_header_size: raw.e_shentsize,
            section_header_count: raw.e_shnum,
            
            section_name_table_index: raw.e_shstrndx as u32,
        }
    }

    pub(crate) fn from_64(raw: &crate::elf::raw::Elf64_Ehdr) -> Self {
        Self {
            magic: [
                raw.e_ident[0],
                raw.e_ident[1],
                raw.e_ident[2],
                raw.e_ident[3],
            ],

            class: ElfClass::from(raw.e_ident[4]),
            endianness: Endianness::from(raw.e_ident[5]),

            file_type: FileType::from(raw.e_type),
            machine: Machine::from(raw.e_machine),

            version: raw.e_version,

            entry: raw.e_entry,
            program_header_offset: raw.e_phoff,
            section_header_offset: raw.e_shoff,

            header_size: raw.e_ehsize,
            program_header_size: raw.e_phentsize,
            program_header_count: raw.e_phnum,

            section_header_size: raw.e_shentsize,
            section_header_count: raw.e_shnum,
            
            section_name_table_index: raw.e_shstrndx as u32,
        }
    }

    pub fn magic(&self) -> [u8; 4] {
        self.magic
    }

    pub fn class(&self) -> ElfClass {
        self.class
    }

    pub fn endianness(&self) -> Endianness {
        self.endianness
    }

    pub fn file_type(&self) -> FileType {
        self.file_type
    }

    pub fn machine(&self) -> Machine {
        self.machine
    }

    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn entry(&self) -> u64 {
        self.entry
    }

    pub fn program_header_offset(&self) -> u64 {
        self.program_header_offset
    }

    pub fn section_header_offset(&self) -> u64 {
        self.section_header_offset
    }

    pub fn header_size(&self) -> u16 {
        self.header_size
    }

    pub fn program_header_size(&self) -> u16 {
        self.program_header_size
    }

    pub fn program_header_count(&self) -> u16 {
        self.program_header_count
    }

    pub fn section_header_size(&self) -> u16 {
        self.section_header_size
    }

    pub fn section_header_count(&self) -> u16 {
        self.section_header_count
    }

    pub fn section_name_table_index(&self) -> u32 {
        self.section_name_table_index
    }
}

pub struct ElfProgramHeader {
    segment_type: SegmentType,

    file_offset: u64,

    virt_address: u64,
    phys_address: u64,

    file_size: u64,
    memory_size: u64,

    flags: SegmentFlags,

    alignment: u64,
}

impl ElfProgramHeader {
    pub(crate) fn from_32(raw: &crate::elf::raw::Elf32_Phdr) -> Self {
        Self {
            segment_type: SegmentType::from(raw.p_type),
            
            file_offset: raw.p_offset as u64,

            virt_address: raw.p_vaddr as u64,
            phys_address: raw.p_paddr as u64,

            file_size: raw.p_filesz as u64,
            memory_size: raw.p_memsz as u64,

            flags: SegmentFlags::from(raw.p_flags),

            alignment: raw.p_align as u64,
        }
    }

    pub(crate) fn from_64(raw: &crate::elf::raw::Elf64_Phdr) -> Self {
        Self {
            segment_type: SegmentType::from(raw.p_type),
            
            file_offset: raw.p_offset,

            virt_address: raw.p_vaddr,
            phys_address: raw.p_paddr,

            file_size: raw.p_filesz,
            memory_size: raw.p_memsz,

            flags: SegmentFlags::from(raw.p_flags),

            alignment: raw.p_align,
        }
    }

    pub fn segment_type(&self) -> SegmentType {
        self.segment_type
    }

    pub fn file_offset(&self) -> u64 {
        self.file_offset
    }

    pub fn virtual_address(&self) -> u64 {
        self.virt_address
    }

    pub fn physical_address(&self) -> u64 {
        self.phys_address
    }

    pub fn file_size(&self) -> u64 {
        self.file_size
    }

    pub fn memory_size(&self) -> u64 {
        self.memory_size
    }

    pub fn flags(&self) -> SegmentFlags {
        self.flags
    }

    pub fn alignment(&self) -> u64 {
        self.alignment
    }

}

#[derive(Debug, Clone, Copy)]
pub struct ElfSectionHeader {
    pub name_offset: u32,
    pub section_type: SectionType,
    pub flags: SectionFlags,

    pub virtual_address: u64,
    pub file_offset: u64,

    pub size: u64,

    pub link: u32,
    pub info: u32,

    pub alignment: u64,
    pub entry_size: u64,
}

impl ElfSectionHeader {
    pub(crate) fn from_32(raw: &crate::elf::raw::Elf32_Shdr) -> Self {
        Self {
            name_offset: raw.sh_name,
            section_type: SectionType::from(raw.sh_type),
            flags: SectionFlags::from_bits_truncate(raw.sh_flags as u64),
            
            virtual_address: raw.sh_addr as u64,
            file_offset: raw.sh_offset as u64,

            size: raw.sh_size as u64,

            link: raw.sh_link,
            info: raw.sh_info,

            alignment: raw.sh_addralign as u64,
            entry_size: raw.sh_entsize as u64
        }
    }

    pub(crate) fn from_64(raw: &crate::elf::raw::Elf64_Shdr) -> Self {
        Self {
            name_offset: raw.sh_name,
            section_type: SectionType::from(raw.sh_type),
            flags: SectionFlags::from_bits_truncate(raw.sh_flags),
            
            virtual_address: raw.sh_addr,
            file_offset: raw.sh_offset,

            size: raw.sh_size,

            link: raw.sh_link,
            info: raw.sh_info,

            alignment: raw.sh_addralign,
            entry_size: raw.sh_entsize
        }
    }

    pub fn name_offset(&self) -> u32 {
        self.name_offset
    }

    pub fn section_type(&self) -> SectionType {
        self.section_type
    }

    pub fn flags(&self) -> SectionFlags {
        self.flags
    }

    pub fn virtual_address(&self) -> u64 {
        self.virtual_address
    }

    pub fn file_offset(&self) -> u64 {
        self.file_offset
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn link(&self) -> u32 {
        self.link
    }

    pub fn info(&self) -> u32 {
        self.info
    }

    pub fn alignment(&self) -> u64 {
        self.alignment
    }

    pub fn entry_size(&self) -> u64 {
        self.entry_size
    }
}

pub struct ElfSection<'a> {
    header: ElfSectionHeader,
    name: Option<&'a str>,
    data: &'a [u8],
}

impl<'a> ElfSection<'a> {
    pub(crate) fn new(
        header: ElfSectionHeader,
        data: &'a [u8],
    ) -> Self {
        Self {
            header,
            name: None,
            data,
        }
    }

    pub fn header(&self) -> &ElfSectionHeader {
        &self.header
    }

    pub fn name(&self) -> Option<&str> {
        self.name
    }
    
    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    pub(crate) fn set_name(&mut self, name: &'a str) {
        self.name = Some(name);
    }

    pub fn name_offset(&self) -> u32 {
        self.header.name_offset()
    }

    pub fn section_type(&self) -> SectionType {
        self.header.section_type()
    }

    pub fn virtual_address(&self) -> u64 {
        self.header.virtual_address()
    }

    pub fn file_offset(&self) -> u64 {
        self.header.file_offset()
    }

    pub fn size(&self) -> u64 {
        self.header.size()
    }

    pub fn flags(&self) -> SectionFlags {
        self.header.flags()
    }

    pub fn alignment(&self) -> u64 {
        self.header.alignment()
    }

    pub fn entry_size(&self) -> u64 {
        self.header.entry_size()
    }
}

// struct ElfSymbol {
//     name: Option<&str>,
//     value: u64,
//     size: u64,
//     binding: SymbolBinding,
//     symbol_type: SymbolType,
// }