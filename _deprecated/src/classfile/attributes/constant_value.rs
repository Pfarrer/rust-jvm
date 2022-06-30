use std::io::Read;

use classfile::util;

pub fn read(reader: &mut Read) -> u16 {
    /*let attribute_length = */util::read_u32(reader);

    util::read_u16(reader)
}
