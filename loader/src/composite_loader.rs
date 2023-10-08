use model::api::{Classloader, NativeMethod};
use model::class::{ClassMethod, JvmClass};

pub struct CompositeLoader {
    composites: Vec<Box<dyn Classloader>>,
}

impl CompositeLoader {
    pub fn open(composites: Vec<Box<dyn Classloader>>) -> CompositeLoader {
        CompositeLoader { composites }
    }
}

impl Classloader for CompositeLoader {
    fn list_classes(&self) -> Vec<&str> {
        self.composites
            .iter()
            .flat_map(|composite| composite.list_classes())
            .collect()
    }

    fn get_class(&self, classpath: &str) -> Option<&JvmClass> {
        for loader in self.composites.iter() {
            let result = loader.get_class(classpath);
            if result.is_some() {
                return result;
            }
        }
        return None;
    }

    fn get_native_method(
        &self,
        jvm_class: &JvmClass,
        class_method: &ClassMethod,
    ) -> Option<NativeMethod> {
        for loader in self.composites.iter() {
            let result = loader.get_native_method(jvm_class, class_method);
            if result.is_some() {
                return result;
            }
        }
        return None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::class::JvmClass;

    struct MockLoader {
        pub value: Option<JvmClass>,
    }
    impl Classloader for MockLoader {
        fn get_class(&self, _classpath: &str) -> Option<&JvmClass> {
            self.value.as_ref()
        }
        fn list_classes(&self) -> std::vec::Vec<&str> {
            todo!()
        }
    }

    #[test]
    fn get_class_and_class_available() {
        let composites: Vec<Box<dyn Classloader>> = vec![
            Box::new(MockLoader { value: None }),
            Box::new(MockLoader {
                value: Some(Default::default()),
            }),
        ];

        let composite_loader = CompositeLoader { composites };
        let result = composite_loader.get_class("");
        assert!(result.is_some());
    }

    #[test]
    fn get_class_and_class_not_found() {
        let composites: Vec<Box<dyn Classloader>> = vec![
            Box::new(MockLoader { value: None }),
            Box::new(MockLoader { value: None }),
        ];

        let composite_loader = CompositeLoader { composites };
        let result = composite_loader.get_class("");
        assert!(result.is_none());
    }
}
