extern crate glob;

#[macro_use]
extern crate log;
extern crate pretty_env_logger;

mod classfile;
mod classloader;
mod vm;

use std::env;

use classloader::Classloader;

fn main() {
    pretty_env_logger::init().unwrap();

    if env::args().len() < 3 {
        error!("Usage: <Class-to-Start> <Classpath1> [<Classpath2>, ...]");
        return;
    }

    let main_class = env::args().nth(1).unwrap();
    let search_paths: Vec<String> = env::args().skip(2).collect();
    let classloader = Classloader::new(search_paths);

    vm::interpret(classloader, main_class);

//    let Some(arg1) = .nth(1) {
//        let class = classfile::load_file(arg1);
//
//        println!("{:#?}", class);
//    }
//    else {
//        panic!("Expect class file as first parameter.");
//    }
}
