use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    trace!("aconst_null: Pushing Null to stack");
    frame.stack_push(Primitive::Null);

    Some(pc+1)
}