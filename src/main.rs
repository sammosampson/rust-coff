#![feature(untagged_unions)]
mod machine_code;
mod intemediate_representation;

use crate::intemediate_representation::*;

use crate::machine_code::*;

fn main() {
    make_main();
    make_print();
    make_std_output_handle();
}

fn make_main() {
    let mut coff = create_coff();
    
    let mut main_ir = create_intermediate_representation(
        compilation_unit_id(2), 
        string("hello.hep"),
        string("main")
    );

    //main
    add_symbol(
        &mut main_ir.symbols,
        external_code_label(string(&main_ir.top_level_symbol), get_current_text_section_pointer(&coff))
    );
    // fn prologue    
    add_byte_code(&mut main_ir.byte_code, push_reg_64_instruction(base_pointer_register()));
    add_byte_code(
        &mut main_ir.byte_code, 
        move_reg_to_reg_64_instruction(stack_pointer_register(), base_pointer_register())
    );
    
    // print call:
    // set shadow space for print call
    add_byte_code(&mut main_ir.byte_code, sub_value_from_reg_8_instruction(32, stack_pointer_register()));
    
    // set pointer to hello world first arg for print call
    let hello = "Hello world!\r\n\0";
    let ds0_pointer = add_data_item(&mut main_ir.data, string_data_item(string(hello)));
    add_symbol(&mut main_ir.symbols, data_section_item(string("ds0"), ds0_pointer));
    add_byte_code(
        &mut main_ir.byte_code, 
        load_data_section_address_to_reg_64(ds0_pointer, call_arg_register(0))
    );
    
    // set hello world length second arg for print call
    add_byte_code(&mut main_ir.byte_code, move_value_to_reg_32_instruction(hello.len() as u32, call_arg_register(1)));
    //call print
    let symbol_index = add_symbol(&mut main_ir.symbols, foreign_external(string("print")));
    add_byte_code(&mut main_ir.byte_code, call_to_symbol_instruction(symbol_index));
    // release shadow space for print call   
    add_byte_code(&mut main_ir.byte_code, add_value_to_reg_8_instruction(32, stack_pointer_register()));
    
    // fn epilogue    
    add_byte_code(&mut main_ir.byte_code, move_reg_to_reg_64_instruction(
        base_pointer_register(), 
        stack_pointer_register())
    );
    add_byte_code(&mut main_ir.byte_code, pop_reg_64_instruction(base_pointer_register()));
    
    // return
    add_byte_code(&mut main_ir.byte_code, ret_instruction());
    
    let file_name = build_machine_code_object(&mut coff, main_ir);
    write_coff_to_file(&coff, &mut create_coff_file(&file_name).unwrap()).unwrap();
}

