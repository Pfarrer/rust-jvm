use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let rc_array = frame.stack_pop_arrayref();
    let array = rc_array.borrow();
    let length = array.elements.len();

    trace!("arraylength: Popping Arrayref from stack and push Int {} to stack", length);
    frame.stack_push(Primitive::Int(length as i32));

    Some(pc + 1)
}