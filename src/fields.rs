use std::fs::File;
use std::io::Read;

pub struct Fields {

}

pub fn read(file: &mut File) -> u16 {
    u2             fields_count;
    field_info     fields[fields_count];

    let mut access_flags_bin = [0u8; 2];
    file.read(&mut access_flags_bin).unwrap();

    make_u16(access_flags_bin)
}

fn make_u16(val: [u8; 2]) -> u16 {
    let high: u16 = (val[0] as u16) << 8;
    let low: u16 = val[1] as u16;

    return high + low;
}