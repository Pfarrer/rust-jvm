mod classfile;

use std::env;

fn main() {
    if let Some(arg1) = env::args().nth(1) {
        let class = classfile::load_file(arg1);

        println!("{:#?}", class);
    }
    else {
        panic!("Expect class file as first parameter.");
    }
}