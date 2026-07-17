# libelfparser

```
ElfFile
│
├── ElfHeader
│
├── Vec<ElfSegment<'a>>
│      │
│      ├── ElfProgramHeader
│      └── &'a [u8]  (segment contents)
│
└── Vec<ElfSection<'a>>
       │
       ├── ElfSectionHeader
       ├── Option<&'a str> (resolved name)
       └── &'a [u8] (section contents)
```

✅ ELF magic validation
✅ ELF32/ELF64 dispatch
✅ ELF header abstraction
✅ Program header parsing
✅ Section header parsing
✅ Section name resolution through .shstrtab
✅ Proper lifetime handling (&'a [u8] ownership model)
✅ Bounds checking instead of blindly indexing
✅ Abstraction away from raw Elf64_* structs
✅ Public API design (ElfFile, ElfSection, ElfProgram)
✅ Human-friendly output tooling