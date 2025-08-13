use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();

    match frame.stack_pop() {
        VmPrimitive::Long(_) | VmPrimitive::Double(_) => {
            trace!("pop2: Popping value from stack");
        }
        _ => {
            frame.stack_pop();
            trace!("pop2: Popping two values from stack");
        }
    };

    Some(pc + 1)
}
