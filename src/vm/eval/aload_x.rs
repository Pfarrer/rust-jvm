use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = code.get(pc as usize).unwrap() - 42;
    trace!("aload_{}: Pushing Reference to stack", index);

    let value = frame.locals_get(index as usize).clone();
    match value {
        Primitive::Arrayref(_) => (),
        Primitive::Objectref(_) => (),
        Primitive::ReturnAddress(_) => (),
        Primitive::Null => (),
        _ => panic!("Popped unexpected value from stack, found: {:?}", value),
    };

    frame.stack_push(value.clone());

    Some(pc + 1)
}