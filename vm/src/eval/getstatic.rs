use classfile::constants::Constant;
use classfile::Classfile;
use vm::class_hierarchy::ClassHierarchy;
use vm::primitive::Primitive;
use vm::utils;
use vm::Vm;

pub fn eval(vm: &Vm, class: &Classfile, code: &Vec<u8>, pc: u16) -> Option<u16> {
    let index = utils::read_u16_code(code, pc);
    match class.constants.get(index as usize).unwrap() {
        &Constant::Fieldref(ref class_path, ref field_name, ref type_name) => {
            vm.load_and_clinit_class(class_path);

            let value = find_static_value(vm, class_path, field_name);
            trace!(
                "getstatic: {}.{}{} -> push value to stack",
                class_path,
                field_name,
                type_name
            );

            let frame = vm.frame_stack.last_mut().unwrap();
            frame.stack_push(value);
        }
        it => panic!("Unexpected constant ref: {:?}", it),
    };

    Some(pc + 3)
}

fn find_static_value(vm: &Vm, root_class_path: &String, field_name: &String) -> Primitive {
    let mut class_paths = Vec::new();
    {
        let hierarchy_iter = ClassHierarchy::hierarchy_iter(vm, root_class_path);
        for (class, _, _) in hierarchy_iter {
            let class_path = utils::get_class_path(&class);
            class_paths.push(class_path);
        }
    }

    for class_path in class_paths {
        let value_option = vm.class_statics.get(&class_path).unwrap().get(field_name);

        if value_option.is_some() {
            return value_option.unwrap().clone();
        }
    }

    panic!("Static field not found");
}
