use crate::{Primitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let index = frame.stack_pop_int() as usize;
    let rc_array = frame.stack_pop_arrayref();
    let array = rc_array.borrow_mut();
    assert_eq!(array.atype, None);

    trace!(
        "aaload: Read Reference at array index {} and push it stack",
        index
    );

    let value = array.elements.get(index).unwrap().clone();
    frame.stack_push(value);

    Some(pc + 1)
}
