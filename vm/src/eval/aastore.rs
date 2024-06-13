use crate::VmThread;

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_reference();
    let index = frame.stack_pop_int() as usize;
    let rc_array = frame.stack_pop_arrayref();
    let mut array = rc_array.borrow_mut();
    assert_eq!(array.atype, None);

    trace!(
        "aastore: Read three values from stack and store reference in array index {}",
        index
    );
    array.elements[index] = value;

    Some(pc + 1)
}
