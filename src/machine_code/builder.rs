use super::Coff;

use crate::{
    machine_code::*,
    intemediate_representation::*
};

pub fn build_machine_code_object(coff: &mut Coff, ir: IntermediateRepresentation) -> String {
    for op in ir.byte_code  {
        match op {
            ByteCodeInstruction::CallToSymbol(symbol_index) => 
                add_call_relocatable_addr_op(
                    coff, 
                    relocatable_value(convert_byte_code_to_coff_symbol_index(ir.symbols.len(), symbol_index), 
                    0x0)
                ),
            ByteCodeInstruction::PushReg64(register) => 
                add_push_reg_op(coff, get_register(register)),
            ByteCodeInstruction::PopReg64(register) =>
                add_pop_reg_op(coff, get_register(register)),    
            ByteCodeInstruction::MoveSymbolToReg32 { symbol_index, to } => 
                add_mov_dword_relocatable_value_to_reg_op(
                    coff,
                    relocatable_value(convert_byte_code_to_coff_symbol_index(ir.symbols.len(), symbol_index), 0x0), 
                    get_register(to)
                ),      
            ByteCodeInstruction::MoveValueToReg32 { value, to } => 
                add_mov_dword_value_to_reg_op(coff, value, get_register(to)),      
            ByteCodeInstruction::MoveRegToReg64 { from, to } => 
                add_mov_from_qword_reg_to_reg_op(coff, get_register(from), get_register(to)),
            ByteCodeInstruction::MoveValueToRegPlusOffset32 { value, to, offset } => 
                add_mov_dword_value_into_reg_plus_offset_pointer_op(coff, value, get_register(to), offset),
            ByteCodeInstruction::MoveRegToRegPlusOffset32 { from, to, offset } => 
                add_mov_reg_to_reg_plus_offset_dword_pointer_op(coff, get_register(from), get_register(to), offset),
            ByteCodeInstruction::MoveRegToRegPlusOffset64 { from, to, offset } => 
                add_mov_reg_to_reg_plus_offset_qword_pointer_op(coff, get_register(from), get_register(to), offset),
            ByteCodeInstruction::MoveRegPlusOffsetToReg32 { from, offset, to } => 
                add_mov_dword_reg_plus_offset_pointer_to_reg_op(coff, get_register(from), offset, get_register(to)),
            ByteCodeInstruction::MoveRegPlusOffsetToReg64 { from, offset, to } => 
                add_mov_qword_reg_plus_offset_pointer_to_reg_op(coff, get_register(from), offset, get_register(to)),
            ByteCodeInstruction::SubValueFromReg8 { value, from } => 
                add_sub_byte_value_from_reg_op(coff, value, get_register(from)),
            ByteCodeInstruction::AddValueToReg8 { value, to } => 
                add_add_byte_value_to_reg_op(coff, value, get_register(to)),
            ByteCodeInstruction::ZeroReg64(register) =>
                add_xor_qword_reg_into_reg_op(coff, get_register(register), get_register(register)),
            ByteCodeInstruction::Return => add_ret_op(coff),
            ByteCodeInstruction::LoadDataSectionAddressToReg64 { data_section_offset, to } => 
                add_lea_reg_plus_offset_pointer_to_reg_op(
                    coff, 
                    REG_IP, 
                    relocatable_value(0x02, data_section_offset), 
                    get_register(to)
                ),
        }
    }

    for data_item in ir.data {
        match data_item {
            ByteCodeDataItem::String { value } => add_string_to_data_section(coff, &value),
        };
    }

    add_debug_file_name_symbols(coff, &ir.filename);
    add_data_section_header_symbols(coff);
    add_text_section_header_symbols(coff);
    add_absolute_static_symbol(coff, ".absolut", 0);
    
    for symbol_index in (0..ir.symbols.len()).rev() {
        match &ir.symbols[symbol_index] {
            ByteCodeSymbol::DataSectionItem { name, value } => add_data_section_static_symbol(coff, name, *value),
            ByteCodeSymbol::ForeignExternal { name } => add_foreign_external_symbol(coff, name),
            ByteCodeSymbol::AbsoluteExternal { name, value } => add_absolute_external_symbol(coff, name, *value),
            ByteCodeSymbol::ExternalCodeLabel { name, position } => add_text_section_external_symbol(coff, name, *position),
        }
    }

    let file_root = ir.filename.replace(".hep", "");
    format!("{}-{}.obj", file_root, ir.top_level_symbol)

}

fn convert_byte_code_to_coff_symbol_index(number_of_symbols: usize, symbol_index: u32) -> u32 {
    (6 + number_of_symbols) as u32 - symbol_index
}

fn get_register(register: ByteCodeRegister) -> u8 {
    match register {
        ByteCodeRegister::CallArg(number) => match number {
            0 => REG_CX,
            1 => REG_DX,
            2 => REG_R8,
            3 => REG_R9,
            r => panic!("call register {} should be passed on stack", r)
        },
        ByteCodeRegister::CallReturnArg(number) => match number {
            0 => REG_AX,
            _ => todo!()
        },
        ByteCodeRegister::StackPointer => REG_SP,
        ByteCodeRegister::BasePointer => REG_BP
    }
}
