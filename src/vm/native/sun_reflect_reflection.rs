use vm::Vm;
use vm::classloader::Classloader;
use vm::primitive::Primitive;
use vm::utils;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "getCallerClass" => get_caller_class(vm, class_path, method_name, method_signature), // (I)Ljava/lang/Class;
        "getClassAccessFlags" => get_class_access_flags(vm, class_path, method_name, method_signature), // (Ljava/lang/Class;)I
        _ => panic!("Native implementation of method {}.{}{} missing.", class_path, method_name, method_signature),
    }
}

/// (I)Ljava/lang/Class;
fn get_caller_class(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

    /* Returns the class of the method realFramesToSkip frames up the stack (zero-based), ignoring
    frames associated with java.lang.reflect.Method.invoke() and its implementation. The first frame
    is that associated with this method, so getCallerClass(0) returns the Class object for
    sun.reflect.Reflection. Frames associated with java.lang.reflect.Method.invoke() and its
    implementation are completely ignored and do not count toward the number of "real" frames
    skipped. */
    let mut real_frames_to_skip = vm.frame_stack.last_mut().unwrap().stack_pop_int() as usize;

    let class_path = {
        let mut class_path = None;
        for frame in vm.frame_stack.iter().rev() {
            if frame.class_path == "java/lang/reflect/Method".to_string() && frame.method_name == "invoke" {
                warn!("Skip frame java/lang/reflect/Method.invoke");
            }
            else {
                real_frames_to_skip -= 1;
            }

            if real_frames_to_skip == 0 {
                class_path = Some(frame.class_path.clone());
                break;
            }
        };

        class_path.unwrap()
    };

    let return_value = Classloader::get_class(vm, &class_path);
    vm.frame_stack.last_mut().unwrap().stack_push(Primitive::Objectref(return_value));

    trace!("Pushed Objectref for caller class {} to stack", class_path);
}

/// (Ljava/lang/Class;)I
fn get_class_access_flags(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    trace!("Execute native {}.{}{}", class_path, method_name, method_signature);
    
    let class_path = {
        let frame = vm.frame_stack.last_mut().unwrap();
        let rc_class = frame.stack_pop_objectref();
        let class = rc_class.borrow();

        match class.fields.get("name").unwrap() {
            &Primitive::Objectref(ref rc_name_instance) => utils::get_java_string_value(&*rc_name_instance.borrow()),
            a => panic!("Not implemented for {:?}", a)
        }
    };

    let classfile = vm.load_and_clinit_class(&class_path);
    let access_flags = classfile.class_info.access_flags as i32;

    trace!("Popped Objectref from stack and pushed Int {} to stack", access_flags);
    vm.frame_stack.last_mut().unwrap().stack_push(Primitive::Int(access_flags));
}