use std::fs::File;
use std::io::Read;

use classfile::util::read_u16;

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

    let minor = read_u16(file);
    let major = read_u16(file);

    Version {
        major,
        minor,
    }
}

fn validate_magic(magic: [u8; 4]) -> bool {
    let expected: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
    return magic.eq(&expected);
}
