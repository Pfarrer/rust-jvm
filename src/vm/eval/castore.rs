use std::char;

use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value = frame.stack_pop_int() as u16;
    let index = frame.stack_pop_int() as usize;

    let rc_array = frame.stack_pop_arrayref();
    let mut array = rc_array.borrow_mut();
    assert_eq!(array.atype.unwrap(), 5);

    trace!("castore: Popping three values from stack and write '{}' at array index {}",
           char::from_u32(value as u32).unwrap(), index);

    array.elements[index] = Primitive::Char(value);

    Some(pc + 1)
}