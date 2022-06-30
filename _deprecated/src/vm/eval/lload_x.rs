use vm::Vm;
use vm::primitive::Primitive;

/// Can handle instructions lload and lload_<n>.
pub fn eval(vm: &mut Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    // Check which instruction triggered this call, if it was lload, then one byte should be read,
    // when it was lload_<n>, the index is implicit
    let (index, pc_inc) = match *code.get(pc as usize).unwrap() {
        // lload
        22 => (*code.get((pc + 1) as usize).unwrap(), 2),
        // lload_<n>
        i @ 30 ... 33 => (i - 30, 1),
        i => panic!("Unexpected invocation of this instruction, found: {}", i),
    };

    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.locals_get_long(index as usize);
    trace!("lload_{}: Read Long {} from locals and push it to the stack", index, value);

    frame.stack_push(Primitive::Long(value));

    Some(pc + pc_inc)
}
