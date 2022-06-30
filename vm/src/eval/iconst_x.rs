use vm::primitive::Primitive;
use vm::Vm;

pub fn eval(vm: &Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = (*code.get(pc as usize).unwrap() as i32) - 3;
    trace!("iconst_{}: Pushing constant Int {} to stack", index, index);

    vm.frame_stack
        .last_mut()
        .unwrap()
        .stack_push(Primitive::Int(index));

    Some(pc + 1)
}
