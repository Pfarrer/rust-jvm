use std;

pub fn to_u16(val: [u8; 2]) -> u16 {
    let reversed = [val[1], val[0]];

    unsafe {
        std::mem::transmute::<[u8; 2], u16>(reversed)
    }
}

pub fn to_u32(val: [u8; 4]) -> u32 {
    let reversed = [val[3], val[2], val[1], val[0]];

    unsafe {
        std::mem::transmute::<[u8; 4], u32>(reversed)
    }
}

pub fn to_i32(val: [u8; 4]) -> i32 {
    let reversed = [val[3], val[2], val[1], val[0]];

    unsafe {
        std::mem::transmute::<[u8; 4], i32>(reversed)
    }
}

pub fn to_f32(val: [u8; 4]) -> f32 {
    let reversed = [val[3], val[2], val[1], val[0]];

    unsafe {
        std::mem::transmute::<[u8; 4], f32>(reversed)
    }
}