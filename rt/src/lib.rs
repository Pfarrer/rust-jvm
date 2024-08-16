use std::path::PathBuf;

use loader::ClassfileLoader;
use model::prelude::*;

pub fn make_classloader(parser: &impl Parser) -> impl Classloader {
    let rt_path = PathBuf::from("/home/brian/rust-jvm/rt/jmods/java.base/classes");

    ClassfileLoader::open(rt_path, parser).unwrap()
}

// pub fn initialize() {
//     VmThread::new(&vm, "vm-init".to_string()).invoke_method(
//         &"java/lang/System".to_string(),
//         &"initPhase1".to_string(),
//         &"()V".to_string(),
//         false,
//     );
// }