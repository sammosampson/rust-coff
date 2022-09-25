use std::*;
use std::fs::File;
use std::io::Write;
use crate::machine_code::*;

pub fn create_coff_file(name: &str) -> io::Result<File> {
    File::create(name)
}

pub fn write_coff_to_file(coff: &Coff, file: &mut File) -> io::Result<()> {
    file.write_all(any_as_u8_slice(&coff.header))?;
    file.write_all(any_as_u8_slice(&coff.data_section_header))?;
    file.write_all(any_as_u8_slice(&coff.text_section_header))?;
    file.write_all(&coff.data_section)?;
    file.write_all(&coff.text_section)?;
    for relocation in &coff.relocations {
        file.write_all(any_as_u8_slice(relocation))?;
    }
    for symbol in &coff.symbols {
        file.write_all(any_as_u8_slice(symbol))?;
    }
    file.write_all(any_as_u8_slice(&coff.strings_table_length))?;
    file.write_all(&coff.strings)?;
    file.flush()?;
    Ok(())
}