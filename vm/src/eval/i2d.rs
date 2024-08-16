use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_int();

    trace!(
        "i2d: Popped Int {} from stack and push it back as Double",
        value
    );
    frame.stack_push(VmPrimitive::Double(value as f64));

    Some(pc + 1)
}
