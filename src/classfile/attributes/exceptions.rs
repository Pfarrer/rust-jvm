use std::io::Read;

use classfile::util;

pub type Exceptions = Vec<u16>;

pub fn read(reader: &mut Read) -> Exceptions {
    /*let attribute_length = */util::read_u32(reader);

    let number_of_exceptions = util::read_u16(reader);
    let mut exceptions = Vec::with_capacity(number_of_exceptions as usize);
    for _ in 0..number_of_exceptions {
        let pool_index = util::read_u16(reader);
        exceptions.push(pool_index);
    }

    exceptions
}
