use super::enums::RelocationType;
use super::error::ElfError;
use super::raw::RawRelocation;
use super::header::ElfSection;
use super::symbols::ElfSymbol;

pub struct ElfRelocation {
    offset: u64,
    symbol_index: u32,
    relocation_type: RelocationType,
    addend: Option<i64>,
}

impl ElfRelocation {
    pub(crate) fn from<T: RawRelocation>(
        raw: &T,
    ) -> Self {
        Self {
            offset: raw.offset(),
            symbol_index: raw.symbol_index(),
            relocation_type: raw.relocation_type(),
            addend: raw.addend(),
        }
    }
    
    pub fn symbol<'a>(
        &self,
        symbols: &'a [ElfSymbol<'a>]
    ) -> Option<&'a ElfSymbol<'a>> {
        symbols.get(self.symbol_index as usize)
    }

    pub fn offset(&self) -> u64 {
        self.offset
    }

    pub fn symbol_index(&self) -> u32 {
        self.symbol_index
    }

    pub fn relocation_type(&self) -> RelocationType {
        self.relocation_type
    }

    pub fn addend(&self) -> Option<i64> {
        self.addend
    }
}

pub struct ElfRelocationSection {
    section_index: usize,
    relocations: Vec<ElfRelocation>,
}

impl ElfRelocationSection {
    pub fn new(
        section_index: usize, 
        relocations: Vec<ElfRelocation>
    ) -> Self {
        Self {
            section_index,
            relocations
        }
    }

    pub fn relocations(&self) -> &[ElfRelocation] {
        &self.relocations
    }

    pub fn section_index(&self) -> usize {
        self.section_index
    }

    pub fn section<'a>(
        &self,
        sections: &'a [ElfSection<'a>],
    ) -> Option<&'a ElfSection<'a>> {
        sections.get(self.section_index as usize)
    }
}

pub fn parse_relocations<'a, T>(
    section_index: usize,
    sections: &'a [ElfSection<'a>]
) -> Result<ElfRelocationSection, ElfError> 
where 
    T: RawRelocation {
        let mut relocations = Vec::new();

        let section = sections
            .get(section_index)
            .ok_or(ElfError::InvalidSectionIndex)?;

        let entsize = section.entry_size() as usize;

        if entsize == 0 {
            return Err(ElfError::InvalidEntrySize);
        }

        for chunk in section.data().chunks_exact(entsize) {
            let raw = T::from_bytes(chunk)?;

            relocations.push(ElfRelocation::from(&raw));
        }

        Ok(ElfRelocationSection::new(section_index, relocations))
}