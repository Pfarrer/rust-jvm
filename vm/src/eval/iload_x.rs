use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

/// Can handle instructions iload and iload_<n>.
pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    // Check which instruction triggered this call, if it was iload, then one byte should be read,
    // when it was iload_<n>, the index is implicit
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        // iload
        21 => (*code.get((pc + 1) as usize).unwrap(), 2),
        // iload_<n>
        i @ 26..=29 => (i - 26, 1),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.locals_get_int(index as usize);
    trace!(
        "iload_{}: Read Int {} from locals and push it to the stack",
        index,
        value
    );

    frame.stack_push(VmPrimitive::Int(value));

    Some(pc + pc_inc)
}
