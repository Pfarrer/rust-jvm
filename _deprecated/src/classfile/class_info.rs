use std::io::Read;

use classfile::util;

#[derive(Clone, Debug)]
pub struct ClassInfo {
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>
}

pub fn read(reader: &mut Read) -> ClassInfo {
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