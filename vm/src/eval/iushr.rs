use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value2 = frame.stack_pop_int();
    let value1 = frame.stack_pop_int();

    let s = value2 & 0x1f;

    // The JVM Spec recommends a different calulation, but this seems to work better
    let result = (value1 as u32 >> s) as i32;

    trace!(
        "iushr: Shifting {} right unsigned by {} -> pushing {} to stack",
        value1,
        s,
        result
    );
    frame.stack_push(VmPrimitive::Int(result));

    Some(pc + 1)
}
