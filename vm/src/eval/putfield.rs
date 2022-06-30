use classfile::constants::Constant;
use classfile::Classfile;
use vm::utils;
use vm::Vm;

pub fn eval(vm: &Vm, class: &Classfile, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match class.constants.get(index as usize).unwrap() {
        &Constant::Fieldref(ref class_path, ref field_name, _) => {
            // Pop value and Objectref from stack
            let frame = vm.frame_stack.last_mut().unwrap();
            let value = frame.stack_pop();
            let rc_instance = frame.stack_pop_objectref();
            let mut instance = rc_instance.borrow_mut();

            trace!(
                "putfield: Popped value and Objectref from stack an store value in field {}.{}",
                class_path,
                field_name
            );
            instance.fields.insert(field_name.clone(), value);
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}
