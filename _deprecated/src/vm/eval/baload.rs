use vm::Vm;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    let (value, index) = {
        let frame = vm.frame_stack.last_mut().unwrap();
        let index = frame.stack_pop_int() as usize;
        let rc_array = frame.stack_pop_arrayref();
        let mut array = rc_array.borrow_mut();

        // Array must be of type byte or boolean
        assert!(array.atype == Some(4) || array.atype == Some(8));

        (array.elements[index].clone(), index)
    };

    trace!("bastore: Read two values from stack and push {} to stack from array at index {}", value, index);
    vm.frame_stack.last_mut().unwrap().stack_push(value);

    Some(pc + 1)
}