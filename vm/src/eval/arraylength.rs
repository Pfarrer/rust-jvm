use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let rc_array = frame.stack_pop_arrayref();
    let array = rc_array.borrow();
    let length = array.elements.len();

    trace!(
        "arraylength: Popped Arrayref from stack and push Int {} to stack",
        length
    );
    frame.stack_push(VmPrimitive::Int(length as i32));

    Some(pc + 1)
}
