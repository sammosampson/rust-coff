#[derive(Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub struct CompilationUnitId {
    id: usize
}

pub fn compilation_unit_id(id: usize) -> CompilationUnitId {
    CompilationUnitId {
        id
    }
}

#[derive(Debug, Clone)]
pub struct IntermediateRepresentation {
    pub id: CompilationUnitId,
    pub filename: String,
    pub top_level_symbol: String,
    pub byte_code: ByteCodeInstructionStream,
    pub symbols: ByteCodeSymbols,
    pub data: ByteCodeData
}

pub fn create_intermediate_representation(id: CompilationUnitId, filename: String, top_level_symbol: String) -> IntermediateRepresentation {
    IntermediateRepresentation { id, filename, top_level_symbol, byte_code: vec!(), symbols: vec!(), data: vec!() }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ByteCodeRegister {
    CallArg(usize),
    CallReturnArg(usize),
    StackPointer,
    BasePointer
}

pub fn call_arg_register(number: usize) -> ByteCodeRegister {
    ByteCodeRegister::CallArg(number)
}

pub fn call_return_arg_register(number: usize) -> ByteCodeRegister {
    ByteCodeRegister::CallReturnArg(number)
}

pub fn base_pointer_register() -> ByteCodeRegister {
    ByteCodeRegister::BasePointer
}

pub fn stack_pointer_register() -> ByteCodeRegister {
    ByteCodeRegister::StackPointer
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ByteCodeInstruction {
    CallToSymbol(u32),
    AddValueToReg8 { value: u8, to: ByteCodeRegister },
    SubValueFromReg8 { value: u8, from: ByteCodeRegister },
    MoveSymbolToReg32 { symbol_index: u32, to: ByteCodeRegister },
    MoveValueToReg32 { value: u32, to: ByteCodeRegister },
    MoveRegToReg64 { from: ByteCodeRegister, to: ByteCodeRegister },
    MoveValueToRegPlusOffset32 { value: u32, to: ByteCodeRegister, offset: u8 },
    MoveRegToRegPlusOffset32 { from: ByteCodeRegister, to: ByteCodeRegister, offset: u8 },
    MoveRegToRegPlusOffset64 { from: ByteCodeRegister, to: ByteCodeRegister, offset: u8 },
    MoveRegPlusOffsetToReg32 { from: ByteCodeRegister, offset: u8, to: ByteCodeRegister },
    MoveRegPlusOffsetToReg64 { from: ByteCodeRegister, offset: u8, to: ByteCodeRegister },
    LoadDataSectionAddressToReg64 { data_section_offset: u32, to: ByteCodeRegister },
    PushReg64(ByteCodeRegister),
    PopReg64(ByteCodeRegister),
    ZeroReg64(ByteCodeRegister),
    Return
}

pub fn call_to_symbol_instruction(symbol_index: u32) -> ByteCodeInstruction {
    ByteCodeInstruction::CallToSymbol(symbol_index)
}

pub fn push_reg_64_instruction(register: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::PushReg64(register)
}

pub fn pop_reg_64_instruction(register: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::PopReg64(register)
}

pub fn add_value_to_reg_8_instruction(value: u8, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::AddValueToReg8 { value, to }
}

pub fn sub_value_from_reg_8_instruction(value: u8, from: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::SubValueFromReg8 { value, from }
}

pub fn move_symbol_to_reg_32_instruction(symbol_index: u32, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveSymbolToReg32 { symbol_index, to }
}

pub fn move_value_to_reg_32_instruction(value: u32, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveValueToReg32 { value, to }
}

pub fn move_reg_to_reg_64_instruction(from: ByteCodeRegister, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveRegToReg64 { from, to }
}

pub fn move_value_to_reg_plus_offset_32_instruction(value: u32, to: ByteCodeRegister, offset: u8) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveValueToRegPlusOffset32 { value, to, offset }
}

pub fn move_reg_to_reg_plus_offset_64_instruction(from: ByteCodeRegister, to: ByteCodeRegister, offset: u8) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveRegToRegPlusOffset64 { from, to, offset }
}

pub fn move_reg_to_reg_plus_offset_32_instruction(from: ByteCodeRegister, to: ByteCodeRegister, offset: u8) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveRegToRegPlusOffset32 { from, to, offset }
}

pub fn move_reg_plus_offset_to_reg_32_instruction(from: ByteCodeRegister, offset: u8, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveRegPlusOffsetToReg32 { from, offset, to }
}

pub fn move_reg_plus_offset_to_reg_64_instruction(from: ByteCodeRegister, offset: u8, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::MoveRegPlusOffsetToReg64 { from, offset, to }
}

pub fn load_data_section_address_to_reg_64(data_section_offset: u32, to: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::LoadDataSectionAddressToReg64 { data_section_offset, to }
}

pub fn zero_reg_64_instruction(register: ByteCodeRegister) -> ByteCodeInstruction {
    ByteCodeInstruction::ZeroReg64(register)
}

pub fn ret_instruction() -> ByteCodeInstruction {
    ByteCodeInstruction::Return
}
pub type ByteCodeInstructionStream = Vec<ByteCodeInstruction>;

pub fn add_byte_code(byte_code_stream: &mut Vec<ByteCodeInstruction>, instruction: ByteCodeInstruction) {
    byte_code_stream.push(instruction);
}

#[derive(Debug, Clone)]
pub enum ByteCodeSymbol {
    DataSectionItem { name: String, value: u32 },
    ForeignExternal { name: String },
    AbsoluteExternal { name: String, value: u32 },
    ExternalCodeLabel { name: String, position: u32 },
}

pub type ByteCodeSymbols = Vec<ByteCodeSymbol>;

pub fn data_section_item(name: String, value: u32) -> ByteCodeSymbol{
    ByteCodeSymbol::DataSectionItem { name, value }
}

pub fn foreign_external(name: String) -> ByteCodeSymbol{
    ByteCodeSymbol::ForeignExternal { name }
}

pub fn absolute_external(name: String, value: u32) -> ByteCodeSymbol{
    ByteCodeSymbol::AbsoluteExternal { name, value }
}

pub fn external_code_label(name: String, position: u32) -> ByteCodeSymbol{
    ByteCodeSymbol::ExternalCodeLabel { name, position }
}

pub fn add_symbol(symbols: &mut ByteCodeSymbols, symbol: ByteCodeSymbol) -> u32 {
    symbols.push(symbol);
    (symbols.len() - 1) as u32
}


#[derive(Debug, Clone)]
pub enum ByteCodeDataItem {
    String { value: String }
}

pub type ByteCodeData = Vec<ByteCodeDataItem>;

pub fn string_data_item(value: String) -> ByteCodeDataItem{
    ByteCodeDataItem::String { value }
}

pub fn add_data_item(data: &mut ByteCodeData, item: ByteCodeDataItem) -> u32 {
    data.push(item);
    (data.len() - 1) as u32
}