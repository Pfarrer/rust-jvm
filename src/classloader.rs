use std::collections::HashMap;
use std::path::PathBuf;
use glob::glob;

use classfile::Classfile;
use classfile::load_file;

#[derive(Debug)]
pub struct Classloader {
    cache: HashMap<String, Option<Classfile>>,
    filepath_cache: HashMap<String, String>,
}

impl Classloader {

    pub fn new(paths: Vec<String>) -> Classloader {
        let mut cache = HashMap::new();
        let mut filepath_cache = HashMap::new();

        for path_string in paths {
            let path = PathBuf::from(path_string).canonicalize().unwrap();
            for fullpath_buf in find_all_classfile_paths(&path).iter() {
                let classpath = convert_fullpath_to_classpath(&path, fullpath_buf);
                cache.insert(classpath.clone(), Option::None);
                filepath_cache.insert(classpath, String::from(fullpath_buf.to_str().unwrap()));
            }
        }

        println!("Classloader created, {} classes found...", cache.len());

        Classloader {
            cache,
            filepath_cache,
        }
    }

    pub fn get_classfile(&mut self, classpath: &String) -> Classfile {
        match self.cache.get(classpath) {
            Some(classfile_opt) => {
                // Check if class was already loaded
                match *classfile_opt {
                    Some(ref classfile) => classfile.clone(),
                    None => {
                        // Try to load that file
                        let filepath = self.filepath_cache.get(classpath).unwrap();
                        let classfile = load_file(filepath);
                        //self.cache.insert(classpath.clone(), Some(classfile.clone()));

                        println!("Classfile loaded: {}", classpath);

                        classfile
                    }
                }
            },
            None => panic!("Required class not found: {}", classpath)
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