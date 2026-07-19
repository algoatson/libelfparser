use std::{env, fs};

use colored::*;
use comfy_table::{
    presets::UTF8_FULL,
    ContentArrangement,
    Table,
};

use libelfctf::elf::{
    ElfFile,
    SectionType,
    SymbolType,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {

    let path = "/bin/cat";

    let bytes = fs::read(&path)?;

    let elf = ElfFile::parse(&bytes)?;

    println!();
    println!(
        "{} {}",
        "ELF Parser".bright_cyan().bold(),
        "v0.1".dimmed()
    );

    println!(
        "{} {}",
        "File:".bold(),
        path.yellow()
    );


    //
    // ELF HEADER
    //
    println!();
    println!("{}", "ELF Header".bright_green().bold());

    let header = elf.header();

    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.set_header(vec![
        "Field",
        "Value",
    ]);

    table.add_row(vec![
        "Class",
        &format!("{:?}", header.class()),
    ]);

    table.add_row(vec![
        "Endian",
        &format!("{:?}", header.endianness()),
    ]);

    table.add_row(vec![
        "Type",
        &format!("{:?}", header.file_type()),
    ]);

    table.add_row(vec![
        "Machine",
        &format!("{:?}", header.machine()),
    ]);

    table.add_row(vec![
        "Entry",
        &format!("0x{:x}", header.entry()),
    ]);

    println!("{table}");



    //
    // SEGMENTS
    //
    println!();
    println!("{}", "Program Headers".bright_green().bold());


    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);


    table.set_header(vec![
        "#",
        "Type",
        "Offset",
        "Virt Addr",
        "File Size",
        "Mem Size",
        "Flags",
    ]);


    for (i, seg) in elf.segments().iter().enumerate() {

        table.add_row(vec![
            i.to_string(),
            format!("{:?}", seg.segment_type()),
            format!("0x{:x}", seg.file_offset()),
            format!("0x{:x}", seg.virtual_address()),
            seg.file_size().to_string(),
            seg.memory_size().to_string(),
            format!("{:?}", seg.flags()),
        ]);
    }


    println!("{table}");



    //
    // SECTIONS
    //
    println!();
    println!("{}", "Section Headers".bright_green().bold());


    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);


    table.set_header(vec![
        "#",
        "Name",
        "Type",
        "Address",
        "Offset",
        "Size",
    ]);


    for (i, section) in elf.sections().iter().enumerate() {

        table.add_row(vec![
            i.to_string(),

            section
                .name()
                .unwrap_or("<unknown>")
                .to_string(),

            format!("{:?}", section.section_type()),

            format!(
                "0x{:x}",
                section.virtual_address()
            ),

            format!(
                "0x{:x}",
                section.file_offset()
            ),

            section.size().to_string(),
        ]);
    }


    println!("{table}");



    //
    // SYMBOLS
    //
    println!();
    println!("{}", "Symbols".bright_green().bold());


    let mut table = Table::new();

    table
        .load_preset(UTF8_FULL)
        .set_content_arrangement(ContentArrangement::Dynamic);


    table.set_header(vec![
        "#",
        "Name",
        "Type",
        "Binding",
        "Value",
        "Size",
    ]);


    for (i, sym) in elf.symbols().iter().enumerate() {

        table.add_row(vec![
            i.to_string(),

            sym.name()
                .unwrap_or("<unknown>")
                .to_string(),

            format!("{:?}", sym.symbol_type()),

            format!("{:?}", sym.binding()),

            format!("0x{:x}", sym.value()),

            sym.size().to_string(),
        ]);
    }


    println!("{table}");

    //
    // RELOCATIONS
    //
    println!();
    println!("{}", "Relocations".bright_green().bold());


    if elf.relocations().is_empty() {

        println!(
            "{}",
            "No relocation sections found.".dimmed()
        );

    } else {


        // for each relocation section
        for (section_index, relocation_section) 
            in elf.relocations().iter().enumerate() {


            let section_name = relocation_section
                .section(elf.sections())
                .and_then(|s| s.name())
                .unwrap_or("<unknown>");


            println!();
            println!(
                "{} {}",
                format!("[{}]", section_index)
                    .bright_yellow()
                    .bold(),

                section_name
                    .bright_cyan()
            );


            let mut table = Table::new();


            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic);



            table.set_header(vec![
                "#",
                "Offset",
                "Type",
                "Symbol",
                "Addend",
            ]);



            for (i, reloc) 
                in relocation_section.relocations()
                    .iter()
                    .enumerate()
            {


                table.add_row(vec![

                    i.to_string(),

                    format!(
                        "0x{:x}",
                        reloc.offset()
                    ),


                    format!(
                        "{:?}",
                        reloc.relocation_type()
                    ),


                    reloc.symbol_index()
                        .to_string(),


                    reloc.addend()
                        .map(|x| x.to_string())
                        .unwrap_or_else(|| "-".into()),

                ]);
            }


            println!("{table}");
        }
    }

        //
    // DYNAMIC SECTION
    //
    println!();
    println!("{}", "Dynamic Section".bright_green().bold());


    match elf.dynamic() {

        Some(dynamic) => {

            let mut table = Table::new();

            table
                .load_preset(UTF8_FULL)
                .set_content_arrangement(ContentArrangement::Dynamic);


            table.set_header(vec![
                "#",
                "Tag",
                "Value",
            ]);


            for (i, entry) in dynamic.entries()
                .iter()
                .enumerate()
            {

                table.add_row(vec![

                    i.to_string(),

                    format!(
                        "{:?}",
                        entry.tag()
                    ),

                    format!(
                        "0x{:x}",
                        entry.value()
                    ),
                ]);
            }


            println!("{table}");
        }


        None => {

            println!(
                "{}",
                "No dynamic section found.".dimmed()
            );

        }
    }

    Ok(())
}