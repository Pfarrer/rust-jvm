use std::collections::HashMap;
use std::path::PathBuf;
use glob::glob;

use classfile::Classfile;
use classfile::load_file;
use vm::Vm;
use vm::utils;

#[derive(Debug)]
pub struct Classloader {
    cache: HashMap<String, Classfile>,
    filepath_cache: HashMap<String, String>,
}

impl Classloader {

    pub fn new(paths: Vec<String>) -> Classloader {
        let cache = HashMap::new();
        let mut filepath_cache = HashMap::new();

        for path_string in paths {
            let path = PathBuf::from(path_string).canonicalize().unwrap();
            for fullpath_buf in find_all_classfile_paths(&path).iter() {
                let classpath = convert_fullpath_to_classpath(&path, fullpath_buf);
                filepath_cache.insert(classpath, String::from(fullpath_buf.to_str().unwrap()));
            }
        }

        trace!("Classloader created, {} classes found", filepath_cache.len());

        Classloader {
            cache,
            filepath_cache,
        }
    }

    pub fn get_class(&mut self, vm: &mut Vm, class_path: &String) -> Classfile {
        let filepath = self.filepath_cache.get(class_path)
            .unwrap_or_else(|| panic!("Class not found: {}", class_path));

        self.cache.entry(class_path.clone()).or_insert_with(|| {
            // Try to load that file: Parse classfile
            let classfile = load_file(filepath);

            trace!("Class {} loaded", class_path);

            // Initialize class if necessary
            if let Some(method) = utils::find_method(&classfile, &"<clinit>".to_string(), &"()V".to_string()) {
                panic!("Class {} contains <clinit> -> executing now: {:#?}", class_path, method);
            }

            classfile
        }).clone()
    }

}

fn convert_fullpath_to_classpath(rel_path: &PathBuf, fullpath_buf: &PathBuf) -> String {
    // Make absolute path relative
    let relative_filepath = fullpath_buf
        .strip_prefix(rel_path).unwrap().to_string_lossy().into_owned();

    // Remove .class extension
    let (classpath, _) = relative_filepath.split_at(relative_filepath.len() - 6);
    String::from(classpath)
}


fn find_all_classfile_paths(path: &PathBuf) -> Vec<PathBuf> {
    let fullpath = [path.clone().into_os_string().into_string().unwrap(), "**/*.class".to_string()].join("/");

    glob(&fullpath).expect("Failed to read files in classpath")
        .filter_map(Result::ok)
        .collect()
}