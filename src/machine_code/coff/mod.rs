mod symbols;
mod relocations;
mod headers;
mod sections;
mod files;

use crate::machine_code::*;
pub use headers::*;
pub use symbols::*;
pub use relocations::*;
pub use sections::*;
pub use files::*;


const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;

const IMAGE_SCN_CNT_INITIALISED_DATA: u32 = 0x00000040;
const IMAGE_SCN_CNT_CODE: u32 = 0x00000020;
const IMAGE_SCN_ALIGN_4BYTES: u32 = 0x00300000;
const IMAGE_SCN_ALIGN_16BYTES: u32 = 0x00500000;
const IMAGE_SCN_MEM_EXECUTE: u32 = 0x20000000;
const IMAGE_SCN_MEM_READ: u32 = 0x40000000;
const IMAGE_SCN_MEM_WRITE: u32 = 0x80000000;

pub struct Coff {
    header: CoffHeader,
    data_section_header: CoffSectionHeader,
    text_section_header: CoffSectionHeader,
    data_section: Vec<u8>,
    text_section: Vec<u8>,
    relocations: Vec<CoffRelocationEntry>,
    symbols: Vec<CoffSymbol>,    
    strings_table_length: u32,
    strings: Vec<u8>
}

#[repr(packed)]
#[allow(dead_code)]
pub struct CoffHeader {
    magic: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    flags: u16,
}

#[repr(packed)]
#[allow(dead_code)]
pub struct CoffSectionHeader {
    short_name: [u8;8],
    physical_address: u32,
    virtual_address: u32,
    size_of_section: u32,
    pointer_to_section: u32,
    pointer_to_relocations: u32,
    pointer_to_line_numbers: u32,
    number_of_relocations: u16,
    number_of_line_numbers: u16,
    flags: u32,
}

#[repr(packed)]
#[allow(dead_code)]
pub struct CoffRelocationEntry {
    pointer_to_reference: u32,
    symbol_index: u32,
    relocation_type: u16,
}

#[repr(packed)]
pub union CoffSymbol {
    short_named: CoffSymbolShortNamed,
    long_named: CoffSymbolLongNamed,
    name: CoffSymbolName,
    section: CoffSymbolSection
}

#[repr(packed)]
#[allow(dead_code)]
pub struct CoffSymbolShortNamed {
    name: [u8;8],
    value: u32,
    section_number: u16,
    symbol_type: u16,
    storage_class: u8,
    number_of_auxillary_symbols: u8,
}

#[repr(packed)]
#[allow(dead_code)]
pub struct CoffSymbolLongNamed {
    pad: u32,
    pointer_to_string_table: u32,
    value: u32,
    section_number: u16,
    symbol_type: u16,
    storage_class: u8,
    number_of_auxillary_symbols: u8,
}

#[repr(packed)]
pub struct CoffSymbolName(pub [u8;18]);

#[repr(packed)]
#[allow(dead_code)]
pub struct CoffSymbolSection {
    length: u32,
    number_of_relocations: u16,
    number_of_line_numbers: u16,
    checksum: u32,
    number: u16,
    selection: u8,
    pad1: u16,
    pad2: u8
}

pub fn create_coff() -> Coff {
    Coff {
        header : header( 
            IMAGE_FILE_MACHINE_AMD64,
            2,
            get_current_timestamp(), 
            initial_base_dynamic_data_pointer(),
            0,
            0,
            0,
        ),
        data_section_header: section_header(
            ".data",
            0,
            0,
            0,
            initial_base_dynamic_data_pointer(),
            initial_base_dynamic_data_pointer(),
            0,
            0,
            0,
            IMAGE_SCN_CNT_INITIALISED_DATA | IMAGE_SCN_ALIGN_4BYTES | IMAGE_SCN_MEM_READ | IMAGE_SCN_MEM_WRITE
        ),
        text_section_header: section_header(
            ".text",
            0,
            0,
            0,
            initial_base_dynamic_data_pointer(),
            initial_base_dynamic_data_pointer(),
            0,
            0,
            0,
            IMAGE_SCN_CNT_CODE | IMAGE_SCN_ALIGN_16BYTES | IMAGE_SCN_MEM_EXECUTE | IMAGE_SCN_MEM_READ,
        ),
        data_section: vec!(),
        text_section: vec!(),
        relocations: vec!(),
        symbols: vec!(),
        strings_table_length: 0x4,
        strings: vec!()
    }
}
