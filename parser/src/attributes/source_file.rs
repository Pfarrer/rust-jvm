use std::io::Read;

use crate::util;

pub fn read(reader: &mut impl Read) -> u16 {
    /*let attribute_length = */
    util::read_u32(reader);

    util::read_u16(reader)
}
