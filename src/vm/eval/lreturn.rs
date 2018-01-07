use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(frame: &mut Frame, parent_frame: &mut Frame) -> Option<u16> {
    let ret_val = frame.stack_pop_long();

    trace!("lreturn: Popped Long {} from stack, returning to parent method", ret_val);
    parent_frame.stack_push(Primitive::Long(ret_val));

    None
}
