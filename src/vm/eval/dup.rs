use vm::Frame;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value = frame.stack_pop();
    trace!("dup: Duplicating last stack element");

    frame.stack_push(value.clone());
    frame.stack_push(value);

    Some(pc + 1)
}