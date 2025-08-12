use ctor::ctor;
use model::prelude::*;
use pretty_assertions::assert_eq;
use rstest::rstest;
use rt::NativeClassloader;
use vm::{frame::VmFrameImpl, new_vm, utils::create_java_string, vm_thread::VmTheadImpl};

#[ctor]
fn foo() {
    env_logger::init();
}

#[rstest]
#[case("", 0)]
#[case("a", 97)]
#[case("1", 49)]
#[case("a1", 3056)]
#[case("Ĥēļlø Ŵörlď", -1580692628)]
fn hash_code(#[case] val: &str, #[case] expected: i32) {
    let vm = new_test_vm();
    let mut vm_thread: VmThread = VmThread::new(&vm, "test".to_string());
    let java_string = create_java_string(&mut vm_thread, val.to_string());

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Objectref(java_string));

    vm_thread.invoke_method(
        &"java/lang/String".to_string(),
        &"hashCode".to_string(),
        &"()I".to_string(),
        true,
    );

    let frame = vm_thread.frame_stack.last_mut().unwrap();
    assert_eq!(frame.stack_pop(), VmPrimitive::Int(expected));
}

#[rstest]
#[case("", "", true)]
#[case("a", "a", true)]
#[case("a", "b", false)]
#[case("hello", "world", false)]
fn equals(#[case] val1: &str, #[case] val2: &str, #[case] expected: bool) {
    let vm = new_test_vm();
    let mut vm_thread: VmThread = VmThread::new(&vm, "test".to_string());

    let java_string1 = create_java_string(&mut vm_thread, val1.to_string());
    let java_string2 = create_java_string(&mut vm_thread, val2.to_string());
    let frame = vm_thread.frame_stack.last_mut().unwrap();
    frame.stack_push(VmPrimitive::Objectref(java_string1));
    frame.stack_push(VmPrimitive::Objectref(java_string2));

    vm_thread.invoke_method(
        &"java/lang/String".to_string(),
        &"equals".to_string(),
        &"(Ljava/lang/Object;)Z".to_string(),
        true,
    );

    assert_eq!(
        vm_thread.frame_stack.last().unwrap().stack.last().unwrap(),
        &VmPrimitive::Int(if expected { 1 } else { 0 })
    );
}

fn new_test_vm() -> Vm {
    let parser = parser::ClassfileParser {};
    let classloader = NativeClassloader {
        classloader: Box::new(rt::make_classloader(&parser)),
    };

    new_vm(classloader)
}
