use std::path::PathBuf;
use vm::Vm;

const MAIN_METHOD_NAME: &str = "main";
const MAIN_METHOD_SIGNATURE: &str = "([Ljava/lang/String;)V";

pub fn run(main_class: String, class_paths: Vec<PathBuf>) {
    let parser = parser::ClassfileParser {};
    let classloader = loader::classloader_for_paths(class_paths, &parser).unwrap();

    let mut vm = Vm::new(classloader);
    let mut thread = vm.spawn_thread();
    thread.(main_class, MAIN_METHOD_NAME, MAIN_METHOD_SIGNATURE);
}
