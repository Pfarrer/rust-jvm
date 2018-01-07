use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let objectref = frame.stack_pop_reference();
    match objectref {
        Primitive::Null => panic!("Not implmented -> throw NullPointerException"),
        _ => (),
    };

    Some(pc + 2)
}