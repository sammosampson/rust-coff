#![feature(untagged_unions)]

use std::*;
use std::fs::File;
use std::io::Write;
use std::mem::size_of;

fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe { 
        slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }
}

fn get_8_padded_u8_array_from_string(from: &str) -> [u8; 8] {
    assert!(from.len() <= 8);
    
    let mut to = [0; 8];
    to[..from.len()].copy_from_slice(&from.as_bytes());
    to
}

fn get_18_padded_u8_array_from_string(from: &str) -> [u8; 18] {
    assert!(from.len() <= 18);
    
    let mut to = [0; 18];
    to[..from.len()].copy_from_slice(&from.as_bytes());
    to
}

fn string_to_bytes_zero_terminated(entry: &str) -> Vec<u8> {
    let mut new_string = string_to_bytes(entry);
    new_string.push(0x0);
    new_string
}

fn string_to_bytes(entry: &str) -> Vec<u8> {
    entry.as_bytes().into()
}

fn u32_to_bytes(entry: &u32) -> Vec<u8> {
    any_as_u8_slice(entry).into()
}

const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;

const IMAGE_SCN_CNT_INITIALISED_DATA: u32 = 0x00000040;
const IMAGE_SCN_CNT_CODE: u32 = 0x00000020;
const IMAGE_SCN_ALIGN_4BYTES: u32 = 0x00300000;
const IMAGE_SCN_ALIGN_16BYTES: u32 = 0x00500000;
const IMAGE_SCN_MEM_EXECUTE: u32 = 0x20000000;
const IMAGE_SCN_MEM_READ: u32 = 0x40000000;
const IMAGE_SCN_MEM_WRITE: u32 = 0x80000000;

const IMAGE_SYM_DEBUG: u16 = 0xFFFE;
const IMAGE_SYM_CLASS_FILE: u8 = 0x67;
const IMAGE_SYM_CLASS_EXTERNAL: u8 = 0x02;
const IMAGE_SYM_CLASS_STATIC: u8 = 0x03;
const IMAGE_SYM_ABSOLUTE: u16 = 0xFFFF;

