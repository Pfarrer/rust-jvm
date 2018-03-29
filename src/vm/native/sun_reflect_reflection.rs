use vm::Vm;
use vm::classloader::Classloader;
use vm::primitive::Primitive;

pub fn invoke(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
    match method_name.as_ref() {
        "getCallerClass" => get_caller_class(vm, class_path, method_name, method_signature), // (I)Ljava/lang/Class;
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

    debug!("{:#?}", vm.frame_stack);

//    let real_frames_to_skip = vm.frame_stack.last_mut().unwrap().stack_pop_int();
//    assert_eq!(real_frames_to_skip, 1);

    let class_path = {
        let frame_stack_len = vm.frame_stack.len();
        let parent_frame = &vm.frame_stack[frame_stack_len - 2];

        parent_frame.class_path.clone()
    };

//    let return_value = Classloader::get_class(vm, &class_path);
    let return_value = Classloader::get_class(vm, &"java/util/Random".to_string());
    vm.frame_stack.last_mut().unwrap().stack_push(Primitive::Objectref(return_value));
}