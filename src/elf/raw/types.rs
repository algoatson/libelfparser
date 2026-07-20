use super::ElfTypes;
use super::{Elf32_Ehdr, Elf64_Ehdr};
use super::{Elf32_Phdr, Elf64_Phdr};
use super::{Elf32_Shdr, Elf64_Shdr};
use super::{Elf32_Sym, Elf64_Sym};
use super::{Elf32_Rel, Elf64_Rel};
use super::{Elf32_Rela, Elf64_Rela};
use super::{Elf32_Dyn, Elf64_Dyn};

pub(crate) struct Elf32Types;

impl ElfTypes for Elf32Types {
    type Header = Elf32_Ehdr;
    type ProgramHeader = Elf32_Phdr;
    type SectionHeader = Elf32_Shdr;
    type Symbol = Elf32_Sym;
    type Rel = Elf32_Rel;
    type Rela = Elf32_Rela;
    type Dynamic = Elf32_Dyn;
}

pub(crate) struct Elf64Types;

impl ElfTypes for Elf64Types {
    type Header = Elf64_Ehdr;
    type ProgramHeader = Elf64_Phdr;
    type SectionHeader = Elf64_Shdr;
    type Symbol = Elf64_Sym;
    type Rel = Elf64_Rel;
    type Rela = Elf64_Rela;
    type Dynamic = Elf64_Dyn;
}
