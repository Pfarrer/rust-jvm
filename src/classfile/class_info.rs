use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct ClassInfo {
    pub access_flags: u16,
    pub this_class: u16,
    pub super_class: u16,
    pub interfaces: Vec<u16>
}

pub fn read(file: &mut File) -> ClassInfo {
    let mut access_flags_bin = [0u8; 2];
    file.read(&mut access_flags_bin).unwrap();
    let access_flags = make_u16(access_flags_bin);

    let mut this_class_bin = [0u8; 2];
    file.read(&mut this_class_bin).unwrap();
    let this_class = make_u16(this_class_bin);

    let mut super_class_bin = [0u8; 2];
    file.read(&mut super_class_bin).unwrap();
    let super_class = make_u16(super_class_bin);

    let mut interfaces_count_bin = [0u8; 2];
    file.read(&mut interfaces_count_bin).unwrap();
    let interfaces_count = make_u16(interfaces_count_bin);

    let mut interfaces = Vec::new();
    for _ in 1..interfaces_count {
        let mut interface_bin = [0u8; 2];
        file.read(&mut interface_bin).unwrap();
        let interface = make_u16(interface_bin);

        interfaces.push(interface);
    }

    ClassInfo {
        access_flags,
        this_class,
        super_class,
        interfaces,
    }
}

fn make_u16(val: [u8; 2]) -> u16 {
    let high: u16 = (val[0] as u16) << 8;
    let low: u16 = val[1] as u16;

    return high + low;
}