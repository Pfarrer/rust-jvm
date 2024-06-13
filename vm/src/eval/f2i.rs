use crate::{VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_float();
    let result = value as i32;

    trace!(
        "i2f: Popped Float {} from stack and push it back as Int {}",
        value,
        result
    );
    frame.stack_push(VmPrimitive::Int(result));

    Some(pc + 1)
}
