use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();
    let index = frame.stack_pop_int() as usize;
    let rc_array = frame.stack_pop_arrayref();
    let mut array = rc_array.borrow_mut();

    // Array must be of type byte or boolean
    assert!(array.atype == Some(4) || array.atype == Some(8));

    if array.atype == Some(4) {
        // Boolean type
        array.elements[index] = Primitive::Boolean(value > 1);
    } else if array.atype == Some(8) {
        // Byte type
        array.elements[index] = Primitive::Byte(value as u8);
    }

    trace!(
        "bastore: Read three values from stack and store {:?} in array index {}",
        array.elements[index],
        index
    );
    Some(pc + 1)
}
