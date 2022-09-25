use crate::machine_code::*;

const IMAGE_SYM_DEBUG: u16 = 0xFFFE;
const IMAGE_SYM_CLASS_FILE: u8 = 0x67;
const IMAGE_SYM_CLASS_EXTERNAL: u8 = 0x02;
const IMAGE_SYM_CLASS_STATIC: u8 = 0x03;
const IMAGE_SYM_ABSOLUTE: u16 = 0xFFFF;

fn short_named_symbol(
    name: &str,
    value: u32,
    section_number: u16,
    symbol_type: u16,
    storage_class: u8,
    number_of_auxillary_symbols: u8
) -> CoffSymbol {
    CoffSymbol { 
        short_named: {
            CoffSymbolShortNamed {
                name: get_8_padded_u8_array_from_string(name),
                value,
                section_number,
                symbol_type,
                storage_class,
                number_of_auxillary_symbols
            }
        }
    }
}

fn long_named_symbol(
    pointer_to_string_table: u32,
    value: u32,
    section_number: u16,
    symbol_type: u16,
    storage_class: u8,
    number_of_auxillary_symbols: u8
) -> CoffSymbol {
    CoffSymbol { 
        long_named: {
            CoffSymbolLongNamed {
                pad: 0,
                pointer_to_string_table,
                value,
                section_number,
                symbol_type,
                storage_class,
                number_of_auxillary_symbols
            }
        }
    }
}

fn name_symbol(name: &str) -> CoffSymbol {
    CoffSymbol { 
        name: { 
            CoffSymbolName(get_truncated_18_padded_u8_array_from_string(name))
        }
    }
}

fn section_symbol(
    length: u32,
    number_of_relocations: u16,
    number_of_line_numbers: u16,
    checksum: u32,
    number: u16,
    selection: u8
) -> CoffSymbol {
    CoffSymbol { 
        section: {
            CoffSymbolSection {
                length,
                number_of_relocations,
                number_of_line_numbers,
                checksum,
                number,
                selection,
                pad1: 0,
                pad2: 0
            }
        }
    }
}

fn add_symbol(coff: &mut Coff, entry: CoffSymbol) {
    coff.symbols.push(entry);
    coff.header.number_of_symbols += 1;
    set_current_timestamp(coff);
}

fn add_string(coff: &mut Coff, entry: &str) -> u32 {
    let mut new_string = string_to_bytes_zero_terminated(entry);
    let pointer = coff.strings_table_length;
    coff.strings_table_length += new_string.len() as u32;
    coff.strings.append(&mut new_string);
    set_current_timestamp(coff);
    pointer
}

fn add_named_symbol(
    coff: &mut Coff,
    name: &str,
    value: u32,
    section_number: u16,
    symbol_type: u16,
    storage_class: u8,
    number_of_auxillary_symbols: u8
) {
    if name.len() <= 8 {
        add_symbol(coff, short_named_symbol(name, value, section_number, symbol_type, storage_class, number_of_auxillary_symbols));
    } else {
        let name_pointer = add_string(coff, name);
        add_symbol(coff, long_named_symbol(name_pointer, value, section_number, symbol_type, storage_class, number_of_auxillary_symbols));
    }
}

pub fn add_debug_file_name_symbols(coff: &mut Coff, file_name: &str) {
    add_named_symbol(coff, ".file", 0, IMAGE_SYM_DEBUG, 0, IMAGE_SYM_CLASS_FILE, 1);
    add_symbol(coff, name_symbol(file_name));
}

fn add_section_symbols(coff: &mut Coff, section_name: &str, section_number: u16, section_length: u32, number_of_relocations: u16) {
    add_named_symbol(coff, section_name, 0, section_number, 0, IMAGE_SYM_CLASS_STATIC, 1);
    add_symbol(coff, section_symbol(section_length, number_of_relocations, 0, 0, 0, 0));
}

pub fn add_data_section_header_symbols(coff: &mut Coff) {
    let section_size = coff.data_section_header.size_of_section;
    let number_of_relocations = coff.data_section_header.number_of_relocations;
    add_section_symbols(coff, ".data", 1, section_size, number_of_relocations);
}

pub fn add_text_section_header_symbols(coff: &mut Coff) {
    let section_size = coff.text_section_header.size_of_section;
    let number_of_relocations = coff.text_section_header.number_of_relocations;
    add_section_symbols(coff, ".text", 2, section_size, number_of_relocations);
}

pub fn add_absolute_static_symbol(coff: &mut Coff, name: &str, value: u32) {
    add_static_symbol(coff, name, value, IMAGE_SYM_ABSOLUTE);
}

pub fn add_absolute_external_symbol(coff: &mut Coff, name: &str, value: u32) {
    add_external_symbol(coff, name, value, IMAGE_SYM_ABSOLUTE);
}

pub fn add_data_section_static_symbol(coff: &mut Coff, name: &str, value: u32) {
    add_static_symbol(coff, name, value, 1);
}

pub fn add_foreign_external_symbol(coff: &mut Coff, name: &str) {
    add_external_symbol(coff, name, 0, 0);
}

pub fn add_text_section_external_symbol(coff: &mut Coff, name: &str, value: u32) {
    add_external_symbol(coff, name, value, 2);
}

fn add_static_symbol(coff: &mut Coff, name: &str, value: u32, section_number: u16) {
    add_named_symbol(coff, name, value, section_number, 0, IMAGE_SYM_CLASS_STATIC, 0);
}

fn add_external_symbol(coff: &mut Coff, name: &str, value: u32, section_number: u16) {
    add_named_symbol(coff, name, value, section_number, 0, IMAGE_SYM_CLASS_EXTERNAL, 0);
}