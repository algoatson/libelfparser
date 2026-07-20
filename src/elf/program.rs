use super::enums::{SegmentFlags, SegmentType};
use super::raw::RawProgramHeader;

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
    pub(crate) fn from<T: RawProgramHeader>(raw: &T) -> Self {
        Self {
            segment_type: raw.segment_type(),
            
            file_offset: raw.file_offset(),

            virt_address: raw.virtual_address(),
            phys_address: raw.physical_address(),

            file_size: raw.file_size(),
            memory_size: raw.memory_size(),

            flags: raw.flags(),

            alignment: raw.alignment(),
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

pub struct ElfSegment<'a> {
    header: ElfProgramHeader,
    data: &'a [u8],
}

impl<'a> ElfSegment<'a> {
    pub(crate) fn new(
        header: ElfProgramHeader,
        data: &'a [u8],
    ) -> Self {
        Self {
            header,
            data,
        }
    }

    pub fn header(&self) -> &ElfProgramHeader {
        &self.header
    }

    pub fn data(&self) -> &'a [u8] {
        self.data
    }

    pub fn contains_address(&self, addr: u64) -> bool {
        addr >= self.virtual_address()
            && addr < self.virtual_address() + self.memory_size()
    }

    // forwarder functions

    pub fn segment_type(&self) -> SegmentType {
        self.header().segment_type
    }

    pub fn file_offset(&self) -> u64 {
        self.header().file_offset
    }

    pub fn virtual_address(&self) -> u64 {
        self.header().virt_address
    }

    pub fn physical_address(&self) -> u64 {
        self.header().phys_address
    }

    pub fn file_size(&self) -> u64 {
        self.header().file_size
    }

    pub fn memory_size(&self) -> u64 {
        self.header().memory_size
    }

    pub fn flags(&self) -> SegmentFlags {
        self.header().flags
    }

    pub fn alignment(&self) -> u64 {
        self.header().alignment
    }
}