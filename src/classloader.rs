use std::collections::HashMap;
use glob::glob;

use classfile::Classfile;

pub struct Classloader {
    cache: HashMap<String, Option<Classfile>>,
}

impl Classloader {

    pub fn new(path: String) -> Classloader {
        let mut cache = HashMap::with_capacity(availClassFilePaths.len());
        findAllClassFilePaths(path).for_each(|classfile_path| {
            cache.insert(classfile_path, Option::None)
        });

        Classloader {
            cache
        }
    }

    fn findAllClassFilePaths(path: String) -> Iter<String> {
        let mut paths = Vec::new();

        for entry in glob(path + "**/*.class").expect("Failed to read files in classpath " + path) {
            match entry {
                Ok(path) => paths.push(path),
                Error(err) => panic!("Failed to read file: "+err),
            }
        }

        paths
    }

}