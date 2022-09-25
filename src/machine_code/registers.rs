pub const REG_AX: u8 = 0x00;
pub const REG_CX: u8 = 0x01;
pub const REG_DX: u8 = 0x02;
pub const REG_SP: u8 = 0x04;
pub const REG_BP: u8 = 0x05;
pub const REG_IP: u8 = 0x05;
pub const REG_R8: u8 = 0x08; 
pub const REG_R9: u8 = 0x09;   

pub fn register_has_high_bit(register: u8) -> bool {
    register & 0x8 == 0x8
}

pub fn remove_register_high_bit(register: u8) -> u8 {
    register & 0x7
}
