use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = code.get(pc as usize).unwrap() - 3;
    trace!("iconst_{}: Pushing constant Int {} to stack", index, index);

    vm.frame_stack.last_mut().unwrap().stack_push(Primitive::Int(index as i32));

    Some(pc + 1)
}