pub mod conv;

use std::fs::File;
use std::io::Read;

pub fn read_u16(file: &mut File) -> u16 {
    let mut bin = [0u8; 2];
    file.read(&mut bin).unwrap();

    conv::make_u16(bin)
}