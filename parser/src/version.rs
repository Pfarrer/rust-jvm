use std::io::Read;

use anyhow::{bail, Result};
use model::class::ClassVersion;

use crate::util;

const SUPPORTED_MAJOR_VERSION: u16 = 55; // Corresponds to Java 11

pub fn parse<T: Read>(reader: &mut T) -> Result<ClassVersion> {
    let mut magic = [0u8; 4];
    reader.read_exact(&mut magic)?;

    if !validate_magic(magic) {
        bail!("No valid Java class file.");
    }

    let minor = util::read_u16(reader)?;
    let major = util::read_u16(reader)?;

    if major > SUPPORTED_MAJOR_VERSION {
        panic!(
            "Unsupported Classfile version: {}.{} > {}.0.",
            major, minor, SUPPORTED_MAJOR_VERSION
        );
    }

    Ok(ClassVersion { major, minor })
}

fn validate_magic(magic: [u8; 4]) -> bool {
    let expected: [u8; 4] = [0xCA, 0xFE, 0xBA, 0xBE];
    return magic.eq(&expected);
}
