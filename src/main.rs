extern crate glob;

mod classfile;
mod classloader;

use std::env;

use classloader::Classloader;

fn main() {
    if env::args().len() < 3 {
        println!("Usage: <Class-to-Start> <Classpath1> [<Classpath2>, ...]");
        return;
    }

    let main_class = env::args().nth(1).unwrap();
    let search_paths: Vec<String> = env::args().skip(2).collect();
    let classloader = Classloader::new(search_paths);

//    let Some(arg1) = .nth(1) {
//        let class = classfile::load_file(arg1);
//
//        println!("{:#?}", class);
//    }
//    else {
//        panic!("Expect class file as first parameter.");
//    }
}
