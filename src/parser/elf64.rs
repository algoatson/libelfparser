use crate::elf::ElfHeader;
use libc::{Elf64_Ehdr, Elf64_Phdr, Elf64_Shdr};

fn parse64(bytes: &[u8]) -> ElfHeader {
    let header = unsafe {
        &*(bytes.as_ptr() as *const Elf64_Ehdr)
    };

    ElfHeader {
        raw_ident: header.e_ident as u64,
        raw_type:  header.e_type as u64,
        raw_machine: header.e_machine as u64,
        raw_version: header.e_version as u64,
        raw_entry: header.e_entry as u64,
        raw_phoff: header.e_phoff as u64,
        raw_shoff: header.e_shoff as u64,
        raw_flags: header.e_flags as u64,
        raw_ehsize: header.e_ehsize as u64,
        raw_phentsize: header.e_phentsize as u64,
        raw_phnum: header.e_phnum as u64,
        raw_shentsize: header.e_shentsize as u64,
        raw_shnum: header.e_shnum as u64,
        raw_shstrndx: header.e_shstrndx as u64,
    }

}