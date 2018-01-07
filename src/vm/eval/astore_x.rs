use vm::Frame;
use vm::primitive::Primitive;

/// Can handle instructions astore and astore_<n>.
pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    // Check which instruction triggered this call, if it was astore, then one byte should be read,
    // when it was astore_<n>, the index is implicit
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        // astore
        58 => (*code.get((pc+1) as usize).unwrap(), 2),
        // astore_<n>
        i @ 75...78 => (i - 75, 1),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    let value = frame.stack_pop();
    match &value {
        &Primitive::Objectref(_) => (),
        &Primitive::ReturnAddress(_) => (),
        &Primitive::Null => (),
        _ => panic!("Popped unexpected value from stack, expected Objectref or ReturnAddress but found: {:?}", value),
    };

    trace!("astore_{}: Popping Reference from to stack and write to locals", index);
    frame.locals_write(index as usize, value);

    Some(pc + pc_inc)
}
