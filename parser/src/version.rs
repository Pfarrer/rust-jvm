use std::io::Read;

use crate::util;
use model::class::ClassVersion;

pub fn read(reader: &mut impl Read) -> ClassVersion {
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic).unwrap();

    if !validate_magic(magic) {
        panic!("No valid Java class file.");
    }

    let minor = util::read_u16(reader);
    let major = util::read_u16(reader);

    if major > 61 {
        panic!("Unsupported Classfile version: {}.{} > 61.0.", major, minor);
    }

    ClassVersion { major, minor }
}

fn validate_magic(magic: [u8; 4]) -> bool {
    let expected: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
    return magic.eq(&expected);
}
