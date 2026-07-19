use super::error::ElfError;

pub fn get_string(table: &[u8], offset: u32) -> Result<&str, ElfError> {
    let offset = offset as usize;

    if offset >= table.len() {
        return Err(ElfError::InvalidStringOffset);
    }

    let string = &table[offset..];

    let end = string
        .iter()
        .position(|&c| c == 0)
        .ok_or(ElfError::InvalidString)?;

    std::str::from_utf8(&string[..end])
        .map_err(|_| ElfError::InvalidString)
}