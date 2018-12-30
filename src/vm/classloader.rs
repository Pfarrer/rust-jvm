use std::collections::HashMap;
use std::path::PathBuf;
use glob::glob;
use std::rc::Rc;
use std::cell::RefCell;

use classfile::Classfile;
use classfile::load_file;
use vm::Vm;
use vm::primitive::Primitive;
use vm::instance::Instance;
use vm::string_pool::StringPool;
use vm::signature::TypeSignature;

#[derive(Debug)]
pub struct Classloader {
    cache: HashMap<String, Classfile>,
    filepath_cache: HashMap<String, String>,

    class_pool: HashMap<String, Rc<RefCell<Instance>>>,
}

impl Classloader {
    pub fn new(paths: Vec<String>) -> Classloader {
        let cache = HashMap::new();
        let mut filepath_cache = HashMap::new();
        let class_pool = HashMap::new();

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

            class_pool,
        }
    }

    pub fn get_classfile(&mut self, class_path: &String) -> Classfile {
        let filepath = self.filepath_cache.get(&class_path.replace(".", "/"))
            .unwrap_or_else(|| panic!("Class not found: {}", class_path));

        self.cache.entry(class_path.clone()).or_insert_with(|| {
            // Try to load that file: Parse classfile
            let classfile = load_file(filepath);

            trace!("Class {} loaded", class_path);

            classfile
        }).clone()
    }

    pub fn get_class(vm: &mut Vm, class_path: &String) -> Rc<RefCell<Instance>> {
        let classfile = vm.classloader.get_classfile(&"java/lang/Class".to_string());

        // Create instance ...
        // THISISSHIT Should be located in the following lambda
        let mut instance = Instance::new(vm, classfile);

        let rc_class_path = StringPool::intern(vm, class_path);

        // Get pooled String instance or create new instance
        vm.classloader.class_pool.entry(class_path.clone()).or_insert_with(|| {
            instance.fields.insert("name".to_string(), Primitive::Objectref(rc_class_path.clone()));
//            instance.fields.insert("classLoader".to_string(), Primitive::Objectref(rc_class_path));

            Rc::new(RefCell::new(instance))
        }).clone()
    }

    pub fn get_class_by_type_signature(vm: &mut Vm, type_signature: &TypeSignature) -> Rc<RefCell<Instance>> {
        match type_signature {
            TypeSignature::Class(class_path) => Classloader::get_class(vm, class_path),
            TypeSignature::Array(_) => Classloader::get_class(vm, &type_signature.as_string()),
            TypeSignature::Int => Classloader::get_class(vm, &"integer".to_string()),
            TypeSignature::Char => Classloader::get_class(vm, &"character".to_string()),
            TypeSignature::Boolean => Classloader::get_class(vm, &"boolean".to_string()),
            TypeSignature::Long => Classloader::get_class(vm, &"long".to_string()),
            TypeSignature::Double => Classloader::get_class(vm, &"double".to_string()),
            TypeSignature::Float => Classloader::get_class(vm, &"float".to_string()),
            TypeSignature::Short => Classloader::get_class(vm, &"short".to_string()),
            TypeSignature::Byte => Classloader::get_class(vm, &"byte".to_string()),
            TypeSignature::Void => Classloader::get_class(vm, &"void".to_string()),
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
