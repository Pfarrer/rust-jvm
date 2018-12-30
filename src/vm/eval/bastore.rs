use vm::Vm;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let frame = vm.frame_stack.last_mut().unwrap();
    let value = frame.stack_pop_reference();
    let index = frame.stack_pop_int() as usize;
    let rc_array = frame.stack_pop_arrayref();
    let mut array = rc_array.borrow_mut();

    // Array must be of type byte or boolean
    assert!(array.atype == Some(4) || array.atype == Some(8));

    trace!("bastore: Read three values from stack and store reference in array index {}", index);
    array.elements[index] = value;

    Some(pc + 1)
}