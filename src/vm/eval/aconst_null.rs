use vm::Frame;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    trace!("aconst_null: Pushing null (0) to stack");
    frame.stack_push_int(0);

    Some(pc+1)
}