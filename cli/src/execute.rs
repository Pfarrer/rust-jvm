use crate::runtime_options::RuntimeOptions;
use std::path::PathBuf;

const MAIN_METHOD_NAME: &str = "main";
const MAIN_METHOD_SIGNATURE: &str = "([Ljava/lang/String;)V";

pub fn run(main_class: String, class_paths: Vec<PathBuf>, runtime_option: Option<RuntimeOptions>) {
    let parser = parser::ClassfileParser {};
    let classloader = loader::classloader_for_paths(class_paths, &parser).unwrap();
    // let classloader: CompositeLoader = if let Some(runtime) = runtime_option {
    //     let runtime_classloader: Box<dyn Classloader> = match runtime {
    //         RuntimeOptions::Native => Box::new(native_rt::make_classloader()),
    //         RuntimeOptions::Java => Box::new(java_rt::make_classloader(&parser)),
    //     };
    //     CompositeLoader::open(vec![runtime_classloader, Box::new(classloader)])
    // } else {
    //     classloader
    // };

    // let vm = Vm::new(classloader);
    // vm.spawn_thread("Thread-0".to_string()).invoke_method(
    //     &main_class,
    //     &MAIN_METHOD_NAME.to_string(),
    //     &MAIN_METHOD_SIGNATURE.to_string(),
    //     false,
    // );
}
