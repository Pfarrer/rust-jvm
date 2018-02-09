use vm::Frame;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = frame.stack_pop_int() as usize;
    let rc_array = frame.stack_pop_arrayref();
    let array = rc_array.borrow_mut();
    assert_eq!(array.atype, None);

    trace!("aaload: Reading reference at array index {} and push it stack", index);

    let value = array.elements.get(index).unwrap().clone();
    frame.stack_push(value);

    Some(pc + 1)
}