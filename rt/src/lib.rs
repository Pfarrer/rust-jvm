use loader::ClassfileLoader;
use model::prelude::*;

pub fn make_classloader(parser: &impl Parser) -> impl Classloader {
    let rt_path = std::env::current_dir()
        .unwrap()
        .join("jmods/java.base/classes");

    ClassfileLoader::open(rt_path, parser).unwrap()
}
