use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();
    let result = (value as i8) as i32;

    trace!(
        "i2b: Popped Int {} from stack, truncated to Byte and pushed back as Int {}",
        value,
        result
    );
    frame.stack_push(VmPrimitive::Int(result));

    Some(pc + 1)
}
