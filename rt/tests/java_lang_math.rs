use ctor::ctor;
use model::prelude::*;
use pretty_assertions::assert_eq;
use rstest::rstest;
use rt::NativeClassloader;
use vm::{new_vm, vm_thread::VmTheadImpl};

#[ctor]
fn foo() {
    env_logger::init();
}

#[rstest]
#[case::negative_4_and_3(-4, 3, -2)]
#[case::negative_3_and_3(-3, 3, -1)]
#[case::negative_2_and_3(-2, 3, -1)]
#[case::negative_1_and_3(-1, 3, -1)]
#[case::zero_and_3(0, 3, 0)]
#[case::positive_1_and_3(1, 3, 0)]
#[case::positive_2_and_3(2, 3, 0)]
#[case::positive_3_and_3(3, 3, 1)]
#[case::positive_6_and_3(6, 3, 2)]
#[case(-1069723481, 12, -89143624)]
fn floor_div_calculates_correctly(#[case] n1: i32, #[case] n2: i32, #[case] expected: i32) {
    let vm = new_test_vm();
    let mut vm_thread = VmThread::new(&vm, "test".to_string());

    vm_thread
        .frame_stack
        .last_mut()
        .unwrap()
        .stack
        .push(VmPrimitive::Int(n1));
    vm_thread
        .frame_stack
        .last_mut()
        .unwrap()
        .stack
        .push(VmPrimitive::Int(n2));
    vm_thread.invoke_method(
        &"java/lang/Math".to_string(),
        &"floorDiv".to_string(),
        &"(II)I".to_string(),
        false,
    );

    assert_eq!(
        vm_thread.frame_stack.last().unwrap().stack.last().unwrap(),
        &VmPrimitive::Int(expected)
    );
}

#[rstest]
#[case(0, 1.)]
#[case(10, 1024.)]
#[case(-1, 0.5)]
#[case(512, 1.3407807929942597e154)]
#[case(-512, 7.458340731200207e-155)]
fn power_of_two_d_calculates_correctly(#[case] n: i32, #[case] expected: f64) {
    let vm = new_test_vm();
    let mut vm_thread = VmThread::new(&vm, "test".to_string());

    vm_thread
        .frame_stack
        .last_mut()
        .unwrap()
        .stack
        .push(VmPrimitive::Int(n));
    vm_thread.invoke_method(
        &"java/lang/Math".to_string(),
        &"powerOfTwoD".to_string(),
        &"(I)D".to_string(),
        false,
    );

    assert_eq!(
        vm_thread.frame_stack.last().unwrap().stack.last().unwrap(),
        &VmPrimitive::Double(expected)
    );
}

#[rstest]
#[case(0, 1.)]
#[case(10, 1024.)]
#[case(-1, 0.5)]
#[case(127, 1.7014118e38)]
#[case(-126, 1.17549435e-38)]
fn power_of_two_f_calculates_correctly(#[case] n: i32, #[case] expected: f32) {
    let vm = new_test_vm();
    let mut vm_thread = VmThread::new(&vm, "test".to_string());

    vm_thread
        .frame_stack
        .last_mut()
        .unwrap()
        .stack
        .push(VmPrimitive::Int(n));
    vm_thread.invoke_method(
        &"java/lang/Math".to_string(),
        &"powerOfTwoF".to_string(),
        &"(I)F".to_string(),
        false,
    );

    assert_eq!(
        vm_thread.frame_stack.last().unwrap().stack.last().unwrap(),
        &VmPrimitive::Float(expected)
    );
}

fn new_test_vm() -> Vm {
    let parser = parser::ClassfileParser {};
    let classloader = NativeClassloader {
        classloader: Box::new(rt::make_classloader(&parser)),
    };

    new_vm(classloader)
}
