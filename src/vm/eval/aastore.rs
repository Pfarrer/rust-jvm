use vm::Frame;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value = frame.stack_pop_reference();
    let index = frame.stack_pop_int() as usize;
    let rc_array = frame.stack_pop_arrayref();
    let mut array = rc_array.borrow_mut();
    assert_eq!(array.atype, None);

    trace!("aastore: Reading three values from stack and store reference in array index {}", index);
    array.elements[index] = value;

    Some(pc + 1)
}