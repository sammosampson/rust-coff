use crate::machine_code::*;

fn advance_data_section(coff: &mut Coff, amount: u32) {
    coff.data_section_header.size_of_section += amount;
    coff.data_section_header.pointer_to_relocations += amount;
    coff.text_section_header.pointer_to_section += amount;
    coff.text_section_header.pointer_to_relocations += amount;
    coff.header.pointer_to_symbol_table += amount;
    set_current_timestamp(coff);
}

pub fn add_string_to_data_section(coff: &mut Coff, to_add: &str) -> u32 {
    let pointer = coff.data_section_header.size_of_section; 
    let mut string_bytes = string_to_bytes(to_add);
    advance_data_section(coff, string_bytes.len() as u32);
    coff.data_section.append(&mut string_bytes);
    pointer
}

pub fn add_entry_to_text_section(coff: &mut Coff, entry: u8) {
    coff.text_section.push(entry);
    coff.text_section_header.size_of_section += 1;
    coff.text_section_header.pointer_to_relocations += 1;
    coff.header.pointer_to_symbol_table += 1;
    set_current_timestamp(coff);
}

pub fn add_entries_to_text_section(coff: &mut Coff, entries: Vec<u8>) {
    for entry in entries {
        add_entry_to_text_section(coff, entry);
    }
}

pub fn get_current_text_section_pointer(coff: &Coff) -> u32 {
    coff.text_section_header.size_of_section
}