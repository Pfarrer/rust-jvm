use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;
use vm::Frame;

pub fn eval(vm: &mut Vm, class: &Classfile, code: &Vec<u8>, pc: u16, frame: &mut Frame) -> Option<u16> {
    let indexbyte1: u16 = (*code.get((pc+1) as usize).unwrap() as u16) << 8;
    let indexbyte2 = (*code.get((pc+2) as usize).unwrap()) as u16;
    let index = indexbyte1 + indexbyte2;

    match class.constants.get(index as usize).unwrap() {
        &Constant::Fieldref(ref class_name, ref field_name, ref type_name) => {
            trace!("getstatic: {}.{}{}", class_name, field_name, type_name);

            let class = vm.load_and_clinit_class(class_name);


//            let method = utils::find_method(&class, &method_name, &method_signature)
//                .unwrap_or_else(|| panic!("Method not found: {}.{}{}", class_path, method_name, method_signature));


            panic!("Class loaded!");

//            let class_val = class.constants.get(class_index as usize).unwrap();
//            class.constants.get(index as usize).unwrap()
//
//            let name_and_type = class.constants.get(name_and_type_index as usize).unwrap();
//
//            println!("{:?}, {:?}", class_val, name_and_type);
        },
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc+3)
}