use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(pc: u16, frame: &mut Frame) -> Option<u16> {
    let value = frame.stack_pop();
    match value {
        Primitive::Long(_) | Primitive::Double(_) => {
            trace!("pop2: Popping value {:?} from stack", value);
        },
        _ => {
            let value2 = frame.stack_pop();
            trace!("pop2: Popping values {:?} and {:?} from stack", value, value2);
        },
    };

    Some(pc + 1)
}