struct Coff {
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
struct CoffHeader {
    magic: u16,
    number_of_sections: u16,
    time_date_stamp: u32, // seconds since 1970-01-01 00:00:00 GMT
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    flags: u16,
}

fn header(
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

#[repr(packed)]
#[allow(dead_code)]
struct CoffSectionHeader {
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

fn section_header(
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

#[repr(packed)]
#[allow(dead_code)]
struct CoffRelocationEntry {
    pointer_to_reference: u32,
    symbol_index: u32,
    relocation_type: u16,
}

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

#[repr(packed)]
union CoffSymbol {
    short_named: CoffSymbolShortNamed,
    long_named: CoffSymbolLongNamed,
    name: CoffSymbolName,
    section: CoffSymbolSection
}

#[repr(packed)]
#[allow(dead_code)]
struct CoffSymbolShortNamed {
    name: [u8;8],
    value: u32,
    section_number: u16,
    symbol_type: u16,
    storage_class: u8,
    number_of_auxillary_symbols: u8,
}

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

#[repr(packed)]
#[allow(dead_code)]
struct CoffSymbolLongNamed {
    pad: u32,
    pointer_to_string_table: u32,
    value: u32,
    section_number: u16,
    symbol_type: u16,
    storage_class: u8,
    number_of_auxillary_symbols: u8,
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

#[repr(packed)]
struct CoffSymbolName([u8;18]);

fn name_symbol(name: &str) -> CoffSymbol {
    CoffSymbol { 
        name: {
            CoffSymbolName(get_18_padded_u8_array_from_string(name))
        }
    }
}

#[repr(packed)]
#[allow(dead_code)]
struct CoffSymbolSection {
    length: u32,
    number_of_relocations: u16,
    number_of_line_numbers: u16,
    checksum: u32,
    number: u16,
    selection: u8,
    pad1: u16,
    pad2: u8
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

fn initial_base_dynamic_data_pointer() -> u32 {
    (size_of::<CoffHeader>() + (size_of::<CoffSectionHeader>() * 2)) as u32
}

fn create_coff() -> Coff {
    Coff {
        header : header( 
            IMAGE_FILE_MACHINE_AMD64,
            2,
            0x631F038B, 
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



struct RelocatableValue { 
    symbol_index: u32,
    initial_value_to_use: u32
}

fn relocatable_value(symbol_index: u32, initial_value_to_use: u32) -> RelocatableValue {
    RelocatableValue { symbol_index, initial_value_to_use }
}

fn add_relocation_entry(coff: &mut Coff, entry: CoffRelocationEntry) {
    coff.relocations.push(entry);
    coff.text_section_header.number_of_relocations += 1;
    coff.header.pointer_to_symbol_table += size_of::<CoffRelocationEntry>() as u32;
}

fn add_relocatable_entry_and_text_section_inital_entry(coff: &mut Coff, relocatable_value: RelocatableValue, relocation_type: u16) { 
    add_relocation_entry(
        coff, 
        relocation_entry(
            coff.text_section_header.size_of_section, 
            relocatable_value.symbol_index, relocation_type
        )
    );
    add_entries_to_text_section(coff, u32_to_bytes(&relocatable_value.initial_value_to_use));
}

fn advance_data_section(coff: &mut Coff, amount: u32) {
    coff.data_section_header.size_of_section += amount;
    coff.data_section_header.pointer_to_relocations += amount;
    coff.text_section_header.pointer_to_section += amount;
    coff.text_section_header.pointer_to_relocations += amount;
    coff.header.pointer_to_symbol_table += amount;
}

fn add_string_to_data_section(coff: &mut Coff, to_add: &str) -> u32 {
    let pointer = coff.data_section_header.size_of_section; 
    let mut string_bytes = string_to_bytes(to_add);
    advance_data_section(coff, string_bytes.len() as u32);
    coff.data_section.append(&mut string_bytes);
    pointer
}

fn add_entry_to_text_section(coff: &mut Coff, entry: u8) {
    coff.text_section.push(entry);
    coff.text_section_header.size_of_section += 1;
    coff.text_section_header.pointer_to_relocations += 1;
    coff.header.pointer_to_symbol_table += 1;
}

fn add_entries_to_text_section(coff: &mut Coff, entries: Vec<u8>) {
    for entry in entries {
        add_entry_to_text_section(coff, entry);
    }
}

fn add_symbol(coff: &mut Coff, entry: CoffSymbol) {
    coff.symbols.push(entry);
    coff.header.number_of_symbols += 1;
}

fn add_string(coff: &mut Coff, entry: &str) -> u32 {
    let mut new_string = string_to_bytes_zero_terminated(entry);
    let pointer = coff.strings_table_length;
    coff.strings_table_length += new_string.len() as u32;
    coff.strings.append(&mut new_string);
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

fn add_debug_file_name_symbols(coff: &mut Coff, file_name: &str) {
    add_named_symbol(coff, ".file", 0, IMAGE_SYM_DEBUG, 0, IMAGE_SYM_CLASS_FILE, 1);
    add_symbol(coff, name_symbol(file_name));
}

fn add_section_symbols(coff: &mut Coff, section_name: &str, section_number: u16, section_length: u32, number_of_relocations: u16) {
    add_named_symbol(coff, section_name, 0, section_number, 0, IMAGE_SYM_CLASS_STATIC, 1);
    add_symbol(coff, section_symbol(section_length, number_of_relocations, 0, 0, 0, 0));
}

fn add_data_section_header_symbols(coff: &mut Coff) {
    let section_size = coff.data_section_header.size_of_section;
    let number_of_relocations = coff.data_section_header.number_of_relocations;
    add_section_symbols(coff, ".data", 1, section_size, number_of_relocations);
}

fn add_text_section_header_symbols(coff: &mut Coff) {
    let section_size = coff.text_section_header.size_of_section;
    let number_of_relocations = coff.text_section_header.number_of_relocations;
    add_section_symbols(coff, ".text", 2, section_size, number_of_relocations);
}

fn add_absolute_static_symbol(coff: &mut Coff, name: &str, value: u32) {
    add_static_symbol(coff, name, value, IMAGE_SYM_ABSOLUTE);
}

fn add_data_section_static_symbol(coff: &mut Coff, name: &str, value: u32) {
    add_static_symbol(coff, name, value, 1);
}

fn add_text_section_static_symbol(coff: &mut Coff, name: &str, value: u32) {
    add_static_symbol(coff, name, value, 2);
}

fn add_foreign_external_symbol(coff: &mut Coff, name: &str) {
    add_external_symbol(coff, name, 0, 0);
}

fn add_text_section_external_symbol(coff: &mut Coff, name: &str, value: u32) {
    add_external_symbol(coff, name, value, 2);
}

fn add_static_symbol(coff: &mut Coff, name: &str, value: u32, section_number: u16) {
    add_named_symbol(coff, name, value, section_number, 0, IMAGE_SYM_CLASS_STATIC, 0);
}

fn add_external_symbol(coff: &mut Coff, name: &str, value: u32, section_number: u16) {
    add_named_symbol(coff, name, value, section_number, 0, IMAGE_SYM_CLASS_EXTERNAL, 0);
}

fn create_coff_file() -> io::Result<File> {
    File::create("coff.obj")
}

fn write_coff_to_file(coff: &Coff, file: &mut File) -> io::Result<()> {
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

fn main() {
    let mut coff = create_coff();

    let print_pointer = coff.text_section_header.size_of_section;

    //print
    // fn prologue    
    add_push_reg_op(&mut coff, REG_RBP);
    add_mov_from_qword_reg_to_reg_op(&mut coff, REG_RSP, REG_RBP);

    //store args 1 and 2 in shadow
    add_mov_reg_to_reg_plus_offset_qword_pointer_op(&mut coff, REG_RCX, REG_RBP, 16);
    add_mov_reg_to_reg_plus_offset_dword_pointer_op(&mut coff, REG_EDX, REG_RBP, 24);

    //resesrve space for 1 local var (8 bytes)    
    add_sub_byte_value_from_reg_op(&mut coff, 8, REG_RSP);

    // call to GetStdHandle
    // resesrve shadow space for call to GetStdHandle
    add_sub_byte_value_from_reg_op(&mut coff, 32, REG_RSP);

    // set first arg (STD_OUTPUT_HANDLE) for call to GetStdHandle
    add_mov_dword_value_to_reg_op(&mut coff, 0xFFFFFFF5, REG_ECX);

    // call GetStdHandle
    add_call_relocatable_addr_op(&mut coff, relocatable_value(0x08, 0x0));
    
    // release shadow space for call to GetStdHandle
    add_add_byte_value_to_reg_op(&mut coff, 32, REG_RSP);
    
    // store local variable handle returned
    add_mov_reg_to_reg_plus_offset_dword_pointer_op(&mut coff, REG_EAX, REG_RBP, 0xF8);

    // call to WriteFile
    // resesrve space for 5 args, shadow + 1    
    add_sub_byte_value_from_reg_op(&mut coff, 40, REG_RSP);

    // get values for args for call from storage
    add_mov_dword_reg_plus_offset_pointer_to_reg_op(&mut coff, REG_RBP, 0xF8, REG_ECX);
    add_mov_qword_reg_plus_offset_pointer_to_reg_op(&mut coff, REG_RBP, 16, REG_RDX);
    add_mov_dword_reg_plus_offset_pointer_to_reg_op(&mut coff, REG_RBP, 24, REG_R8);
    add_xor_qword_reg_into_reg_op(&mut coff, REG_R9, REG_R9);
    add_mov_dword_value_into_reg_plus_offset_pointer_op(&mut coff, 0x0, REG_RSP, 32);

    // call WriteFile
    add_call_relocatable_addr_op(&mut coff, relocatable_value(0x07, 0x0));

    // release space for 5 args, shadow + 1    
    add_add_byte_value_to_reg_op(&mut coff, 40, REG_RSP);
    
    // fn epilogue    
    add_mov_from_qword_reg_to_reg_op(&mut coff, REG_RBP, REG_RSP);
    add_pop_reg_op(&mut coff, REG_RBP);

    add_ret_op(&mut coff);

    let main_pointer = coff.text_section_header.size_of_section;

    //main:
    // fn prologue    
    add_push_reg_op(&mut coff, REG_RBP);
    add_mov_from_qword_reg_to_reg_op(&mut coff, REG_RSP, REG_RBP);

    // print call:
    // set shadow space for print call
    add_sub_byte_value_from_reg_op(&mut coff, 32, REG_RSP);
    
    let hello = "Hello world!\r\n\0";
    let ds0_pointer = add_string_to_data_section(&mut coff, hello);
    
    // set pointer to hello world first arg for print call
    add_lea_reg_plus_offset_pointer_to_reg_op(
        &mut coff, 
        REG_RIP, 
        relocatable_value(0x02, ds0_pointer), 
        REG_RCX
    );
    
    // set hello world length second arg for print call
    add_mov_dword_value_to_reg_op(&mut coff, 0x0F, REG_EDX);
    
    //call print
    add_call_addr_op(&mut coff, 0xFFFFFF9A);
    
    // release shadow space for print call    
    add_add_byte_value_to_reg_op(&mut coff, 32, REG_RSP);

    // fn epilogue    
    add_mov_from_qword_reg_to_reg_op(&mut coff, REG_RBP, REG_RSP);
    add_pop_reg_op(&mut coff, REG_RBP);

    add_ret_op(&mut coff);   
       
    add_debug_file_name_symbols(&mut coff, "hello1.asm");
    add_data_section_header_symbols(&mut coff);    
    add_text_section_header_symbols(&mut coff);
    add_absolute_static_symbol(&mut coff, ".absolut", 0);
    add_foreign_external_symbol(&mut coff, "WriteFile");
    add_foreign_external_symbol(&mut coff, "GetStdHandle");
    add_absolute_static_symbol(&mut coff, "STD_OUTPUT_HANDLE", 0xFFFFFFF5);
    add_data_section_static_symbol(&mut coff, "ds0", ds0_pointer);
    add_text_section_static_symbol(&mut coff, "print", print_pointer);
    add_text_section_external_symbol(&mut coff, "main", main_pointer);
    
    write_coff_to_file(&coff, &mut create_coff_file().unwrap()).unwrap();
}

const MOD_REGISTER_INDIRECT: u8 = 0x01;
const MOD_REGISTER_DIRECT: u8 = 0x03;
const REG_EAX: u8 = 0x00;
const REG_ECX: u8 = 0x01;
const REG_RCX: u8 = REG_ECX;
const REG_EDX: u8 = 0x02;
const REG_RDX: u8 = REG_EDX;
const REG_RSP: u8 = 0x04;
const REG_RBP: u8 = 0x05;
const REG_RIP: u8 = 0x05;
const REG_R8: u8 = 0x08; // plus B bit to make 100
const REG_R9: u8 = 0x09; // plus B bit to make 101    
const REX_B: u8 = 0x41;
const REX_R: u8 = 0x44;
const REX_W: u8 = 0x48;
const OP_ADD: u8 = 0x83;
const OP_LEA: u8 = 0x8D;
const OP_XOR: u8 = 0x31;
const OP_PUSH: u8 = 0x50;
const OP_POP: u8 = 0x58;
const OP_MOV_R_TO_RM: u8 = 0x89;
const OP_MOV_RM_TO_R: u8 = 0x8B;
const OP_MOV_IMM_TO_R: u8 = 0xB8;
const OP_MOV_IMM_TO_RM: u8 = 0xC7;
const OP_CALL: u8 = 0xE8;
const OP_RET: u8 = 0xC3;

const SECONDARY_ADD_OP_SUB: u8 = 0x5;
const SECONDARY_OP_NONE: u8 = 0x0;

fn mod_rm(mod_part: u8, reg_part: u8, r_m_part: u8) -> u8 {
    mod_part << 6 | reg_part << 3 | r_m_part
}

fn register_has_high_bit(register: u8) -> bool {
    register & 0x8 == 0x8
}

fn remove_register_high_bit(register: u8) -> u8 {
    register & 0x7
}

fn add_push_reg_op(coff: &mut Coff, register: u8) {
    add_entry_to_text_section(coff, OP_PUSH + register);
}

fn add_pop_reg_op(coff: &mut Coff, register: u8) {
    add_entry_to_text_section(coff, OP_POP + register);
}

fn add_sub_byte_value_from_reg_op(coff: &mut Coff, value: u8, register: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff, OP_ADD);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_DIRECT, SECONDARY_ADD_OP_SUB, register));
    add_entry_to_text_section(coff, value);
}

fn add_add_byte_value_to_reg_op(coff: &mut Coff, value: u8, register: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff, OP_ADD);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_DIRECT, SECONDARY_OP_NONE, register));
    add_entry_to_text_section(coff, value);
}

fn add_mov_dword_value_to_reg_op(coff: &mut Coff, value: u32, register: u8) {
    add_entry_to_text_section(coff, OP_MOV_IMM_TO_R + register);
    add_entries_to_text_section(coff, u32_to_bytes(&value));
}

fn add_mov_from_qword_reg_to_reg_op(coff: &mut Coff, register_from: u8, register_to: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_mov_from_dword_reg_to_reg_op(coff, register_from, register_to);
}

fn add_mov_from_dword_reg_to_reg_op(coff: &mut Coff, register_from: u8, register_to: u8) {
    add_entry_to_text_section(coff, OP_MOV_R_TO_RM);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_DIRECT, register_from, register_to));
}

fn add_mov_dword_value_into_reg_plus_offset_pointer_op(coff: &mut Coff, value: u32, address_register: u8, address_offset: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff,OP_MOV_IMM_TO_RM);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_INDIRECT, 0, address_register));
    add_entry_to_text_section(coff, 0x24);
    add_entry_to_text_section(coff, address_offset);
    add_entries_to_text_section(coff, u32_to_bytes(&value));

}

