use std::fs::File;
use std::io::Read;

use classfile::conv::make_u16;

#[derive(Debug)]
pub struct Version {
    pub major: u16,
    pub minor: u16
}

pub fn read(file: &mut File) -> Version {
    let mut magic = [0u8; 4];
    file.read(&mut magic).unwrap();

    if !validate_magic(magic) {
        panic!("No valid Java class file.");
    }

    let mut minor_bin = [0u8; 2];
    file.read(&mut minor_bin).unwrap();

    let mut major_bin = [0u8; 2];
    file.read(&mut major_bin).unwrap();

    Version {
        major: make_u16(major_bin),
        minor: make_u16(minor_bin)
    }
}

fn validate_magic(magic: [u8; 4]) -> bool {
    let expected: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
    return magic.eq(&expected);
}
