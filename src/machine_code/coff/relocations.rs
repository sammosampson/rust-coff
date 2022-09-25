use std::mem::size_of;
use crate::machine_code::*;

pub const IMAGE_REL_AMD64_ADDR32: u16 = 0x02;
pub const IMAGE_REL_AMD64_REL32: u16 = 0x04;

fn relocation_entry(
    pointer_to_reference: u32,
    symbol_index: u32,
    relocation_type: u16
) -> CoffRelocationEntry {
    CoffRelocationEntry {
        pointer_to_reference,
        symbol_index,
        relocation_type
    }
}

pub struct RelocatableValue { 
    symbol_index: u32,
    initial_value_to_use: u32
}

pub fn relocatable_value(symbol_index: u32, initial_value_to_use: u32) -> RelocatableValue {
    RelocatableValue { symbol_index, initial_value_to_use }
}

fn add_relocation_entry(coff: &mut Coff, entry: CoffRelocationEntry) {
    coff.relocations.push(entry);
    coff.text_section_header.number_of_relocations += 1;
    coff.header.pointer_to_symbol_table += size_of::<CoffRelocationEntry>() as u32;
    set_current_timestamp(coff);
}

pub fn add_relocatable_entry_and_text_section_inital_entry(coff: &mut Coff, relocatable_value: RelocatableValue, relocation_type: u16) { 
    add_relocation_entry(
        coff, 
        relocation_entry(
            coff.text_section_header.size_of_section, 
            relocatable_value.symbol_index, relocation_type
        )
    );
    add_entries_to_text_section(coff, u32_to_bytes(&relocatable_value.initial_value_to_use));
}