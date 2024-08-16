use crate::{frame::VmFrameImpl, VmPrimitive, VmThread};

pub fn eval(vm_thread: &mut VmThread, pc: u16) -> Option<u16> {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let index = frame.stack_pop_int() as usize;
    let rc_array = frame.stack_pop_arrayref();
    let array = rc_array.borrow();

    assert_eq!(array.atype.unwrap(), 7);

    if let &VmPrimitive::Double(ref value) = array.elements.get(index).unwrap() {
        trace!("daload: Popped two values from stack, read array at index {} and push Double '{}' to stack",
               index, value);

        frame.stack_push(VmPrimitive::Double(*value));
    } else {
        panic!("Unexpected array value.");
    }

    Some(pc + 1)
}
