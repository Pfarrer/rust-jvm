use std::io::Read;

use crate::util;
use model::class::ClassInfo;

pub fn read(reader: &mut impl Read) -> ClassInfo {
    let access_flags = util::read_u16(reader);
    let this_class = util::read_u16(reader);
    let super_class = util::read_u16(reader);

    let interfaces_count = util::read_u16(reader);
    let mut interfaces = Vec::with_capacity(interfaces_count as usize);
    for _ in 0..interfaces_count {
        let interface = util::read_u16(reader);
        interfaces.push(interface);
    }

    ClassInfo {
        access_flags,
        this_class,
        super_class,
        interfaces,
    }
}
