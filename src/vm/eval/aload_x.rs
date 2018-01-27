use vm::Frame;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    // Check which instruction triggered this call, if it was aload, then one byte should be read,
    // when it was aload_<n>, the index is implicit
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        // aload
        25 => (*code.get((pc+1) as usize).unwrap(), 2),
        // aload_<n>
        i @ 42...45 => (i - 42, 1),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    trace!("aload_{}: Pushing Reference to stack", index);

    let value = frame.locals_get_reference(index as usize).clone();
    frame.stack_push(value.clone());

    Some(pc + pc_inc)
}