use super::RawElfHeader;
use super::RawProgramHeader;
use super::RawSectionHeader;
use super::RawSymbol;
use super::RawRelocation;
use super::RawDynamic;

pub trait ElfTypes {
    type Header: RawElfHeader;
    type ProgramHeader: RawProgramHeader;
    type SectionHeader: RawSectionHeader;
    type Symbol: RawSymbol;
    type Rel: RawRelocation;
    type Rela: RawRelocation;
    type Dynamic: RawDynamic;
}
