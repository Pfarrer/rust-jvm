use std::io::Read;

use classfile::util;

#[derive(Clone, Debug)]
pub struct Version {
    pub major: u16,
    pub minor: u16
}

pub fn read(reader: &mut Read) -> Version {
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic).unwrap();

    if !validate_magic(magic) {
        panic!("No valid Java class file.");
    }

    let minor = util::read_u16(reader);
    let major = util::read_u16(reader);

    if major > 49 {
        panic!("Unsupported Classfile version: {}.{} > 49.0.", major, minor);
    }

    Version {
        major,
        minor,
    }
}

fn validate_magic(magic: [u8; 4]) -> bool {
    let expected: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
    return magic.eq(&expected);
}