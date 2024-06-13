use crate::{VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread) -> Option<u16> {
    let mut frame = vm_thread.frame_stack.pop().unwrap();
    let mut parent_frame = vm_thread.frame_stack.pop().unwrap();

    let ret_val = frame.stack_pop();

    match ret_val {
        VmPrimitive::Objectref(_) => {
            trace!("areturn: Popped Reference from stack, returning to parent method");
            parent_frame.stack_push(ret_val);
        }
        VmPrimitive::Arrayref(_) => {
            trace!("areturn: Popped VmArray from stack, returning to parent method");
            parent_frame.stack_push(ret_val);
        }
        VmPrimitive::Null => {
            trace!("areturn: Popped Null from stack, returning to parent method");
            parent_frame.stack_push(ret_val);
        }
        v => panic!("Popped {:?} from stack but expected a Reference or Null", v),
    };

    // Push frames back to the stack
    vm_thread.frame_stack.push(parent_frame);
    vm_thread.frame_stack.push(frame);

    None
}
