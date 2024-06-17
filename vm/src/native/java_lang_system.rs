// extern crate time;
// extern crate dirs;

use std::env;

use log::trace;
use model::prelude::*;

use crate::{frame::VmFrameImpl, utils, vm_mem::VmStringPoolImpl, vm_thread::VmTheadImpl};

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "registerNatives" => Some(register_natives),
        "initProperties" => Some(init_properties), // (Ljava/util/Properties;)Ljava/util/Properties;

        // "currentTimeMillis" => current_time_millis(vm, class_path, method_name, method_signature), // ()J
        // "nanoTime" => nano_time(vm, class_path, method_name, method_signature), // ()J
        
        // "setIn0" => set_in0(vm, class_path, method_name, method_signature), // (Ljava/io/InputStream;)V
        // "setOut0" => set_out0(vm, class_path, method_name, method_signature), // (Ljava/io/PrintStream;)V
        // "setErr0" => set_err0(vm, class_path, method_name, method_signature), // (Ljava/io/PrintStream;)V
        // "arraycopy" => arraycopy(vm, class_path, method_name, method_signature), // (Ljava/lang/Object;ILjava/lang/Object;II)V
        // "mapLibraryName" => map_library_name(vm, class_path, method_name, method_signature), // (Ljava/lang/String;)Ljava/lang/String;
        _ => None,
    }
}

fn register_natives(_: &mut VmThread) {
    trace!("Execute native java/lang/System.registerNatives()V");
}

/// (Ljava/util/Properties;)Ljava/util/Properties;
fn init_properties(vm_thread: &mut VmThread) {
    trace!("Execute native java/lang/System.initProperties(Ljava/util/Properties;)Ljava/util/Properties;");
    warn!("This method is only partially implemented!");

//    set_property(vm, "sun.stdout.encoding", "UTF-8");
//    set_property(vm, "sun.stderr.encoding", "UTF-8");
    set_property(vm_thread, "file.encoding", "UTF-8");

    set_property(vm_thread, "line.separator", "\n");
    set_property(vm_thread, "file.separator", "/");
    set_property(vm_thread, "path.separator", ":");

    let java_home_pathbuf = env::current_dir().unwrap();
    set_property(vm_thread, "java.home",  java_home_pathbuf.to_str().unwrap());

    let user_dir_pathbuf = dirs::home_dir().unwrap();
    set_property(vm_thread, "user.dir",  user_dir_pathbuf.to_str().unwrap());

    fn set_property(vm_thread: &mut VmThread, key: &str, value: &str) {
        // Intern key and value first
        let rc_interned_key = vm_thread.vm.mem.string_pool.intern(vm_thread, &key.to_string());
        let rc_interned_value = vm_thread.vm.mem.string_pool.intern(vm_thread, &value.to_string());

        {
            let frame = vm_thread.frame_stack.last_mut().unwrap();

            // Clone instance first
            let value = frame.stack_pop();
            frame.stack_push(value.clone());
            frame.stack_push(value);

            // Push the key to the stack
            frame.stack_push(VmPrimitive::Objectref(rc_interned_key));

            // Push the value to the stack
            frame.stack_push(VmPrimitive::Objectref(rc_interned_value));
        }

        // Invoke the setProperty method
        vm_thread.invoke_method(&"java/util/Properties".to_string(), &"setProperty".to_string(),
                             &"(Ljava/lang/String;Ljava/lang/String;)Ljava/lang/Object;".to_string(), true);

        // Pop return value from stack
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        frame.stack_pop();
    }
}

// fn current_time_millis(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
//     trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

//     let millis_int = if vm.initialized {
//         let time_spec = time::get_time();

//         // 1459440009.113178
//         let millis_float: f64 = time_spec.sec as f64 + (time_spec.nsec as f64 / 1000.0 / 1000.0 / 1000.0);
//         (millis_float * 1000.0) as i64
//     }
//     else { -1 };

//     // Push result to stack
//     let frame = vm.frame_stack.last_mut().unwrap();
//     frame.stack_push(VmPrimitive::Long(millis_int));
// }

// fn nano_time(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
//     trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

//     let nano_time = time::precise_time_ns();

//     // Push result to stack
//     let frame = vm.frame_stack.last_mut().unwrap();
//     frame.stack_push(VmPrimitive::Long(nano_time as i64));
// }

// /// arraycopy(Ljava/lang/Object;ILjava/lang/Object;II)V
// fn arraycopy(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
//     trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

//     let frame = vm.frame_stack.last_mut().unwrap();

//     let length = frame.stack_pop_int() as usize;
//     let dest_pos = frame.stack_pop_int() as usize;
//     let rc_dest_array = frame.stack_pop_arrayref();
//     let src_pos = frame.stack_pop_int() as usize;
//     let rc_src_array = frame.stack_pop_arrayref();

//     let mut dest_array = rc_dest_array.borrow_mut();
//     let src_array = rc_src_array.borrow();

//     for i in 0..length {
//         dest_array.elements[dest_pos + i] = src_array.elements[src_pos + i].clone();
//     }
// }

// // (Ljava/io/InputStream;)V
// fn set_in0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
//     trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

//     let frame = vm.frame_stack.last_mut().unwrap();
//     let rc_stream = frame.stack_pop_objectref();

//     vm.class_statics.get_mut(class_path).unwrap()
//         .insert("in".to_string(), VmPrimitive::Objectref(rc_stream));
// }

// // (Ljava/io/PrintStream;)V
// fn set_out0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
//     trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

//     let frame = vm.frame_stack.last_mut().unwrap();
//     let rc_stream = frame.stack_pop_objectref();

//     vm.class_statics.get_mut(class_path).unwrap()
//         .insert("out".to_string(), VmPrimitive::Objectref(rc_stream));
// }

// // (Ljava/io/PrintStream;)V
// fn set_err0(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
//     trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

//     let frame = vm.frame_stack.last_mut().unwrap();
//     let rc_stream = frame.stack_pop_objectref();

//     vm.class_statics.get_mut(class_path).unwrap()
//         .insert("err".to_string(), VmPrimitive::Objectref(rc_stream));
// }

// /// (Ljava/lang/String;)Ljava/lang/String;
// fn map_library_name(vm: &mut Vm, class_path: &String, method_name: &String, method_signature: &String) {
//     trace!("Execute native {}.{}{}", class_path, method_name, method_signature);

//     let libname =  {
//         let frame = vm.frame_stack.last_mut().unwrap();
//         let rc_string = frame.stack_pop_objectref();
//         let libname_string = rc_string.borrow();

//         utils::get_java_string_value(&*libname_string)
//     };

//     assert_eq!("zip", libname);

//     let mapped_libname = "libzip.so".to_string();
//     let rc_mapped_libname = VmStringPool::intern(vm, &mapped_libname);

//     trace!("Popped an Objectref (String {}) from stack and push Objecref (String {}) back to the stack", libname, mapped_libname);

//     let frame = vm.frame_stack.last_mut().unwrap();
//     frame.stack_push(VmPrimitive::Objectref(rc_mapped_libname));
// }
