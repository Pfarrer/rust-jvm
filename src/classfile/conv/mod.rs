use std;

pub fn make_u16(val: [u8; 2]) -> u16 {
    let high: u16 = (val[0] as u16) << 8;
    let low: u16 = val[1] as u16;

    return high + low;
}

pub fn make_f32(val: [u8; 4]) -> f32 {
    unsafe {
        std::mem::transmute::<[u8; 4], f32>(val)
    }
}

pub fn make_i32(val: [u8; 4]) -> i32 {
    unsafe {
        std::mem::transmute::<[u8; 4], i32>(val)
    }
}