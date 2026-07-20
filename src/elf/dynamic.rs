use super::enums::DynamicTag;
use super::error::ElfError;
use super::section::ElfSection;
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

pub fn parse_dynamic<'a, T>(
    section_index: usize,
    section: &ElfSection<'a>
) -> Result<Option<ElfDynamicSection>, ElfError>
where
    T: RawDynamic {
        let mut entries = Vec::new();
        let entsize = section.entry_size() as usize;

        if entsize == 0 {
            return Err(ElfError::InvalidEntrySize);
        }

        for chunk in section.data().chunks_exact(entsize) {
            let raw = T::from_bytes(chunk)?;

            let entry = ElfDynamicEntry::from(&raw);

            if entry.tag() == DynamicTag::Null {
                break;
            }

            entries.push(entry);
        }

        Ok(Some(ElfDynamicSection::new(section_index, entries)))
    }