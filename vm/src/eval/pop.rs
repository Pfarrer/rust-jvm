use crate::{frame::VmFrameImpl, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    trace!("pop: Popped value from stack");
    vm_thread.frame_stack.last_mut().unwrap().stack_pop();

    Some(pc + 1)
}
