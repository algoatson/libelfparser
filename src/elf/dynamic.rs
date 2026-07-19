use super::enums::DynamicTag;
use super::header::ElfSection;
use super::raw::RawDynamic;

pub struct ElfDynamicEntry {
    tag: DynamicTag,
    value: u64,
}

impl ElfDynamicEntry {
    pub(crate) fn from<T: RawDynamic>(
        raw: &T
    ) -> Self {
        Self {
            tag: raw.tag(),
            value: raw.value(),
        }
    }

    pub fn tag(&self) -> DynamicTag {
        self.tag
    }

    pub fn value(&self) -> u64 {
        self.value
    }
}

pub struct ElfDynamicSection {
    section_index: usize,
    entries: Vec<ElfDynamicEntry>,
}

impl ElfDynamicSection {
    pub fn new(
        section_index: usize,
        entries: Vec<ElfDynamicEntry>
    ) -> Self {
        Self {
            section_index,
            entries,
        }
    }

    pub fn entries(&self) -> &[ElfDynamicEntry] {
        &self.entries
    }

    pub fn section_index(&self) -> usize {
        self.section_index
    }

    pub fn section<'a> (
        &self,
        sections: &'a [ElfSection<'a>],
    ) -> Option<&'a ElfSection<'a>> {
        sections.get(self.section_index as usize)
    }
}