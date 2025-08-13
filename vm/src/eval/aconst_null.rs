use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    trace!("aconst_null: Pushing Null to stack");
    vm_thread
        .frame_stack
        .last_mut()
        .unwrap()
        .stack_push(VmPrimitive::Null);

    Some(pc + 1)
}