fn add_mov_dword_reg_plus_offset_pointer_to_reg_op(coff: &mut Coff, address_register: u8, address_offset: u8, into_register: u8) {
    if register_has_high_bit(into_register) {
        add_entry_to_text_section(coff, REX_R);    
    }
    add_entry_to_text_section(coff, OP_MOV_RM_TO_R);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_INDIRECT, remove_register_high_bit(into_register), address_register));
    add_entry_to_text_section(coff, address_offset);
}

fn add_mov_qword_reg_plus_offset_pointer_to_reg_op(coff: &mut Coff, address_register: u8, address_offset: u8, into_register: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff, OP_MOV_RM_TO_R);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_INDIRECT, into_register, address_register));
    add_entry_to_text_section(coff, address_offset);
}

fn add_mov_reg_to_reg_plus_offset_qword_pointer_op(coff: &mut Coff, from_register: u8, into_address_register: u8, into_address_offset: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff, OP_MOV_R_TO_RM);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_INDIRECT, from_register, into_address_register));
    add_entry_to_text_section(coff, into_address_offset);
}

fn add_mov_reg_to_reg_plus_offset_dword_pointer_op(coff: &mut Coff, from_register: u8, into_address_register: u8, into_address_offset: u8) {
    add_entry_to_text_section(coff, OP_MOV_R_TO_RM);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_INDIRECT, from_register, into_address_register));
    add_entry_to_text_section(coff, into_address_offset);
}

