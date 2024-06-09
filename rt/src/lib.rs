use loader::ClassfileLoader;
use model::prelude::*;

mod native;

struct RtClassloader {
    classfile_loader: ClassfileLoader,
}

impl Classloader for RtClassloader {
    fn list_classes(&self) -> Vec<&str> {
        self.classfile_loader.list_classes()
    }
    
    fn get_class(&self, classpath: &str) -> Option<&JvmClass> {
        self.classfile_loader.get_class(classpath)
    }

    fn get_native_method(
        &self,
        jvm_class: &JvmClass,
        class_method: &ClassMethod,
    ) -> Option<NativeMethod> {
        native::get_method(jvm_class, class_method)
    }
}

pub fn make_classloader(parser: &impl Parser) -> impl Classloader {
    let rt_path = std::env::current_dir()
        .unwrap()
        .join("jmods/java.base/classes");

    RtClassloader {
        classfile_loader: ClassfileLoader::open(rt_path, parser).unwrap(),
    }
}
