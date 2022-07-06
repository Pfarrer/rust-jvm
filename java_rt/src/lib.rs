use loader::ClassfileLoader;
use model::api::{Classloader, Parser};

pub fn make_classloader(parser: &impl Parser) -> impl Classloader {
    let rt_path = std::env::current_dir().unwrap().join("java_rt").join("rt_build");
    println!("{:?}", rt_path);
    ClassfileLoader::open(rt_path, parser).unwrap()
}
