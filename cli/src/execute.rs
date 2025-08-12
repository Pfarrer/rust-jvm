use std::path::PathBuf;

use loader::CompositeLoader;
use model::vm::VmThread;
use rt::bootstrap_vm;
use tracing::Level;
use vm::vm_thread::VmTheadImpl;

const MAIN_METHOD_NAME: &str = "main";
const MAIN_METHOD_SIGNATURE: &str = "([Ljava/lang/String;)V";

pub fn run(
    main_class: String,
    class_paths: Vec<PathBuf>,
    vm_init_log_level: Level,
    vm_exec_log_level: Level,
    set_log_level_fn: impl Fn(Level),
) {
    let parser = parser::ClassfileParser {};

    let classloader = loader::classloader_for_paths(class_paths, &parser).unwrap();
    let runtime_classloader = rt::make_classloader(&parser);

    let classloader =
        CompositeLoader::open(vec![Box::new(runtime_classloader), Box::new(classloader)]);

    set_log_level_fn(vm_init_log_level);
    let vm = bootstrap_vm(classloader);

    set_log_level_fn(vm_exec_log_level);
    VmThread::new(&vm, "Thread-0".to_string()).invoke_method(
        &main_class,
        &MAIN_METHOD_NAME.to_string(),
        &MAIN_METHOD_SIGNATURE.to_string(),
        false,
    );
}
