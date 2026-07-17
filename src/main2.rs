use std::fs;

use libelfctf::elf::ElfFile;

fn main() {
    let bytes = fs::read("/bin/ls")
        .expect("Failed to read ELF file");

    let elf = ElfFile::parse(&bytes)
        .expect("Failed to parse ELF file");

    let header = elf.header();

    println!("ELF Information");
    println!("===============");

    println!("Magic: {:02x?}", header.magic());

    println!("Class: {:?}", header.class());
    println!("Endianness: {:?}", header.endianness());
    println!("File type: {:?}", header.file_type());
    println!("Machine: {:?}", header.machine());

    println!();
    println!("Version: {}", header.version());

    println!("Entry point: 0x{:x}", header.entry());

    println!(
        "Program header offset: 0x{:x}",
        header.program_header_offset()
    );

    println!(
        "Section header offset: 0x{:x}",
        header.section_header_offset()
    );

    println!();

    println!("Header size: {}", header.header_size());

    println!(
        "Program headers: {} entries of {} bytes",
        header.program_header_count(),
        header.program_header_size()
    );

    println!(
        "Section headers: {} entries of {} bytes",
        header.section_header_count(),
        header.section_header_size()
    );

    println!(
        "Section name table index: {}",
        header.section_name_table_index()
    );
}