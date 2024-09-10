use ctor::ctor;
use model::prelude::*;
use rstest::rstest;
use rt::NativeClassloader;
use vm::{new_vm, vm_thread::VmTheadImpl};

#[ctor]
fn foo() {
    env_logger::init();
}

#[rstest]
fn default_charset() {
    let vm = new_test_vm();
    let mut vm_thread = VmThread::new(&vm, "test".to_string());

    vm_thread.invoke_method(
        &"java/nio/charset/Charset".to_string(),
        &"defaultCharset".to_string(),
        &"()Ljava/nio/charset/Charset;".to_string(),
        false,
    );

    // assert_eq!(vm_thread.frame_stack.last().unwrap().stack.last().unwrap(), &VmPrimitive::Int(expected));
}

fn new_test_vm() -> Vm {
    let parser = parser::ClassfileParser {};
    let classloader = NativeClassloader {
        classloader: Box::new(rt::make_classloader(&parser)),
    };

    new_vm(classloader)
}