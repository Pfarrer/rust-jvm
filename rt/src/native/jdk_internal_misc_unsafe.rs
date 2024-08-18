use log::{trace, warn};
use model::prelude::*;

use vm::{frame::VmFrameImpl, utils::get_java_string_value, vm_thread::VmTheadImpl};

pub fn get_method(_jvm_class: &JvmClass, class_method: &ClassMethod) -> Option<NativeMethod> {
    match class_method.name.as_str() {
        "registerNatives" => Some(register_natives), // ()V
        "arrayBaseOffset0" => Some(array_base_offset0), // (Ljava/lang/Class;)I
        "arrayIndexScale0" => Some(array_index_scale0), // (Ljava/lang/Class;)I
        "addressSize0" => Some(address_size0), // ()I
        "isBigEndian0" => Some(is_big_endian0), // ()Z
        "unalignedAccess0" => Some(unaligned_access0), // ()Z
        "objectFieldOffset1" => Some(object_field_offset1), // (Ljava/lang/Class;Ljava/lang/String;)J
        "loadFence" => Some(load_fence), // ()V
        "storeFence" => Some(store_fence), // ()V
        "fullFence" => Some(full_fence), // ()V
        "compareAndSetInt" => Some(compare_and_set_int), // (Ljava/lang/Object;JII)Z
        "compareAndSetLong" => Some(compare_and_set_long), // (Ljava/lang/Object;JJJ)Z
        "compareAndSetObject" => Some(compare_and_set_object), // (Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z
        "getObjectVolatile" => Some(get_object_volatile), //(Ljava/lang/Object;J)Ljava/lang/Object;

        _ => None,
    }
}

fn register_natives(_: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.registerNatives()V");
}

/// (Ljava/lang/Class;)I
fn array_base_offset0(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.arrayBaseOffset0(Ljava/lang/Class;)I");

    // Remove parameter from stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let _ = frame.stack_pop_objectref();

    warn!("Not properly implemented -> will always return 0");
    frame.stack_push(VmPrimitive::Int(0));
}

/// (Ljava/lang/Class;)I
fn array_index_scale0(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.arrayBaseOffset0(Ljava/lang/Class;)I");

    // Remove parameter from stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let _ = frame.stack_pop_objectref();

    warn!("Not properly implemented -> will always return 1");
    frame.stack_push(VmPrimitive::Int(1));
}

/// ()I
fn address_size0(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.addressSize0()I");

    let frame = vm_thread.frame_stack.last_mut().unwrap();

    warn!("Not properly implemented -> will always return 8");
    frame.stack_push(VmPrimitive::Int(8));
}

/// ()Z
fn is_big_endian0(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.isBigEndian0()Z");

    // Remove parameter from stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();

    let primitive = VmPrimitive::Boolean(cfg!(target_endian = "big"));
    frame.stack_push(primitive);
}

/// ()Z
fn unaligned_access0(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.unalignedAccess0()Z");

    // Remove parameter from stack
    let frame = vm_thread.frame_stack.last_mut().unwrap();

    warn!("Not properly implemented -> will always return false");
    frame.stack_push(VmPrimitive::Boolean(false));
}

/// (Ljava/lang/Class;Ljava/lang/String;)J
fn object_field_offset1(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.objectFieldOffset1(Ljava/lang/Class;Ljava/lang/String;)J");

    // Remove parameters from stack
    let (field_name, class_path) = {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        let rc_instance_string = frame.stack_pop_objectref();
        let rc_instance_class = frame.stack_pop_objectref();

        let field_name = get_java_string_value(&*rc_instance_string.borrow_mut());
        let class_path = {
            let class_instance = &*rc_instance_class.borrow_mut();
            assert_eq!(class_instance.class_path, "java/lang/Class");
    
            match &class_instance.fields["name"] {
                VmPrimitive::Objectref(ref rc_object) => get_java_string_value(&*rc_object.borrow_mut()),
                a => panic!("Expected Arrayref but found: {:?}", a),
            }
        };

        (field_name, class_path)
    };
    

    let jvm_class = vm_thread.load_and_clinit_class(&class_path);
    let (n, _) = jvm_class.fields.iter().enumerate().find(|(_, field)| field.name == field_name).unwrap();

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Long(n as i64));
}