fn add_call_relocatable_addr_op(coff: &mut Coff, relocatable_address: RelocatableValue) {
    add_entry_to_text_section(coff, OP_CALL);
    add_relocatable_entry_and_text_section_inital_entry(coff, relocatable_address, 0x04);
}

fn add_call_addr_op(coff: &mut Coff, address: u32) {
    add_entry_to_text_section(coff, OP_CALL);
    add_entries_to_text_section(coff, u32_to_bytes(&address));
}

fn add_lea_reg_plus_offset_pointer_to_reg_op(coff: &mut Coff, address_register: u8, relocatable_address_offset: RelocatableValue, into_register: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff, OP_LEA);
    add_entry_to_text_section(coff, mod_rm(0, into_register, address_register));
    add_relocatable_entry_and_text_section_inital_entry(coff, relocatable_address_offset, 0x04);
}

fn add_xor_qword_reg_into_reg_op(coff: &mut Coff, register_from: u8, register_into: u8) {
    let mut rex = REX_W | REX_B;
    if register_has_high_bit(register_from) {
        rex = rex | REX_R
    }
    add_entry_to_text_section(coff, rex);
    add_entry_to_text_section(coff, OP_XOR);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_DIRECT, remove_register_high_bit(register_from), remove_register_high_bit(register_into)));
}

fn add_ret_op(coff: &mut Coff) {
    add_entry_to_text_section(coff, OP_RET);
}