fn make_print() {
    let mut coff = create_coff();
    
    let mut print_ir = create_intermediate_representation(
        compilation_unit_id(1),
        string("hello.hep"),
        string("print")
    );

    //print
    add_symbol(
        &mut print_ir.symbols,
        external_code_label(string(&print_ir.top_level_symbol), get_current_text_section_pointer(&coff))
    );
    // fn prologue    
    add_byte_code(&mut print_ir.byte_code, push_reg_64_instruction(base_pointer_register()));
    add_byte_code(&mut print_ir.byte_code, move_reg_to_reg_64_instruction(
        stack_pointer_register(), 
        base_pointer_register())
    );
    //store args 1 and 2 in shadow
    add_byte_code(
        &mut print_ir.byte_code, 
        move_reg_to_reg_plus_offset_64_instruction(call_arg_register(0), base_pointer_register(), 16)
    );
    add_byte_code(
        &mut print_ir.byte_code, 
        move_reg_to_reg_plus_offset_32_instruction(call_arg_register(1), base_pointer_register(), 24)
    );
    //resesrve space for 1 local var (8 bytes)    
    add_byte_code(&mut print_ir.byte_code, sub_value_from_reg_8_instruction(8, stack_pointer_register()));
    // call to GetStdHandle
    // resesrve shadow space for call to GetStdHandle
    add_byte_code(&mut print_ir.byte_code, sub_value_from_reg_8_instruction(32, stack_pointer_register()));
    // set first arg (STD_OUTPUT_HANDLE) for call to GetStdHandle
    let symbol_index = add_symbol(&mut print_ir.symbols, foreign_external(string("STD_OUTPUT_HANDLE")));
    add_byte_code(&mut print_ir.byte_code,move_symbol_to_reg_32_instruction(symbol_index, call_arg_register(0)));
    // call GetStdHandle
    
    let symbol_index = add_symbol(&mut print_ir.symbols, foreign_external(string("GetStdHandle")));
    add_byte_code(&mut print_ir.byte_code, call_to_symbol_instruction(symbol_index));
    // release shadow space for call to GetStdHandle
    add_byte_code(&mut print_ir.byte_code, add_value_to_reg_8_instruction(32, stack_pointer_register()));
    // store local variable handle returned
    add_byte_code(
        &mut print_ir.byte_code, 
        move_reg_to_reg_plus_offset_32_instruction(call_return_arg_register(0), base_pointer_register(), 0xF8)
    );
    // call to WriteFile
    // resesrve space for 5 args, shadow + 1    
    add_byte_code(&mut print_ir.byte_code, sub_value_from_reg_8_instruction(40, stack_pointer_register()));
    // get values for args for call from storage
    add_byte_code(
        &mut print_ir.byte_code, 
        move_reg_plus_offset_to_reg_32_instruction(base_pointer_register(), 0xF8, call_arg_register(0))
    );
    add_byte_code(
        &mut print_ir.byte_code, 
        move_reg_plus_offset_to_reg_64_instruction(base_pointer_register(), 16, call_arg_register(1))
    );
    add_byte_code(
        &mut print_ir.byte_code, 
        move_reg_plus_offset_to_reg_32_instruction(base_pointer_register(), 24, call_arg_register(2))
    );
    add_byte_code(
        &mut print_ir.byte_code, 
        zero_reg_64_instruction(call_arg_register(3))
    );
    add_byte_code(
        &mut print_ir.byte_code, 
        move_value_to_reg_plus_offset_32_instruction(0x0, stack_pointer_register(), 32)
    );
    // call WriteFile
    let symbol_index = add_symbol(&mut print_ir.symbols, foreign_external(string("WriteFile")));
    add_byte_code(&mut print_ir.byte_code, call_to_symbol_instruction(symbol_index));
    // release space for 5 args, shadow + 1    
    add_byte_code(&mut print_ir.byte_code, add_value_to_reg_8_instruction(40, stack_pointer_register()));
    // fn epilogue    
    add_byte_code(&mut print_ir.byte_code, move_reg_to_reg_64_instruction(
        base_pointer_register(), 
        stack_pointer_register())
    );
    add_byte_code(&mut print_ir.byte_code, pop_reg_64_instruction(base_pointer_register()));
    // return
    add_byte_code(&mut print_ir.byte_code, ret_instruction());
    
    let file_name = build_machine_code_object(&mut coff, print_ir);   
    write_coff_to_file(&coff, &mut create_coff_file(&file_name).unwrap()).unwrap();
}

fn make_std_output_handle() {
    let mut coff = create_coff();

    let mut std_output_handle_ir = create_intermediate_representation(
        compilation_unit_id(2), 
        string("hello.hep"),
        string("STD_OUTPUT_HANDLE")
    );

    add_symbol(
        &mut std_output_handle_ir.symbols,
        absolute_external(string(&std_output_handle_ir.top_level_symbol), 0xFFFFFFF5)
    );

    let file_name = build_machine_code_object(&mut coff, std_output_handle_ir);   
    write_coff_to_file(&coff, &mut create_coff_file(&file_name).unwrap()).unwrap();
}