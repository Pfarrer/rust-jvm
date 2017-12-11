use vm::Frame;
use vm::primitive::Primitive;

pub fn eval(frame: &mut Frame, parent_frame: &mut Frame) -> Option<u16> {
    let ret_val = frame.stack_pop();

    match ret_val {
        Primitive::Reference(_) => {
            parent_frame.stack_push(ret_val);
            trace!("areturn: Popped Reference from stack, returning to parent method");
        },
        Primitive::Null => {
            parent_frame.stack_push(ret_val);
            trace!("areturn: Popped Null from stack, returning to parent method");
        },
        v => panic!("Popped {:?} from stack but expected a Reference or Null", v),
    };

    None
}
