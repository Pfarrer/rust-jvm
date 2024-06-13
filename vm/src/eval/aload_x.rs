use crate::VmThread;

pub fn eval(vm_thread: &mut VmThread, code: &Vec<u8>, pc: u16) -> Option<u16> {
    // Check which instruction triggered this call, if it was aload, then one byte should be read,
    // when it was aload_<n>, the index is implicit
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        // aload
        25 => (*code.get((pc + 1) as usize).unwrap(), 2),
        // aload_<n>
        i @ 42..=45 => (i - 42, 1),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let value = frame.locals_get_reference(index as usize).clone();
    frame.stack_push(value.clone());

    trace!("aload_{}: Pushed Reference to stack", index);

    Some(pc + pc_inc)
}
