use std::{
    *,
    time::*
};

pub fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe { 
        slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }
}

pub fn get_8_padded_u8_array_from_string(from: &str) -> [u8; 8] {
    assert!(from.len() <= 8);
    
    let mut to = [0; 8];
    to[..from.len()].copy_from_slice(&from.as_bytes());
    to
}

pub fn get_truncated_18_padded_u8_array_from_string(from: &str) -> [u8; 18] {
    let from_len = if from.len() < 18 { from.len() } else { 18 };
    
    let mut to = [0; 18];
    to[..from_len].copy_from_slice(&from[..from_len].as_bytes());
    to
}

pub fn string_to_bytes_zero_terminated(entry: &str) -> Vec<u8> {
    let mut new_string = string_to_bytes(entry);
    new_string.push(0x0);
    new_string
}

pub fn string_to_bytes(entry: &str) -> Vec<u8> {
    entry.as_bytes().into()
}

pub fn u32_to_bytes(entry: &u32) -> Vec<u8> {
    any_as_u8_slice(entry).into()
}

pub fn string(value: &str) -> String {
    value.to_string()
}

pub fn get_current_timestamp() -> u32 {
    // seconds since 1970-01-01 00:00:00 GMT
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => return n.as_secs() as u32,
        Err(_) => panic!("SystemTime before UNIX EPOCH!"),
    }
}