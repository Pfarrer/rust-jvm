use std::{cell::RefCell, rc::Rc};

use ctor::ctor;
use model::prelude::*;
use rstest::rstest;
use pretty_assertions::assert_eq;
use rt::NativeClassloader;
use vm::{frame::VmFrameImpl, instance::VmInstanceImpl, new_vm, utils::{create_java_string, get_java_string_value}, vm_thread::VmTheadImpl};

#[ctor]
fn foo() {
    env_logger::init();
}

#[rstest]
fn set_and_get_float() {
    let vm = new_test_vm();
    let mut vm_thread: VmThread = VmThread::new(&vm, "test".to_string());
    let rc_map_instance = new_instance(&mut vm_thread, "java/util/concurrent/ConcurrentHashMap");
    
    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        frame.stack_push(VmPrimitive::Objectref(rc_map_instance.clone()));
    }
    push_float(&mut vm_thread, 0.);
    push_float(&mut vm_thread, 1.);
    vm_thread.invoke_method(
        &"java/util/concurrent/ConcurrentHashMap".to_string(),
        &"put".to_string(),
        &"(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;".to_string(),
        true,
    );
    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        assert_eq!(frame.stack_pop(), VmPrimitive::Null);
    }

    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        frame.stack_push(VmPrimitive::Objectref(rc_map_instance.clone()));
    }
    push_float(&mut vm_thread, 0.);
    push_float(&mut vm_thread, 2.);
    vm_thread.invoke_method(
        &"java/util/concurrent/ConcurrentHashMap".to_string(),
        &"put".to_string(),
        &"(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;".to_string(),
        true,
    );
    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        let rc_instance = frame.stack_pop_objectref();
        let float_instance = &*rc_instance.borrow_mut();
        assert_eq!(float_instance.fields["value"], VmPrimitive::Float(1.));
    }

    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        frame.stack_push(VmPrimitive::Objectref(rc_map_instance.clone()));
    }
    push_float(&mut vm_thread, 0.);
    vm_thread.invoke_method(
        &"java/util/concurrent/ConcurrentHashMap".to_string(),
        &"get".to_string(),
        &"(Ljava/lang/Object;)Ljava/lang/Object;".to_string(),
        true,
    );
    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        let rc_instance = frame.stack_pop_objectref();
        let float_instance = &*rc_instance.borrow_mut();
        assert_eq!(float_instance.fields["value"], VmPrimitive::Float(2.));
    }
}

#[rstest]
fn get_missing_float() {
    let vm = new_test_vm();
    let mut vm_thread: VmThread = VmThread::new(&vm, "test".to_string());
    let rc_map_instance = new_instance(&mut vm_thread, "java/util/concurrent/ConcurrentHashMap");
    
    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        frame.stack_push(VmPrimitive::Objectref(rc_map_instance.clone()));
    }
    push_float(&mut vm_thread, 0.);
    vm_thread.invoke_method(
        &"java/util/concurrent/ConcurrentHashMap".to_string(),
        &"get".to_string(),
        &"(Ljava/lang/Object;)Ljava/lang/Object;".to_string(),
        true,
    );
    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        assert_eq!(frame.stack_pop(), VmPrimitive::Null);
    }
}

#[rstest]
fn set_and_get_string() {
    let vm = new_test_vm();
    let mut vm_thread: VmThread = VmThread::new(&vm, "test".to_string());
    let rc_map_instance = new_instance(&mut vm_thread, "java/util/concurrent/ConcurrentHashMap");

    {
        let key = create_java_string(&mut vm_thread, "k".to_string());
        let val = create_java_string(&mut vm_thread, "v1".to_string());

        let frame = vm_thread.frame_stack.last_mut().unwrap();
        frame.stack_push(VmPrimitive::Objectref(rc_map_instance.clone()));
        frame.stack_push(VmPrimitive::Objectref(key.clone()));
        frame.stack_push(VmPrimitive::Objectref(val.clone()));
    }
    vm_thread.invoke_method(
        &"java/util/concurrent/ConcurrentHashMap".to_string(),
        &"put".to_string(),
        &"(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;".to_string(),
        true,
    );
    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        assert_eq!(frame.stack_pop(), VmPrimitive::Null);
    }

    {
        let key = create_java_string(&mut vm_thread, "k".to_string());
        let val = create_java_string(&mut vm_thread, "v2".to_string());

        let frame = vm_thread.frame_stack.last_mut().unwrap();
        frame.stack_push(VmPrimitive::Objectref(rc_map_instance.clone()));
        frame.stack_push(VmPrimitive::Objectref(key));
        frame.stack_push(VmPrimitive::Objectref(val));
    }
    vm_thread.invoke_method(
        &"java/util/concurrent/ConcurrentHashMap".to_string(),
        &"put".to_string(),
        &"(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;".to_string(),
        true,
    );
    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        let rc_instance = frame.stack_pop_objectref();
        let string_instance = &*rc_instance.borrow_mut();
        assert_eq!(get_java_string_value(string_instance).as_bytes(), "v1".as_bytes());
    }

    {
        let key = create_java_string(&mut vm_thread, "k".to_string());

        let frame = vm_thread.frame_stack.last_mut().unwrap();
        frame.stack_push(VmPrimitive::Objectref(rc_map_instance.clone()));
        frame.stack_push(VmPrimitive::Objectref(key));
    }
    vm_thread.invoke_method(
        &"java/util/concurrent/ConcurrentHashMap".to_string(),
        &"get".to_string(),
        &"(Ljava/lang/Object;)Ljava/lang/Object;".to_string(),
        true,
    );
    {
        let frame = vm_thread.frame_stack.last_mut().unwrap();
        let rc_instance = frame.stack_pop_objectref();
        let string_instance = &*rc_instance.borrow_mut();
        assert_eq!(get_java_string_value(string_instance), "v2");
    }
}

fn new_test_vm() -> Vm {
    let parser = parser::ClassfileParser {};
    let classloader = NativeClassloader {
        classloader: Box::new(rt::make_classloader(&parser)),
    };

    new_vm(classloader)
}

fn new_instance(vm_thread: &mut VmThread, class_path: &str) -> Rc<RefCell<VmInstance>> {
    let class = vm_thread.load_and_clinit_class(&class_path.to_string());
    let instance = VmInstance::new(vm_thread, &class);
    let rc_instance = Rc::new(RefCell::new(instance));

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Objectref(rc_instance.clone()));

    vm_thread.invoke_method(
        &class_path.to_string(),
        &"<init>".to_string(),
        &"()V".to_string(),
        true,
    );

    rc_instance
}

fn push_float(vm_thread: &mut VmThread, val: f32) {
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Float(val));

    vm_thread.invoke_method(
        &"java/lang/Float".to_string(),
        &"valueOf".to_string(),
        &"(F)Ljava/lang/Float;".to_string(),
        false,
    );
}