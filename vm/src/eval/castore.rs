use std::char;

use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int() as u16;
    let index = frame.stack_pop_int() as usize;

    let rc_array = frame.stack_pop_arrayref();
    let mut array = rc_array.borrow_mut();
    assert_eq!(array.atype.unwrap(), 5);

    trace!(
        "castore: Popped three values from stack and write '{}' at array index {}",
        char::from_u32(value as u32).unwrap(),
        index
    );

    array.elements[index] = Primitive::Char(value);

    Some(pc + 1)
}
