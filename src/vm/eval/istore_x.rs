use vm::Frame;
use vm::primitive::Primitive;

/// Can handle instructions istore and istore_<n>.
pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    // Check which instruction triggered this call, if it was istore, then one byte should be read,
    // when it was istore_<n>, the index is implicit
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        // istore
        54 => (*code.get((pc+1) as usize).unwrap(), 2),
        // istore_<n>
        i @ 59...62 => (i - 59, 1),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    let value = frame.stack_pop_int();
    trace!("istore_{}: Popping Int {} from to stack and write to locals", index, value);

    frame.locals_write(index as usize, Primitive::Int(value));

    Some(pc + pc_inc)
}
