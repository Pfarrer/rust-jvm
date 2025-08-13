use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();
    let index = frame.stack_pop_int() as usize;

    let rc_array = frame.stack_pop_arrayref();
    let mut array = rc_array.borrow_mut();
    assert_eq!(array.atype.unwrap(), 10);

    trace!(
        "iastore: Popped three values from stack and write '{}' at array index {}",
        value,
        index
    );
    array.elements[index] = VmPrimitive::Int(value);

    Some(pc + 1)
}
