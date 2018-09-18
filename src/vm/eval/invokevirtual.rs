use classfile::Classfile;
use classfile::constants::Constant;
use vm::primitive::Primitive;
use vm::signature;
use vm::utils;
use vm::Vm;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);

    // First, find Methodref and extract values
    let (class_path, method_name, method_signature) = match class.constants.get(index as usize).unwrap() {
        &Constant::Methodref(ref class_path, ref method_name, ref method_signature) =>
            (class_path.clone(), method_name.clone(), method_signature.clone()),
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    // Next, figure out how many arguments this method has. This is important in
    // order to know to peek at which position the stack to fetch the Objectref
    // of the instance that shall be invoked.
    let args_count = signature::parse_method(&method_signature).parameters.len();

    let root_class_path = {
        let frame = vm.frame_stack.last_mut().unwrap();
        let class_path = match frame.stack_peek_reverse(args_count) {
            &Primitive::Objectref(ref rc_object) => rc_object.borrow().class_path.clone(),
            p => panic!("Expected to pop Objectref from stack but found: {:?}", p),
        };

        class_path
    };

    debug!("invokevirtual: {}.{}{} on class {}", class_path, method_name, method_signature, root_class_path);
    utils::invoke_method(vm, &root_class_path, &method_name, &method_signature, true);

    Some(pc + 3)
}
