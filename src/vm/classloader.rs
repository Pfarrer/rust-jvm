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
        let filepath = self.filepath_cache.get(class_path)
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

        /*
        thread 'main' panicked at 'Instance {
    class_path: "java/lang/Class",
    fields: {
        "serialVersionUID": Long(
            0
        ),
        "declaredConstructors": Null,
        "newInstanceCallerCache": Nul
        "initted": Boolean(
            false
        ),
        "genericInfo": Null,
        "allPermDomain": Null,
        "cachedConstructor": Null,
        "declaredAnnotations": Null,
        "publicConstructors": Null,
        "declaredMethods": Null,
        "declaredPublicFields": Null,
        "enumConstantDirectory": Null
        "declaredFields": Null,
        "annotationType": Null,
        "enumConstants": Null,
        "useCaches": Boolean(
            false
        ),
        "ENUM": Int(
            0
        ),
        "reflectionFactory": Null,
        "declaredPublicMethods": Null
        "ANNOTATION": Int(
            0
        ),
        "SYNTHETIC": Int(
            0
        ),
        "EMPTY_ANNOTATIONS_ARRAY": Nu
        "annotations": Null,
        "serialPersistentFields": Nul
        "classRedefinedCount": Int(
            0
        ),
        "publicFields": Null,
        "lastRedefinedCount": Int(
            0
        ),
        "name": Null,
        "publicMethods": Null
    }
}', src/vm/classloader.rs:65:8
        */

        // Create instance ...
        // THISISSHIT Should be located in the following lambda
        let mut instance = Instance::new(vm, classfile);

        let rc_class_path = StringPool::intern(vm, class_path);

        // Get pooled String instance or create new instance
        vm.classloader.class_pool.entry(class_path.clone()).or_insert_with(|| {
            instance.fields.insert("name".to_string(), Primitive::Objectref(rc_class_path));

            Rc::new(RefCell::new(instance))
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