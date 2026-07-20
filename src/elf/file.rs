use super::header::{ElfHeader, ElfProgramHeader, ElfSegment, ElfSectionHeader, ElfSection, parse_header};
use super::symbols::{ElfSymbol, parse_symbols};
use super::relocation::{ElfRelocationSection, parse_relocations};
use super::dynamic::{ElfDynamicSection, parse_dynamic};
use super::raw::{Elf32_Ehdr, Elf64_Ehdr, Elf32_Phdr, Elf64_Phdr, Elf32_Shdr, Elf64_Shdr, Elf32_Sym, Elf64_Sym, Elf32_Rel, Elf32_Rela, Elf64_Rel, Elf64_Rela, Elf32_Dyn, Elf64_Dyn, RawElfHeader, RawProgramHeader, RawSectionHeader};
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
            1 => Self::parse_gen::<Elf32_Ehdr, Elf32_Phdr, Elf32_Shdr>(bytes),
            2 => Self::parse_gen::<Elf64_Ehdr, Elf64_Phdr, Elf64_Shdr>(bytes),
            other => Err(ElfError::UnknownClass(other)),
        }
    }

    // today our goal is to write a generic parse() function,
    // because we're straight up repeating logic twice in 
    // parse32 and parse64, we could achieve this through
    // generic helpers and the generic parse.

    // we need to implement the trait RawElfHeader and implement
    // it for both Elf32_Ehdr, and Elf64_Ehdr.
    fn parse_gen<Ehdr, Phdr, Shdr>(bytes: &'a [u8]) ->  Result<Self, ElfError> 
        where Ehdr: RawElfHeader,
              Phdr: RawProgramHeader,
              Shdr: RawSectionHeader, {
            let header =
                parse_header::<Ehdr>(bytes)?;

            let mut segments = Vec::new();

            let ph_offset = header.program_header_offset() as usize;
            let ph_size = header.program_header_size() as usize;
            let ph_count = header.program_header_count() as usize;

            for i in 0..ph_count {
                let start = ph_offset + (i * ph_size);
                let end = start + ph_size;

                let raw_phdr = Phdr::from_bytes(&bytes[start..end])?;

                segments.push(ElfSegment::new(
                    ElfProgramHeader::from(&raw_phdr),
                    &bytes[start..end]
                ));
            }

            let mut sections = Vec::new();

            let sh_offset = header.section_header_offset() as usize;
            let sh_size = header.section_header_size() as usize;
            let sh_count = header.section_header_count() as usize;

            for i in 0..sh_count {
                let start = sh_offset + (i * sh_size);
                let end = start + sh_size;

                let raw_shdr = Shdr::from_bytes(&bytes[start..end])?;

                let shdr = ElfSectionHeader::from(&raw_shdr);

                let data = match shdr.section_type() {
                    SectionType::NoBits => &[],

                    _ => {
                        let start: usize = shdr.file_offset()
                            .try_into()
                        .map_err(|_| ElfError::InvalidOffset)?;
                                    
                    let end: usize = shdr.file_offset()
                        .checked_add(shdr.size())
                        .ok_or(ElfError::InvalidOffset)?
                        .try_into()
                        .map_err(|_| ElfError::InvalidOffset)?;
                
                    bytes
                        .get(start..end)
                        .ok_or(ElfError::UnexpectedEOF)?
                    }
                };

                sections.push(ElfSection::new(
                    shdr,
                    data,
                ));
            }

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

        for (index, section) in sections.iter().enumerate() {
            match section.section_type() {
                // parse symbols
                SectionType::SymbolTable | SectionType::DynSym => {
                    symbols.extend(
                        parse_symbols::<Elf64_Sym>(
                            index, 
                            &sections)?
                    );
                }

                // parse dynamic section
                SectionType::Dynamic => {
                    dynamic = parse_dynamic::<Elf64_Dyn>(
                        index, 
                        &section
                    )?;
                }

                // parse relocations
                SectionType::Rel => {
                    relocation_sections.extend(
                        parse_relocations::<Elf64_Rel>(
                            index,
                            &sections
                        )
                    );
                }

                // parse relocations
                SectionType::Rela => {
                    relocation_sections.extend(
                        parse_relocations::<Elf64_Rela>(
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