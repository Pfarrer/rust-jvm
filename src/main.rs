extern crate glob;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod classfile;
mod vm;

use std::env;

const MAIN_METHOD_NAME: &str = "main";
const MAIN_METHOD_SIGNATURE: &str = "([Ljava/lang/String;)V";

fn main() {
    pretty_env_logger::init().unwrap();

    if env::args().len() < 3 {
        error!("Usage: <Class-to-Start> <Classpath1> [<Classpath2>, ...]");
        return;
    }

    let main_class = env::args().nth(1).unwrap();
    let search_paths: Vec<String> = env::args().skip(2).collect();

    vm::Vm::new(search_paths)
        .invoke_static(&main_class,
                       &MAIN_METHOD_NAME.to_string(),
                       &MAIN_METHOD_SIGNATURE.to_string());
}
