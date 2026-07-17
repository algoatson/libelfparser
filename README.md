# LIBELFCTF

Let's start by writing a similar interface to what we have in our C++ libelfinspect project.
We had a ByteSource parent class, and FileByteSource, CoreDumpByteSource, MemoryByteSource,...


Introduce a trait.

For example, your C++ version had:

Traits::Ehdr
Traits::Phdr
Traits::Shdr

You can do something similar in Rust.

Imagine a trait:

trait RawElfHeader {
    fn ident(&self) -> &[u8; 16];
    fn file_type(&self) -> u16;
    fn machine(&self) -> u16;
    fn version(&self) -> u32;
    fn entry(&self) -> u64;
    ...
}

Then implement it for both:

impl RawElfHeader for Elf32_Ehdr { ... }

impl RawElfHeader for Elf64_Ehdr { ... }

Now you only write one constructor:

pub(crate) fn from<T: RawElfHeader>(raw: &T) -> Self {
    ...
}

This is probably where you'll end up if you continue growing the library.