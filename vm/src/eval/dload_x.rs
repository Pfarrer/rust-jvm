use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

/// Can handle instructions dload and dload_<n>.
pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    // Check which instruction triggered this call, if it was dload, then one byte should be read,
    // when it was dload_<n>, the index is implicit
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        // dload
        24 => (*code.get((pc + 1) as usize).unwrap(), 2),
        // dload_<n>
        i @ 38..=41 => (i - 38, 1),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.locals_get_double(index as usize);
    trace!(
        "dload_{}: Read Double {} from locals and push it to the stack",
        index,
        value
    );

    frame.stack_push(VmPrimitive::Double(value));

    Some(pc + pc_inc)
}
