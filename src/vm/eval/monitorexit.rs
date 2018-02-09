use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let objectref = frame.stack_pop_reference();
    match objectref {
        Primitive::Null => panic!("Not implemented -> throw NullPointerException"),
        _ => (),
    };

    trace!("monitorexit: Popped one reference from stack and did nothing else");

    Some(pc + 1)
}