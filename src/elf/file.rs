use std::marker::PhantomData;

use super::header::{ElfHeader, ElfProgramHeader, ElfProgram, ElfSectionHeader, ElfSection};
use super::raw::{Elf32_Ehdr, Elf64_Ehdr, Elf32_Phdr, Elf64_Phdr, Elf32_Shdr, Elf64_Shdr};
use super::enums::SectionType;
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
    elf_hdr: ElfHeader,
    prog_hdrs: Vec<ElfProgram<'a>>,
    sect_hdrs: Vec<ElfSection<'a>>
}

impl<'a> ElfFile<'a> {
    pub fn header(&self) -> &ElfHeader {
        &self.elf_hdr
    }

    pub fn segments(&self) -> &[ElfProgram<'a>] {
        &self.prog_hdrs
    }

    pub fn sections(&self) -> &[ElfSection<'a>] {
        &self.sect_hdrs
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

            segments.push(ElfProgram::new(
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

            let shdr = ElfSectionHeader::from_32(&raw_shdr);

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
                section.header().name_offset()
            )?;

            section.set_name(name);
        }

        Ok(Self {
            bytes,
            elf_hdr: header,
            prog_hdrs: segments,
            sect_hdrs: sections,
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

            segments.push(ElfProgram::new(
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

            let shdr = ElfSectionHeader::from_64(&raw_shdr);

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
                section.header().name_offset()
            )?;

            section.set_name(name);
        }

        // next step is to resolve symbols.

        Ok(Self {
            bytes,
            elf_hdr: header,
            prog_hdrs: segments,
            sect_hdrs: sections,
        })
    }
}