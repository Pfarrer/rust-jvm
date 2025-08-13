use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

/// Can handle instructions dstore and dstore_<n>.
pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    // Check which instruction triggered this call, if it was lstore, then one byte should be read,
    // when it was dstore_<n>, the index is implicit
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        // dstore
        57 => (*code.get((pc + 1) as usize).unwrap(), 2),
        // dstore_<n>
        i @ 71..=74 => (i - 71, 1),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_double();

    trace!(
        "dstore_{}: Popped Double {} from to stack and write to locals",
        index,
        value
    );

    frame.locals_write(index as usize, VmPrimitive::Double(value));

    Some(pc + pc_inc)
}
