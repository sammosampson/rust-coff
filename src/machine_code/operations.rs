use std::*;
use crate::machine_code::*;

const MOD_REGISTER_INDIRECT: u8 = 0x01;
const MOD_REGISTER_DIRECT: u8 = 0x03;
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

pub fn add_push_reg_op(coff: &mut Coff, register: u8) {
    add_entry_to_text_section(coff, OP_PUSH + register);
}

pub fn add_pop_reg_op(coff: &mut Coff, register: u8) {
    add_entry_to_text_section(coff, OP_POP + register);
}

pub fn add_sub_byte_value_from_reg_op(coff: &mut Coff, value: u8, register: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff, OP_ADD);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_DIRECT, SECONDARY_ADD_OP_SUB, register));
    add_entry_to_text_section(coff, value);
}

pub fn add_add_byte_value_to_reg_op(coff: &mut Coff, value: u8, register: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff, OP_ADD);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_DIRECT, SECONDARY_OP_NONE, register));
    add_entry_to_text_section(coff, value);
}

pub fn add_mov_dword_relocatable_value_to_reg_op(coff: &mut Coff, relocatable_value: RelocatableValue, register: u8) {
    add_entry_to_text_section(coff, OP_MOV_IMM_TO_R + register);
    add_relocatable_entry_and_text_section_inital_entry(coff, relocatable_value, IMAGE_REL_AMD64_ADDR32);
}

pub fn add_mov_dword_value_to_reg_op(coff: &mut Coff, value: u32, register: u8) {
    add_entry_to_text_section(coff, OP_MOV_IMM_TO_R + register);
    add_entries_to_text_section(coff, u32_to_bytes(&value));
}

pub fn add_mov_from_qword_reg_to_reg_op(coff: &mut Coff, register_from: u8, register_to: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_mov_from_dword_reg_to_reg_op(coff, register_from, register_to);
}

pub fn add_mov_from_dword_reg_to_reg_op(coff: &mut Coff, register_from: u8, register_to: u8) {
    add_entry_to_text_section(coff, OP_MOV_R_TO_RM);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_DIRECT, register_from, register_to));
}

pub fn add_mov_dword_value_into_reg_plus_offset_pointer_op(coff: &mut Coff, value: u32, address_register: u8, address_offset: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff,OP_MOV_IMM_TO_RM);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_INDIRECT, 0, address_register));
    add_entry_to_text_section(coff, 0x24);
    add_entry_to_text_section(coff, address_offset);
    add_entries_to_text_section(coff, u32_to_bytes(&value));
}

pub fn add_mov_dword_reg_plus_offset_pointer_to_reg_op(coff: &mut Coff, address_register: u8, address_offset: u8, into_register: u8) {
    if register_has_high_bit(into_register) {
        add_entry_to_text_section(coff, REX_R);    
    }
    add_entry_to_text_section(coff, OP_MOV_RM_TO_R);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_INDIRECT, remove_register_high_bit(into_register), address_register));
    add_entry_to_text_section(coff, address_offset);
}

pub fn add_mov_qword_reg_plus_offset_pointer_to_reg_op(coff: &mut Coff, address_register: u8, address_offset: u8, into_register: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff, OP_MOV_RM_TO_R);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_INDIRECT, into_register, address_register));
    add_entry_to_text_section(coff, address_offset);
}

pub fn add_mov_reg_to_reg_plus_offset_qword_pointer_op(coff: &mut Coff, from_register: u8, into_address_register: u8, into_address_offset: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff, OP_MOV_R_TO_RM);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_INDIRECT, from_register, into_address_register));
    add_entry_to_text_section(coff, into_address_offset);
}

pub fn add_mov_reg_to_reg_plus_offset_dword_pointer_op(coff: &mut Coff, from_register: u8, into_address_register: u8, into_address_offset: u8) {
    add_entry_to_text_section(coff, OP_MOV_R_TO_RM);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_INDIRECT, from_register, into_address_register));
    add_entry_to_text_section(coff, into_address_offset);
}

pub fn add_call_relocatable_addr_op(coff: &mut Coff, relocatable_address: RelocatableValue) {
    add_entry_to_text_section(coff, OP_CALL);
    add_relocatable_entry_and_text_section_inital_entry(coff, relocatable_address, IMAGE_REL_AMD64_REL32);
}

pub fn add_lea_reg_plus_offset_pointer_to_reg_op(coff: &mut Coff, address_register: u8, relocatable_address_offset: RelocatableValue, into_register: u8) {
    add_entry_to_text_section(coff, REX_W);
    add_entry_to_text_section(coff, OP_LEA);
    add_entry_to_text_section(coff, mod_rm(0, into_register, address_register));
    add_relocatable_entry_and_text_section_inital_entry(coff, relocatable_address_offset, IMAGE_REL_AMD64_REL32);
}

pub fn add_xor_qword_reg_into_reg_op(coff: &mut Coff, register_from: u8, register_into: u8) {
    let mut rex = REX_W | REX_B;
    if register_has_high_bit(register_from) {
        rex = rex | REX_R
    }
    add_entry_to_text_section(coff, rex);
    add_entry_to_text_section(coff, OP_XOR);
    add_entry_to_text_section(coff, mod_rm(MOD_REGISTER_DIRECT, remove_register_high_bit(register_from), remove_register_high_bit(register_into)));
}

pub fn add_ret_op(coff: &mut Coff) {
    add_entry_to_text_section(coff, OP_RET);
}