pub mod conv;

use std::io::Read;

pub fn read_u16(reader: &mut Read) -> u16 {
    let mut bin = [0u8; 2];
    reader.read_exact(&mut bin).unwrap();

    conv::to_u16(bin)
}

pub fn read_u32(reader: &mut Read) -> u32 {
    let mut bin = [0u8; 4];
    reader.read_exact(&mut bin).unwrap();

    conv::to_u32(bin)
}

pub fn read_raw(reader: &mut Read, length: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(length);
    let n = reader.take(length as u64).read_to_end(&mut bytes).expect("Unexpected end of file");
    assert_eq!(length, n);

    bytes
}
