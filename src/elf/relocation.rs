use super::enums::RelocationType;
use super::header::ElfSymbol;

pub struct ElfRelocation {
    offset: u64,
    symbol_index: u32,
    relocation_type: RelocationType,
    addend: i64,
}

impl ElfRelocation {
    pub fn symbol<'a>(
        &self,
        symbols: &'a [ElfSymbol<'a>]
    ) -> Option<&'a ElfSymbol<'a>> {
        symbols.get(self.symbol_index as usize)
    }
}