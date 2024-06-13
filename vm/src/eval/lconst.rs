use crate::{VmPrimitive, VmThread};

pub fn eval(val: i64, vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    trace!("lconst_{}: Pushing {}L to stack", val, val);
    vm_thread
        .frame_stack
        .last_mut()
        .unwrap()
        .stack_push(VmPrimitive::Long(val));

    Some(pc + 1)
}
