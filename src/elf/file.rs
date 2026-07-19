use std::marker::PhantomData;

use super::header::{ElfHeader, ElfProgramHeader, ElfSegment, ElfSectionHeader, ElfSection, ElfSymbol};
use super::relocation::{ElfRelocation, ElfRelocationSection};
use super::dynamic::{ElfDynamicEntry, ElfDynamicSection};
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

        for section in &sections {
            match section.section_type() {
                SectionType::SymbolTable | SectionType::DynSym => {
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
                        let raw = Elf32_Sym::from_bytes(chunk)?;

                        let name = get_string(
                            string_table.data(),
                            raw.st_name,
                        ).ok();

                        symbols.push(ElfSymbol::from(&raw, name));
                    }

                }

                _ => {}
            }
        }

        let mut relocation_sections = Vec::new();

        // next step is to resolve relocations
        for (index, section) in sections.iter().enumerate() {
            match section.section_type() {
                SectionType::Rela => {
                    let mut relocations = Vec::new();
                
                    for chunk in section.data().chunks_exact(section.entry_size() as usize) {
                        let raw = Elf32_Rela::from_bytes(chunk)?;
                    
                        relocations.push(
                            ElfRelocation::from(&raw)
                        );
                    }
                
                    relocation_sections.push(
                        ElfRelocationSection::new(
                            index,
                            relocations,
                        )
                    );
                }
            
                SectionType::Rel => {
                    let mut relocations = Vec::new();
                
                    for chunk in section.data().chunks_exact(section.entry_size() as usize) {
                        let raw = Elf32_Rel::from_bytes(chunk)?;
                    
                        relocations.push(
                            ElfRelocation::from(&raw)
                        );
                    }
                
                    relocation_sections.push(
                        ElfRelocationSection::new(
                            index,
                            relocations,
                        )
                    );
                }
            
                _ => {}
            }
        }

        let mut dynamic: Option<ElfDynamicSection> = None;

        for (index, section) in sections.iter().enumerate() {
            if section.section_type() != SectionType::Dynamic {
                continue;
            }

            let mut entries = Vec::new();
            let entsize = section.entry_size() as usize;

            if entsize == 0 {
                return Err(ElfError::InvalidEntrySize);
            }

            for chunk in section.data().chunks_exact(entsize) {
                let raw = Elf64_Dyn::from_bytes(chunk)?;

                let entry = ElfDynamicEntry::from(&raw);

                // dynamic table is terminated by DT_NULL
                if entry.tag() == DynamicTag::Null {
                    break;
                }

                entries.push(entry);
            }

            dynamic = Some(
                ElfDynamicSection::new(index, entries)
            );

            break;


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
        let mut symbols = Vec::new();

        for section in &sections {
            match section.section_type() {
                SectionType::SymbolTable | SectionType::DynSym => {
                    // locate linked string table (sh_link)
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

                    // iterate every Elf64_Sym
                    for chunk in section_data.chunks_exact(entsize) {
                        let raw = Elf64_Sym::from_bytes(chunk)?;

                        // resolve st_name
                        let name = get_string(
                            string_table.data(),
                            raw.st_name,
                        ).ok();

                        // push ElfSymbol
                        symbols.push(ElfSymbol::from(&raw, name));
                    }

                }

                _ => {}
            }
        }

        // i could probably merge match block, however
        // im not too certain about this.

        let mut relocation_sections = Vec::new();

        // next step is to resolve relocations
        for (index, section) in sections.iter().enumerate() {
            match section.section_type() {
                SectionType::Rela => {
                    let mut relocations = Vec::new();
                
                    for chunk in section.data().chunks_exact(section.entry_size() as usize) {
                        let raw = Elf64_Rela::from_bytes(chunk)?;
                    
                        relocations.push(
                            ElfRelocation::from(&raw)
                        );
                    }
                
                    relocation_sections.push(
                        ElfRelocationSection::new(
                            index,
                            relocations,
                        )
                    );
                }
            
                SectionType::Rel => {
                    let mut relocations = Vec::new();
                
                    for chunk in section.data().chunks_exact(section.entry_size() as usize) {
                        let raw = Elf64_Rel::from_bytes(chunk)?;
                    
                        relocations.push(
                            ElfRelocation::from(&raw)
                        );
                    }
                
                    relocation_sections.push(
                        ElfRelocationSection::new(
                            index,
                            relocations,
                        )
                    );
                }
            
                _ => {}
            }
        }

        let mut dynamic: Option<ElfDynamicSection> = None;

        for (index, section) in sections.iter().enumerate() {
            if section.section_type() != SectionType::Dynamic {
                continue;
            }

            let mut entries = Vec::new();
            let entsize = section.entry_size() as usize;

            if entsize == 0 {
                return Err(ElfError::InvalidEntrySize);
            }

            for chunk in section.data().chunks_exact(entsize) {
                let raw = Elf64_Dyn::from_bytes(chunk)?;

                let entry = ElfDynamicEntry::from(&raw);

                // dynamic table is terminated by DT_NULL
                if entry.tag() == DynamicTag::Null {
                    break;
                }

                entries.push(entry);
            }

            dynamic = Some(
                ElfDynamicSection::new(index, entries)
            );

            break;


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