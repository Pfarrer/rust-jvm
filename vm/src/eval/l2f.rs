use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_long();
    let result = value as f32;

    trace!(
        "l2f: Popped Long {} from stack and push it back as Float {}",
        value,
        result
    );
    frame.stack_push(VmPrimitive::Float(result));

    Some(pc + 1)
}
