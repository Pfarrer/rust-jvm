use vm::Frame;
use vm::primitive::Primitive;

/// Can handle instructions lstore and lstore_<n>.
pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    // Check which instruction triggered this call, if it was lstore, then one byte should be read,
    // when it was lstore_<n>, the index is implicit
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        // lstore
        55 => (*code.get((pc+1) as usize).unwrap(), 2),
        // lstore_<n>
        i @ 63...66 => (i - 63, 1),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    let value = frame.stack_pop_long();
    trace!("lstore_{}: Popping Long {} from to stack and write to locals", index, value);

    frame.locals_write(index as usize, Primitive::Long(value));

    Some(pc + pc_inc)
}