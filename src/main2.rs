use libelfctf::elf::{ElfError, ElfFile};

fn main() -> Result<(), ElfError> {
    let bytes = std::fs::read("/bin/ls").expect("penis");

    let elf = ElfFile::parse(&bytes)?;

    println!("Architecture: {:?}", elf.header().machine());

    for segment in elf.segments() {
        println!(
            "{:?} @ 0x{:x}",
            segment.segment_type(),
            segment.virtual_address()
        );
    }

    for section in elf.sections() {
        println!(
            "{}",
            section.name().unwrap_or("<unknown>")
        );
    }

    Ok(())
}