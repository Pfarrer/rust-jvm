use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_long();

    let result = value1 << (value2 & 0x3f);

    trace!(
        "lshl: Shifting Long {} left by {} (originally {}) -> pushing Long {} to stack",
        value1,
        value2 & 0x3f,
        value2,
        result
    );
    frame.stack_push(VmPrimitive::Long(result));

    Some(pc + 1)
}
