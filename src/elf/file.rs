use super::header::{ElfHeader, parse_header};
use super::program::{ElfProgramHeader, ElfSegment, parse_segments};
use super::section::{ElfSectionHeader, ElfSection, parse_sections};
use super::symbols::{ElfSymbol, parse_symbols};
use super::relocation::{ElfRelocationSection, parse_relocations};
use super::dynamic::{ElfDynamicSection, parse_dynamic};
use super::raw::{RawElfHeader, RawProgramHeader, RawSectionHeader, ElfTypes, Elf32Types, Elf64Types};
use super::enums::{SectionType};
use super::constants::SHN_XINDEX;
use super::error::ElfError;

fn get_string(table: &[u8], offset: u32) -> Result<&str, ElfError> {
    let offset = offset as usize;

    if offset >= table.len() {
        return Err(ElfError::InvalidStringOffset);
    }

    let string = &table[offset..];

    let end = string
        .iter()
        .position(|&c| c == 0)
        .ok_or(ElfError::InvalidString)?;

    std::str::from_utf8(&string[..end])
        .map_err(|_| ElfError::InvalidString)
}

pub struct ElfFile<'a> {
    bytes: &'a [u8],
    header: ElfHeader,
    segments: Vec<ElfSegment<'a>>,
    sections: Vec<ElfSection<'a>>,
    symbols: Vec<ElfSymbol<'a>>,
    relocations: Vec<ElfRelocationSection>,
    dynamic: Option<ElfDynamicSection>
}

impl<'a> ElfFile<'a> {
    pub fn header(&self) -> &ElfHeader {
        &self.header
    }

    pub fn segments(&self) -> &[ElfSegment<'a>] {
        &self.segments
    }

    pub fn sections(&self) -> &[ElfSection<'a>] {
        &self.sections
    }

    pub fn symbols(&self) -> &[ElfSymbol<'a>] {
        &self.symbols
    }

    pub fn relocations(&self) -> &[ElfRelocationSection] {
        &self.relocations
    }

    pub fn dynamic(&self) -> &Option<ElfDynamicSection> {
        &self.dynamic
    }

    pub fn sections_by_type(
        &self,
        ty: SectionType,
    ) -> impl Iterator<Item=&ElfSection<'a>> {
        self.sections()
            .iter()
            .filter(move |section| section.section_type() == ty)
    }

    pub fn parse(bytes: &'a[u8]) -> Result<Self, ElfError> {
        let ident = bytes.get(0..16)
            .ok_or(ElfError::UnexpectedEOF)?;

        if ident[0] != 0x7f ||
           ident[1] != b'E' ||
           ident[2] != b'L' ||
           ident[3] != b'F'
        {
            return Err(ElfError::InvalidMagic);
        }

        match ident[4] {
            1 => Self::parse_impl::<Elf32Types>(bytes),
            2 => Self::parse_impl::<Elf64Types>(bytes),
            other => Err(ElfError::UnknownClass(other)),
        }
    }

    // today our goal is to write a generic parse() function,
    // because we're straight up repeating logic twice in 
    // parse32 and parse64, we could achieve this through
    // generic helpers and the generic parse.

    // we need to implement the trait RawElfHeader and implement
    // it for both Elf32_Ehdr, and Elf64_Ehdr.
    fn parse_impl<E: ElfTypes>(bytes: &'a [u8]) ->  Result<Self, ElfError> {
            let header =
                parse_header::<E::Header>(bytes)?;

            // we need a parse_segments generic
            let segments = 
                parse_segments::<E::ProgramHeader>(bytes, &header)?;

            // we need a parse_sections generic
            let mut sections = 
                parse_sections::<E::SectionHeader>(bytes, &header)?;

            let mut strndx = header.section_name_table_index();

            if strndx == SHN_XINDEX {
                strndx = sections[0].header().link();
            }

            let strtab = sections
                .get(strndx as usize)
                .ok_or(ElfError::InvalidSectionIndex)?;

            let strtab_data = strtab.data();

            for section in &mut sections {
                let name = get_string(
                    strtab_data,
                    section.name_offset()
                )?;

                section.set_name(name)
            }

        let mut symbols = Vec::new();
        let mut relocation_sections = Vec::new();
        let mut dynamic: Option<ElfDynamicSection> = None;

        // im not a big fan of the following tab cascade
        // but its so far the best way i think i couldve
        // implemented this so far.

        // we loop once and we do all the parsing within
        // that loop.
        for (index, section) in sections.iter().enumerate() {
            match section.section_type() {
                // parse symbols
                SectionType::SymbolTable | SectionType::DynSym => {
                    symbols.extend(
                        parse_symbols::<E::Symbol>(
                            index, 
                            &sections)?
                    );
                }

                // parse dynamic section
                SectionType::Dynamic => {
                    dynamic = parse_dynamic::<E::Dynamic>(
                        index, 
                        &section
                    )?;
                }

                // parse relocations
                SectionType::Rel => {
                    relocation_sections.extend(
                        parse_relocations::<E::Rel>(
                            index,
                            &sections
                        )
                    );
                }

                // parse relocations
                SectionType::Rela => {
                    relocation_sections.extend(
                        parse_relocations::<E::Rela>(
                            index,
                            &sections
                        )
                    );
                }

                _ => {}
            }
        }

        Ok(Self {
            bytes,
            header,
            segments,
            sections,
            symbols,
            relocations: relocation_sections,
            dynamic
        })

    }
}