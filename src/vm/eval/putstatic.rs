use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::Frame;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let indexbyte1: u16 = (*code.get((pc+1) as usize).unwrap() as u16) << 8;
    let indexbyte2 = (*code.get((pc+2) as usize).unwrap()) as u16;
    let index = indexbyte1 + indexbyte2;

    match class.constants.get(index as usize).unwrap() {
        &Constant::Fieldref(ref class_path, ref field_name, ref type_name) => {
            // Initialize class
            vm.load_and_clinit_class(class_path);

            // Pop value and push to statics
            let value = frame.stack_pop();
            trace!("putstatic: {}.{}{} = {:?}", class_path, field_name, type_name, value);

            vm.class_statics.get_mut(class_path).unwrap()
                .insert(field_name.clone(), value);
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc+3)
}