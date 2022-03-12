use std;
use std::io::Read;

pub fn read_u16(reader: &mut impl Read) -> u16 {
    let mut bin = [0u8; 2];
    reader.read_exact(&mut bin).unwrap();

    to_u16(bin)
}

pub fn read_u32(reader: &mut impl Read) -> u32 {
    let mut bin = [0u8; 4];
    reader.read_exact(&mut bin).unwrap();

    to_u32(bin)
}

pub fn read_raw(reader: &mut impl Read, length: usize) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(length);
    let n = reader
        .take(length as u64)
        .read_to_end(&mut bytes)
        .expect("Unexpected end of file");
    assert_eq!(length, n);

    bytes
}

pub fn to_u16(val: [u8; 2]) -> u16 {
    let reversed = [val[1], val[0]];

    unsafe { std::mem::transmute::<[u8; 2], u16>(reversed) }
}

pub fn to_u32(val: [u8; 4]) -> u32 {
    let reversed = [val[3], val[2], val[1], val[0]];

    unsafe { std::mem::transmute::<[u8; 4], u32>(reversed) }
}

pub fn to_i32(val: [u8; 4]) -> i32 {
    let reversed = [val[3], val[2], val[1], val[0]];

    unsafe { std::mem::transmute::<[u8; 4], i32>(reversed) }
}

pub fn to_i64(val: [u8; 8]) -> i64 {
    let reversed = [
        val[7], val[6], val[5], val[4], val[3], val[2], val[1], val[0],
    ];

    unsafe { std::mem::transmute::<[u8; 8], i64>(reversed) }
}

pub fn to_f32(val: [u8; 4]) -> f32 {
    let reversed = [val[3], val[2], val[1], val[0]];

    unsafe { std::mem::transmute::<[u8; 4], f32>(reversed) }
}

pub fn to_f64(val: [u8; 8]) -> f64 {
    let reversed = [
        val[7], val[6], val[5], val[4], val[3], val[2], val[1], val[0],
    ];

    unsafe { std::mem::transmute::<[u8; 8], f64>(reversed) }
}
