use super::enums::{SymbolBinding, SymbolType};
use super::error::ElfError;
use super::utility::get_string;
use super::section::ElfSection;
use super::raw::RawSymbol;

pub struct ElfSymbol<'a> {
    name: Option<&'a str>,
    value: u64,
    size: u64,
    binding: SymbolBinding,
    symbol_type: SymbolType,
    section_index: u32,
}

impl<'a> ElfSymbol<'a> {
    pub(crate) fn from<T: RawSymbol>(
        raw: &T, 
        name: Option<&'a str>
    ) -> Self {
        let info = raw.info();

        Self {
            name,
            value: raw.value() as u64,
            size: raw.size() as u64,
            binding: SymbolBinding::from(info >> 4),
            symbol_type: SymbolType::from(info & 0xf),
            section_index: raw.section_index() as u32,
        }
    }

    pub fn name(&self) -> Option<&'a str> {
        self.name
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn binding(&self) -> SymbolBinding {
        self.binding
    }

    pub fn symbol_type(&self) -> SymbolType {
        self.symbol_type
    }

    pub fn section_index(&self) -> u32 {
        self.section_index
    }
}

pub fn parse_symbols<'a, T>(
    section_index: usize,
    sections: &[ElfSection<'a>]
) -> Result<Vec<ElfSymbol<'a>>, ElfError>
where 
    T: RawSymbol {
    let mut symbols = Vec::new();

    let section = sections
        .get(section_index)
        .ok_or(ElfError::InvalidSectionIndex)?;

    let string_table = sections
                    .get(section.link() as usize)
                    .ok_or(ElfError::InvalidSectionIndex)?;

    // iterate over symbols
    let section_data = section.data();
    let entsize = section.entry_size() as usize;

    // validate entry size
    if entsize == 0 {
        return Err(ElfError::InvalidEntrySize);
    }

    for chunk in section_data.chunks_exact(entsize) {
        let raw = T::from_bytes(chunk)?;

        let name = get_string(
            string_table.data(),
            raw.name_offset(),
        ).ok();

        symbols.push(ElfSymbol::from(&raw, name));
    }

    Ok(symbols)

}