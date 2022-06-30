use vm::Vm;
use vm::primitive::Primitive;

pub fn eval(vm: &mut Vm, pc: u16) -> Option<u16> {
    trace!("aconst_null: Pushing Null to stack");
    vm.frame_stack.last_mut().unwrap().stack_push(Primitive::Null);

    Some(pc + 1)
}