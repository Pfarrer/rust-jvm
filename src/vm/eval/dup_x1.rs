use vm::Frame;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value1 = frame.stack_pop();
    let value2 = frame.stack_pop();

    trace!("dup_x1: Duplicate the top operand stack value and insert two values down");
    frame.stack_push(value1.clone());
    frame.stack_push(value2);
    frame.stack_push(value1);

    Some(pc + 1)
}