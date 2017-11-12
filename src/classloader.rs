use std::collections::HashMap;
use std::path::PathBuf;
use glob::glob;

use classfile::Classfile;
use classfile::load_file;

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

        Classloader {
            cache,
            filepath_cache,
        }
    }

    pub fn get_classfile(&mut self, classpath: &String) -> Classfile {
        if self.cache.contains_key(classpath) {
            // Classfile already loaded
            self.cache.get(classpath).unwrap().clone()
        }
        else {
            // Try to load that file
            let filepath = self.filepath_cache.get(classpath)
                .unwrap_or_else(|| panic!("Class not found: {}", classpath));
            let classfile = load_file(filepath);
            self.cache.insert(classpath.clone(), classfile.clone());

            info!("Class {} loaded", classpath);

            classfile
        }
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