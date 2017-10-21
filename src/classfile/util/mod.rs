pub mod conv;

use std::fs::File;
use std::io::Read;

pub fn read_u16(file: &mut File) -> u16 {
    let mut bin = [0u8; 2];
    file.read(&mut bin).unwrap();

    conv::make_u16(bin)
}

pub fn read_u32(file: &mut File) -> u32 {
    let mut bin = [0u8; 4];
    file.read(&mut bin).unwrap();

    conv::make_u32(bin)
}

pub fn read_bytes(file: &mut File, length: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(length as usize);
    let n = file.take(length as u64).read_to_end(&mut bytes).expect("Unexpected end of file");
    assert_eq!(length, n);

    bytes
}
