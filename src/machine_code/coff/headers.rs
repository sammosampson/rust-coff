use std::mem::size_of;
use crate::machine_code::*;

pub fn initial_base_dynamic_data_pointer() -> u32 {
    (size_of::<CoffHeader>() + (size_of::<CoffSectionHeader>() * 2)) as u32
}

pub fn header(
    magic: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    flags: u16,
) -> CoffHeader {
    CoffHeader {
        magic,
        number_of_sections,
        time_date_stamp,
        pointer_to_symbol_table,
        number_of_symbols,
        size_of_optional_header,
        flags,
    }
}

pub fn section_header(
    short_name: &str,
    physical_address: u32,
    virtual_address: u32,
    size_of_section: u32,
    pointer_to_section: u32,
    pointer_to_relocations: u32,
    pointer_to_line_numbers: u32,
    number_of_relocations: u16,
    number_of_line_numbers: u16,
    flags: u32,
) -> CoffSectionHeader {
    CoffSectionHeader {
        short_name: get_8_padded_u8_array_from_string(short_name),
        physical_address,
        virtual_address,
        size_of_section,
        pointer_to_section,
        pointer_to_relocations,
        pointer_to_line_numbers,
        number_of_relocations,
        number_of_line_numbers,
        flags,
    }
}

pub fn set_current_timestamp(coff: &mut Coff) {
    coff.header.time_date_stamp = get_current_timestamp();
}