
use std::*;
use std::fs::File;
use std::io::Write;

fn any_as_u8_slice<T: Sized>(p: &T) -> &[u8] {
    unsafe { 
        slice::from_raw_parts(
            (p as *const T) as *const u8,
            ::std::mem::size_of::<T>(),
        )
    }
}

const IMAGE_FILE_MACHINE_AMD64: u16 = 0x8664;

struct CoffFileHeaderSection {
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32
}

struct CoffFile {
    header: CoffFileHeaderSection
}

fn main() {
    let coff = CoffFile {
        header : CoffFileHeaderSection { 
            machine: IMAGE_FILE_MACHINE_AMD64,
            number_of_sections: 2,
            time_date_stamp: 0x631F038B
        }
    };
    let coff_bytes: &[u8] = any_as_u8_slice(&coff);
    let mut coff_write_file_buffer = File::create("coff.obj").unwrap();
    coff_write_file_buffer.write_all(coff_bytes).unwrap();
    coff_write_file_buffer.flush().unwrap();
}