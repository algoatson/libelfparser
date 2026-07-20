use super::enums::{Endianness, Machine, ElfClass, FileType, SegmentType, SegmentFlags, SectionType, SectionFlags, SymbolBinding, SymbolType, RelocationType, DynamicTag};
use super::error::ElfError;
use super::raw::{RawProgramHeader, RawSectionHeader, RawSymbol, RawElfHeader};

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
    pub(crate) fn from<T: RawElfHeader>(
        raw: &T,
    ) -> Self {
        Self {
            magic: raw.magic(),
            class: raw.class(),
            endianness: raw.endianness(),
            file_type: raw.file_type(),
            machine: raw.machine(),
            version: raw.version(),
            entry: raw.entry(),
            program_header_offset: raw.program_header_offset(),
            section_header_offset: raw.section_header_offset(),

            header_size: raw.header_size(),
            program_header_size: raw.program_header_size(),
            program_header_count: raw.program_header_count(),
            section_header_size: raw.section_header_size(),
            section_header_count: raw.section_header_count(),
            section_name_table_index: raw.section_name_table_index(),
        }
    }

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

pub fn parse_header<T>(bytes: &[u8]) -> Result<ElfHeader, ElfError>
    where T: RawElfHeader {
        let raw = match T::from_bytes(bytes) {
            Ok(value) => value,
            Err(e) => return Err(e),
        };

        Ok(ElfHeader::from(&raw))
    }