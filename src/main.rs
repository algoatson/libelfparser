use std::fs;

use clap::Parser;
use comfy_table::{
    presets::UTF8_FULL,
    Table,
};

use owo_colors::OwoColorize;

use libelfctf::elf::ElfFile;


#[derive(Parser)]
#[command(
    name="elfparser",
    about="A Rust ELF inspection tool"
)]
struct Args {

    /// ELF file to inspect
    file: String,

}


fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();

    let bytes = fs::read(&args.file)?;

    let elf = ElfFile::parse(&bytes)?;


    println!();
    println!("{}", "╭──────────────────────────────╮".cyan());
    println!("{}", "│          ELF FILE            │".cyan());
    println!("{}", "╰──────────────────────────────╯".cyan());
    println!();


    let header = elf.header();


    println!(
        "{} {}",
        "File:".bold(),
        args.file
    );

    println!(
        "{} {} bytes",
        "Size:".bold(),
        bytes.len()
    );

    println!();


    println!(
        "{} {} {} {}",
        format!("{:?}", header.class()).green(),
        format!("{:?}", header.machine()).green(),
        format!("{:?}", header.endianness()).green(),
        format!("{:?}", header.file_type()).green(),
    );


    println!(
        "{} 0x{:x}",
        "Entry:".bold(),
        header.entry()
    );


    println!();


    //
    // PROGRAM HEADERS
    //

    println!("{}", "PROGRAM HEADERS".yellow().bold());

    let mut table = Table::new();

    table.load_preset(UTF8_FULL);

    table.set_header(vec![
        "#",
        "Type",
        "Offset",
        "Virt Addr",
        "File Size",
        "Memory",
        "Flags",
    ]);


    for (i, segment) in elf.segments().iter().enumerate() {

        table.add_row(vec![
            i.to_string(),
            format!("{:?}", segment.segment_type()),
            format!("0x{:x}", segment.file_offset()),
            format!("0x{:x}", segment.virtual_address()),
            segment.file_size().to_string(),
            segment.memory_size().to_string(),
            format!("{}", segment.flags()),
        ]);

    }


    println!("{}", table);

    println!();


    //
    // SECTIONS
    //

    println!("{}", "SECTIONS".yellow().bold());


    let mut table = Table::new();

    table.load_preset(UTF8_FULL);


    table.set_header(vec![
        "#",
        "Name",
        "Type",
        "Address",
        "Offset",
        "Size",
        "Flags",
    ]);


    for (i, section) in elf.sections().iter().enumerate() {

        table.add_row(vec![
            i.to_string(),

            section
                .name()
                .unwrap_or("<unknown>")
                .to_string(),

            format!("{:?}", section.section_type()),

            format!("0x{:x}",
                section.virtual_address()),

            format!("0x{:x}",
                section.file_offset()),

            section.size().to_string(),

            format!("{:?}",
                section.flags()),
        ]);

    }


    println!("{}", table);


    println!();
    println!(
        "{} parsed successfully",
        "✓".green()
    );


    Ok(())
}