use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::Frame;
use vm::signature;
use vm::utils;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match class.constants.get(index as usize).unwrap() {
        &Constant::Methodref(ref class_path, ref method_name, ref method_signature) => {
            trace!("invokespecial: {}.{}{}", class_path, method_name, method_signature);

            let inner_class = vm.load_and_clinit_class(class_path);
            let inner_method = utils::find_method(&inner_class, method_name, method_signature)
                .unwrap_or_else(|| panic!("Method not found: {}.{}{}", class_path, method_name, method_signature));
            let mut inner_frame = Frame::new();

            let sig = signature::parse_method(method_signature);
            if sig.parameters.len() > 0 {
                panic!("Case not implemented");
            }

            // Push the instance reference to local no. 0
            inner_frame.locals_push(frame.stack_pop());

//            // Parse signature and move arguments from caller frame to callee frame
//            for _ in 0..sig.parameters.len() {
//                panic!("Not implemented... the following code is wrong");
//                inner_frame.stack_push(frame.stack_pop());
//            }

            vm.execute_method(&inner_class, &inner_method, &mut inner_frame, frame);
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc+3)
}