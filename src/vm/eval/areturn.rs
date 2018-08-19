use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm) -> Option<u16> {
    let mut frame = vm.frame_stack.pop().unwrap();
    let mut parent_frame = vm.frame_stack.pop().unwrap();

    let ret_val = frame.stack_pop();

    match ret_val {
        Primitive::Objectref(_) => {
            trace!("areturn: Popped Reference from stack, returning to parent method");
            parent_frame.stack_push(ret_val);
        },
        Primitive::Arrayref(_) => {
            trace!("areturn: Popped Array from stack, returning to parent method");
            parent_frame.stack_push(ret_val);
        },
        Primitive::Null => {
            trace!("areturn: Popped Null from stack, returning to parent method");
            parent_frame.stack_push(ret_val);
        },
        v => panic!("Popped {:?} from stack but expected a Reference or Null", v),
    };

    // Push frames back to the stack
    vm.frame_stack.push(parent_frame);
    vm.frame_stack.push(frame);

    None
}
