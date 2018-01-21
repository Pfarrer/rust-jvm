use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::Frame;
use vm::utils;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match class.constants.get(index as usize).unwrap() {
        &Constant::Fieldref(ref class_path, ref field_name, _) => {
            // Initialize class
            vm.load_and_clinit_class(class_path);

            // Pop value and push to statics
            let value = frame.stack_pop();
            trace!("putstatic: Popping value from stack and store it in {}.{}", class_path, field_name);

            vm.class_statics.get_mut(class_path).unwrap()
                .insert(field_name.clone(), value);
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc+3)
}