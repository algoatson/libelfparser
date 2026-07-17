use std::fmt;

#[derive(Debug)]
pub enum ElfError {
    InvalidMagic,
    UnknownClass(u8),
    UnexpectedEOF,
    InvalidSectionIndex,
    InvalidOffset,
    InvalidString,
    InvalidStringOffset,
}

impl fmt::Display for ElfError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ElfError::InvalidMagic => {
                write!(f, "invalid magic bytes")
            }
            
            ElfError::UnexpectedEOF => {
                write!(f, "unexpected end of file")
            }

            ElfError::UnknownClass(u8) => {
                write!(f, "unknown ELF class")
            }

            ElfError::InvalidSectionIndex => {
                write!(f, "invalid section index")
            }
            
            ElfError::InvalidOffset => {
                write!(f, "invalid offset")
            }

            ElfError::InvalidString => {
                write!(f, "invalid string")
            }

            ElfError::InvalidStringOffset => {
                write!(f, "invalid string offset")
            }
        }
    }
}

impl std::error::Error for ElfError {}