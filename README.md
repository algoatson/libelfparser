# libelfparser

A modern, safe, and extensible ELF parsing library written in Rust.

`libelfparser` provides a high-level representation of ELF binaries while preserving access to the underlying binary data. The goal is to build a clean foundation for reverse engineering tools, binary analysis frameworks, and security research tooling.

---

## Features

### Implemented

- ✅ ELF magic validation
- ✅ ELF32 and ELF64 parsing
- ✅ ELF header parsing
- ✅ Program header parsing
- ✅ Segment extraction
- ✅ Section header parsing
- ✅ Section data access
- ✅ Section name resolution through `.shstrtab`
- ✅ Safe bounds checking
- ✅ Zero-copy parsing using Rust lifetimes
- ✅ Little-endian ELF support

### In Progress

- ✅ Symbol table parsing
- ✅ Dynamic section parsing
- ✅ Relocation parsing (in progress)
- 🚧 String table abstraction
- 🚧 Architecture-specific extensions

### Planned

- ⬜ DWARF debug information support
- ⬜ ELF patching utilities
- ⬜ Disassembly integration
- ⬜ Dependency analysis
- ⬜ ELF rewriting support

---

## Design Goals

`libelfparser` is designed around a few principles:

### Zero-copy parsing

The parser does not copy binary data unnecessarily.

Parsed objects borrow from the original ELF buffer:

```rust
let elf = ElfFile::parse(&bytes)?;

for section in elf.sections() {
    println!("{:?}", section.name());
}
```

The ELF file owns the metadata while slices point directly into the original binary.

---

### Safe abstraction over raw ELF structures

Raw ELF structures mirror the Linux ELF ABI:

```rust
#[repr(C)]
pub struct Elf64_Ehdr {
    ...
}
```

but users interact with safe Rust representations:

```rust
ElfHeader
ElfSegment
ElfSection
ElfSymbol
```

This separates binary layout concerns from the public API.

---

## Example

```rust
use libelfparser::elf::ElfFile;

let bytes = std::fs::read("/bin/ls")?;

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
```

Example output:

```
Architecture: X86_64

Load @ 0x0
Load @ 0x3000
Load @ 0x1d000

.text
.rodata
.data
.bss
```

---

## Internal Architecture

The library separates ELF parsing into multiple layers:

```
ELF Binary
    |
    |
    v
+----------------+
| Raw Structures |
| Elf64_Ehdr     |
| Elf64_Phdr     |
| Elf64_Shdr     |
+----------------+
        |
        v
+----------------+
| Safe Models    |
| ElfHeader      |
| ElfSegment     |
| ElfSection     |
+----------------+
        |
        v
+----------------+
| Analysis Layer |
| Symbols        |
| Relocations    |
| Dynamic Data   |
+----------------+
```

---

## Example Structure

A parsed ELF file is represented as:

```
ElfFile
 |
 +-- ElfHeader
 |
 +-- Vec<ElfSegment>
 |       |
 |       +-- ElfProgramHeader
 |       +-- &[u8]
 |
 +-- Vec<ElfSection>
         |
         +-- ElfSectionHeader
         +-- Name
         +-- &[u8]
```

Segments and sections borrow directly from the original file buffer.

---

## Supported ELF Concepts

Currently supported:

| Feature | Status |
|---|---|
| ELF Header | ✅ |
| Program Headers | ✅ |
| Load Segments | ✅ |
| Section Headers | ✅ |
| Section Names | ✅ |
| Symbols | ✅ |
| Relocations | 🚧 |
| Dynamic Linking | 🚧 |
| DWARF | ⬜ |

---

## Building

Clone:

```bash
git clone https://github.com/<username>/libelfparser
cd libelfparser
```

Build:

```bash
cargo build
```

Run the example parser:

```bash
cargo run -- /bin/ls
```

---

## Why another ELF parser?

Existing ELF libraries are often designed around either:

- direct bindings to C libraries
- minimal parsing utilities
- compiler/toolchain use cases

`libelfparser` is focused on reverse engineering and binary analysis.

The goal is a library that feels closer to the internal representation used by tools such as:

- Binary Ninja
- Ghidra
- IDA Pro

while keeping Rust's safety guarantees.

---

## License

MIT License

---

## Author

Built for learning, reverse engineering, and binary analysis.
