use crate::utils;

pub fn eval(code: &Vec<u8>, pc: u16) -> Option<u16> {
    let offset = utils::read_u16_code(code, pc) as i16;
    trace!("goto: Offset {}", offset);

    Some((pc as i16 + offset) as u16)
}