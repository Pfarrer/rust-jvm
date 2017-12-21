use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let (_, array) = frame.stack_pop_arrayref();
    let length = array.len();

    trace!("arraylength: Popping Arrayref from stack and push Int {} to stack", length);
    frame.stack_push(Primitive::Int(length as i32));

    Some(pc + 1)
}