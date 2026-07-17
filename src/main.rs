use std::fs;

use libelfctf::elf::ElfFile;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bytes = fs::read("/bin/ls")?;

    let elf = ElfFile::parse(&bytes)?;

    let header = elf.header();

    println!("ELF Header:");
    println!("  Class:       {:?}", header.class());
    println!("  Endianness:  {:?}", header.endianness());
    println!("  Type:        {:?}", header.file_type());
    println!("  Machine:     {:?}", header.machine());
    println!("  Entry:       0x{:x}", header.entry());


    println!();
    println!("Program Headers:");

    for (index, segment) in elf.segments().iter().enumerate() {
        println!();
        println!("  [{}] {:?}", index, segment.header().segment_type());

        println!(
            "      Offset:          0x{:x}",
            segment.file_offset()
        );

        println!(
            "      Virtual Address: 0x{:x}",
            segment.virtual_address()
        );

        println!(
            "      Physical Address: 0x{:x}",
            segment.physical_address()
        );

        println!(
            "      File Size:       {}",
            segment.file_size()
        );

        println!(
            "      Memory Size:     {}",
            segment.memory_size()
        );

        println!(
            "      Flags:           {:?}",
            segment.flags()
        );

        println!(
            "      Alignment:       {}",
            segment.alignment()
        );
    }


    println!();
    println!("Section Headers:");

    for (index, shdr) in elf.sections().iter().enumerate() {
        println!();
        println!("  [{}]", index);

        println!(
            "      Name:       0x{:x}",
            shdr.name_offset()
        );

        println!(
            "      Type:       {:?}",
            shdr.section_type()
        );

        println!(
            "      Address:    0x{:x}",
            shdr.virtual_address()
        );

        println!(
            "      Offset:     0x{:x}",
            shdr.file_offset()
        );

        println!(
            "      Size:       {}",
            shdr.size()
        );

        println!(
            "      Flags:      {:?}",
            shdr.flags()
        );

        println!(
            "      Alignment:  {}",
            shdr.alignment()
        );

        println!(
            "      Entry Size: {}",
            shdr.entry_size()
        );
    }


    Ok(())
}