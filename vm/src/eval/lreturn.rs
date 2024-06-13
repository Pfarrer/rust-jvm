use crate::{VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread) -> Option<u16> {
    let mut frame = vm_thread.frame_stack.pop().unwrap();
    let mut parent_frame = vm_thread.frame_stack.pop().unwrap();

    let ret_val = frame.stack_pop_long();
    parent_frame.stack_push(VmPrimitive::Long(ret_val));
    trace!(
        "lreturn: Popped Long {} from stack, returning to parent method",
        ret_val
    );

    // Push frames back to the stack
    vm_thread.frame_stack.push(parent_frame);
    vm_thread.frame_stack.push(frame);

    None
}
