use vm::Frame;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    trace!("pop: Popping value from stack");
    frame.stack_pop();

    Some(pc + 1)
}
