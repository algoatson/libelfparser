use super::enums::{SectionFlags, SectionType};
use super::raw::RawSectionHeader;

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
    pub(crate) fn from<T: RawSectionHeader>(raw: &T) -> Self {
        Self {
            name_offset: raw.name_offset(),
            section_type: SectionType::from(raw.section_type()),
            flags: raw.flags(),
            
            virtual_address: raw.virtual_address(),
            file_offset: raw.file_offset(),

            size: raw.size(),

            link: raw.link(),
            info: raw.info(),

            alignment: raw.alignment(),
            entry_size: raw.entry_size(),
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

    pub fn linked_section<'b>(
        &self,
        sections: &'b [ElfSection<'a>]
    ) -> Option<&'b ElfSection<'a>> {
        sections.get(self.link() as usize)
    }

    // forwarder functions

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

    pub fn link(&self) -> u32 {
        self.header().link()
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