/// ()V
fn load_fence(_: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.loadFence()V");
    warn!("Not properly implemented -> will do nothing");
}
/// ()V
fn store_fence(_: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.storeFence()V");
    warn!("Not properly implemented -> will do nothing");
}
/// ()V
fn full_fence(_: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.fullFence()V");
    warn!("Not properly implemented -> will do nothing");
}

/// (Ljava/lang/Object;JII)Z
fn compare_and_set_int(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.compareAndSetInt(Ljava/lang/Object;JII)Z");

    // Remove parameters from stack
    let (rc_instance, field_index, expected, x) = {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        let x = frame.stack_pop_int();
        let expected = frame.stack_pop_int();
        let field_index = frame.stack_pop_long();
        let rc_instance = frame.stack_pop_objectref();

        (rc_instance, field_index, expected, x)
    };

    let instance = &mut *rc_instance.borrow_mut();
    let jvm_class = vm_thread.load_and_clinit_class(&instance.class_path);

    let (_, field) = jvm_class.fields.iter().enumerate().find(|(n, _)| *n == field_index as usize).unwrap();

    let success = match instance.fields.get(&field.name).unwrap() {
        VmPrimitive::Int(ref actual) => if *actual == expected {
            instance.fields.insert(field.name.clone(), VmPrimitive::Int(x));
            true
        } else {
            false
        },
        a => panic!("Expected Int field in '{}' but found {:?}", field.name, a),
    };

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Boolean(success));
}

/// (Ljava/lang/Object;JJJ)Z
fn compare_and_set_long(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.compareAndSetLong(Ljava/lang/Object;JJJ)Z");

    // Remove parameters from stack
    let (rc_instance, field_index, expected, x) = {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        let x = frame.stack_pop_long();
        let expected = frame.stack_pop_long();
        let field_index = frame.stack_pop_long();
        let rc_instance = frame.stack_pop_objectref();

        (rc_instance, field_index as usize, expected, x)
    };

    let instance = &mut *rc_instance.borrow_mut();
    let jvm_class = vm_thread.load_and_clinit_class(&instance.class_path);

    let (_, field) = jvm_class.fields.iter().enumerate().find(|(n, _)| *n == field_index).unwrap();

    let success = match instance.fields.get(&field.name).unwrap() {
        VmPrimitive::Long(ref actual) => if *actual == expected {
            instance.fields.insert(field.name.clone(), VmPrimitive::Long(x));
            true
        } else {
            false
        },
        a => panic!("Expected Int field in '{}' but found {:?}", field.name, a),
    };

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Boolean(success));
}

/// (Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z
fn compare_and_set_object(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.compareAndSetObject(Ljava/lang/Object;JLjava/lang/Object;Ljava/lang/Object;)Z");

    // Remove parameters from stack
    let (rc_instance, offset, expected, x) = {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        let x = frame.stack_pop_reference();
        let expected = frame.stack_pop_reference();
        let offset = frame.stack_pop_long();
        let rc_instance = frame.stack_pop_reference();

        (rc_instance, offset as usize, expected, x)
    };

    let success = match rc_instance {
        VmPrimitive::Arrayref(ref rc_array) => {
            let mut array = rc_array.borrow_mut();
            let actual = array.elements.get(offset).unwrap();
            
            if actual == &expected {
                array.elements.insert(offset, x);
                true
            } else {
                false
            }
        },
        a => panic!("Not implemented for {:?}", a),
    };

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Boolean(success));
}

/// (Ljava/lang/Object;J)Ljava/lang/Object;
fn get_object_volatile(vm_thread: &mut VmThread) {
    trace!("Execute native jdk/internal/misc/Unsafe.getObjectVolatile(Ljava/lang/Object;J)Ljava/lang/Object;");

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    let offset = frame.stack_pop_long();
    let vm_object = frame.stack_pop();

    let rc_object = match vm_object {
        VmPrimitive::Arrayref(ref rc_array) => {
            rc_array.borrow_mut().elements.get(offset as usize).unwrap().clone()
        },
        a => panic!("Not implemented for {:?}", a),
    };

    frame.stack_push(rc_object);
}
