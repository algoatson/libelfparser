use std::env;
use std::fs;
use std::process;

use libelfctf::elf::ElfFile;


fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <elf-file>", args[0]);
        process::exit(1);
    }


    let path = &args[1];


    let bytes = match fs::read(path) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Failed to read {}: {}", path, e);
            process::exit(1);
        }
    };


    let elf = match ElfFile::parse(&bytes) {
        Ok(elf) => elf,
        Err(e) => {
            eprintln!("Failed to parse ELF: {:?}", e);
            process::exit(1);
        }
    };


    let header = elf.header();



    println!();
    println!("╔═══════════════════════════════════════════════╗");
    println!("║             libelfparser analyzer             ║");
    println!("╚═══════════════════════════════════════════════╝");
    println!();


    println!("File");
    println!("----");
    println!("Path:       {}", path);
    println!("Size:       {} bytes", bytes.len());


    println!();
    println!("ELF Header");
    println!("----------");

    println!("Magic:      {:02x?}", header.magic());
    println!("Class:      {:?}", header.class());
    println!("Endian:     {:?}", header.endianness());
    println!("Type:       {:?}", header.file_type());
    println!("Machine:    {:?}", header.machine());
    println!("Entry:      0x{:x}", header.entry());



    println!();
    println!("Program Headers");
    println!("----------------");


    println!(
        "{:<5} {:<18} {:<18} {:<12} {:<12}",
        "ID",
        "TYPE",
        "ADDRESS",
        "FILE SIZE",
        "MEM SIZE"
    );


    for (i, segment) in elf.segments().iter().enumerate() {

        println!(
            "{:<5} {:<18?} 0x{:016x} {:<12} {:<12}",
            i,
            segment.segment_type(),
            segment.virtual_address(),
            segment.file_size(),
            segment.memory_size()
        );

        println!(
            "      Offset: 0x{:x} Flags: {:?}",
            segment.file_offset(),
            segment.flags()
        );
    }



    println!();
    println!("Sections");
    println!("--------");


    println!(
        "{:<5} {:<25} {:<18} {:<12}",
        "ID",
        "NAME",
        "ADDRESS",
        "SIZE"
    );


    for (i, section) in elf.sections().iter().enumerate() {

        let name = section
            .name()
            .unwrap_or("<unnamed>");


        println!(
            "{:<5} {:<25} 0x{:016x} {:<12}",
            i,
            name,
            section.virtual_address(),
            section.size()
        );


        println!(
            "      Type: {:?} Flags: {:?}",
            section.section_type(),
            section.flags()
        );
    }



    println!();
    println!("Statistics");
    println!("----------");

    println!(
        "Segments: {}",
        elf.segments().len()
    );

    println!(
        "Sections: {}",
        elf.sections().len()
    );


    let executable_sections =
        elf.sections()
            .iter()
            .filter(|s| {
                format!("{:?}", s.flags())
                    .contains("EXEC")
            })
            .count();


    println!(
        "Executable sections: {}",
        executable_sections
    );


    println!();
    println!("Analysis complete.");
}