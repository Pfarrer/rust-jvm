use std::collections::HashMap;
use glob::glob;

use classfile::Classfile;

pub struct Classloader {
    cache: HashMap<String, Option<Classfile>>,
}

impl Classloader {

    pub fn new(path: String) -> Classloader {
        let mut cache = HashMap::new();
        for classfile_path in Classloader::find_all_classfile_paths(path).iter() {
            cache.insert(*classfile_path, Option::None);
        }

        Classloader {
            cache
        }
    }

    fn find_all_classfile_paths(path: String) -> Vec<String> {
        let mut paths = Vec::new();

        let fullpath = [path, "**/*.class".to_string()].join("");
        for entry in glob(&fullpath).expect("Failed to read files in classpath") {
            match entry {
                Ok(path) => paths.push(path.into_os_string().into_string().unwrap()),
                Err(err) => panic!("Failed to read file"),
            }
        }

        paths
    }

}