use vm::Frame;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value = frame.stack_pop();
    trace!("pop: Popping value {:?} from stack", value);

    Some(pc + 1)
}
