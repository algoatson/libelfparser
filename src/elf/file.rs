use std::marker::PhantomData;

use super::header::{ElfHeader, ElfProgramHeader, ElfSegment, ElfSectionHeader, ElfSection};
use super::symbols::{ElfSymbol, parse_symbols};
use super::relocation::{ElfRelocation, ElfRelocationSection, parse_relocations};
use super::dynamic::{ElfDynamicEntry, ElfDynamicSection, parse_dynamic};
use super::raw::{Elf32_Ehdr, Elf64_Ehdr, Elf32_Phdr, Elf64_Phdr, Elf32_Shdr, Elf64_Shdr, Elf32_Sym, Elf64_Sym, Elf32_Rel, Elf32_Rela, Elf64_Rel, Elf64_Rela, Elf32_Dyn, Elf64_Dyn};
use super::enums::{DynamicTag, SectionType};
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
            1 => Self::parse32(bytes),
            2 => Self::parse64(bytes),
            other => Err(ElfError::UnknownClass(other)),
        }
    }

    fn parse32(bytes: &'a [u8]) -> Result<Self, ElfError> {
        let raw = match Elf32_Ehdr::from_bytes(bytes) {
            Ok(raw) => raw,
            Err(e) => return Err(e),
        };

        let header = ElfHeader::from_32(&raw);

        let mut segments = Vec::new();

        let ph_offset = header.program_header_offset() as usize;
        let ph_size = header.program_header_size() as usize;
        let ph_count = header.program_header_count() as usize;

        for i in 0..ph_count {
            let start = ph_offset + (i * ph_size);
            let end = start + ph_size;

            let raw_phdr = match Elf32_Phdr::from_bytes(&bytes[start..end]) {
                Ok(raw) => raw,
                Err(e) => return Err(e),
            };

            segments.push(ElfSegment::new(
                ElfProgramHeader::from_32(&raw_phdr),
                &bytes[start..end]
            ));
        }

        let mut sections = Vec::new();

        let sh_offset: usize = header.section_header_offset() as usize;
        let sh_size: usize = header.section_header_size() as usize;
        let sh_count: usize = header.section_header_count() as usize;

        for i in 0..sh_count {
            let start = sh_offset + (i * sh_size);
            let end = start + sh_size;

            let raw_shdr = Elf32_Shdr::from_bytes(&bytes[start..end])?;

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
            // read actual index from section 0 sh_link.
            strndx = sections[0].link();
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

            section.set_name(name);
        }

        // for every section
        // if type == SYMTAB or DYNSYM
        //     locate linked string table (sh_link)
        //     iterate every Elf*_Sym
        //     resolve st_name
        //     push ElfSymbol

        let mut symbols = Vec::new();
        let mut relocation_sections = Vec::new();
        let mut dynamic: Option<ElfDynamicSection> = None;

        for (index, section) in sections.iter().enumerate() {
            match section.section_type() {
                // parse symbols
                SectionType::SymbolTable | SectionType::DynSym => {
                    symbols = match
                        parse_symbols::<Elf64_Sym>(index, &sections) {
                            Ok(value) => value,
                            Err(e) => return Err(e),
                        }
                }

                SectionType::Dynamic => {
                    dynamic = match parse_dynamic::<Elf64_Dyn>(index, &section) {
                        Ok(value) => value,
                        Err(e) => return Err(e), 
                    };
                }

                SectionType::Rel => {
                    let x = parse_relocations::<Elf64_Rel>(index, &sections);
                    match x {
                        Ok(value) => relocation_sections.push(value),
                        Err(e) => return Err(e),
                    }
                }

                SectionType::Rela => {
                    let x = parse_relocations::<Elf64_Rela>(index, &sections);
                    match x {
                        Ok(value) => relocation_sections.push(value),
                        Err(e) => return Err(e),
                    }
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

    fn parse64(bytes: &'a [u8]) -> Result<Self, ElfError> {
        let raw = match Elf64_Ehdr::from_bytes(bytes) {
            Ok(raw) => raw,
            Err(e) => return Err(e),
        };

        let header = ElfHeader::from_64(&raw);

        let mut segments = Vec::new();

        let ph_offset = header.program_header_offset() as usize;
        let ph_size = header.program_header_size() as usize;
        let ph_count = header.program_header_count() as usize;

        for i in 0..ph_count {
            let start = ph_offset + (i * ph_size);
            let end = start + ph_size;

            let raw_phdr = Elf64_Phdr::from_bytes(&bytes[start..end])?;

            segments.push(ElfSegment::new(
                ElfProgramHeader::from_64(&raw_phdr),
                &bytes[start..end]
            ));
        }

        let mut sections = Vec::new();

        let sh_offset: usize = header.section_header_offset() as usize;
        let sh_size: usize = header.section_header_size() as usize;
        let sh_count: usize = header.section_header_count() as usize;

        for i in 0..sh_count {
            let start = sh_offset + (i * sh_size);
            let end = start + sh_size;

            let raw_shdr = Elf64_Shdr::from_bytes(&bytes[start..end])?;

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
            // read actual index from section 0 sh_link.
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

            section.set_name(name);
        }

        // next step is to resolve symbols.

        // so right now we have implemented the following way,
        // however i think what would be better is a match case
        // that checks the section type and call parse_symbols in there.
        // parse_symbols would then take a section index, or straight
        // up a section.

        let mut symbols = Vec::new();
        let mut relocation_sections = Vec::new();
        let mut dynamic: Option<ElfDynamicSection> = None;

        for (index, section) in sections.iter().enumerate() {
            match section.section_type() {
                // parse symbols
                SectionType::SymbolTable | SectionType::DynSym => {
                    symbols = match
                        parse_symbols::<Elf64_Sym>(index, &sections) {
                            Ok(value) => value,
                            Err(e) => return Err(e),
                        }
                }

                SectionType::Dynamic => {
                    dynamic = match parse_dynamic::<Elf64_Dyn>(index, &section) {
                        Ok(value) => value,
                        Err(e) => return Err(e), 
                    };
                }

                SectionType::Rel => {
                    let x = parse_relocations::<Elf64_Rel>(index, &sections);
                    match x {
                        Ok(value) => relocation_sections.push(value),
                        Err(e) => return Err(e),
                    }
                }

                SectionType::Rela => {
                    let x = parse_relocations::<Elf64_Rela>(index, &sections);
                    match x {
                        Ok(value) => relocation_sections.push(value),
                        Err(e) => return Err(e),
                    }
                }

                _ => {}
            }
        }

        // i could probably merge match block, however
        // im not too certain about this.

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