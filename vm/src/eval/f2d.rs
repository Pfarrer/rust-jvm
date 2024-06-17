use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_float();
    let result = value as f64;

    trace!(
        "f2d: Popped Float {} from stack and push it back as Double {}",
        value,
        result
    );
    frame.stack_push(VmPrimitive::Double(result));

    Some(pc + 1)
}
