use std::iter::Iterator;
use std::collections::HashMap;

use classfile::Classfile;
use classfile::constants::Constant;
use vm::Vm;

pub type HierarchyClassInfo = (Classfile, String, Vec<String>);

pub struct HierarchyIterator<'a> {
    vm: &'a mut Vm,
    next_class_path: Option<String>,
}

impl<'a> Iterator for HierarchyIterator<'a> {
    type Item = HierarchyClassInfo;

    fn next(&mut self) -> Option<HierarchyClassInfo> {
        let next_class_path = self.next_class_path.clone();
        match next_class_path {
            Some(ref class_path) => {
                let class = self.vm.load_and_clinit_class(class_path);
                let hierarchy_info = self.vm.class_hierarchy.hierarchy_cache
                    .entry(class_path.clone())
                    .or_insert_with(|| make_hierarchy_class_info(class))
                    .to_owned();

                match &hierarchy_info {
                    &Some((_, ref super_class_path, _)) => self.next_class_path = Some(super_class_path.clone()),
                    &None => self.next_class_path = None,
                }

                hierarchy_info
            },
            None => None,
        }
    }
}

fn make_hierarchy_class_info(class: Classfile) -> Option<HierarchyClassInfo> {
    // Find super class
    let super_class_index = class.class_info.super_class;
    let super_class_path = match class.constants.get(super_class_index as usize).unwrap() {
        &Constant::Class(ref path) => path.clone(),
        &Constant::None() => return None,
        it => panic!("Unexpected constant value: {:?}", it),
    };

    // Also get all interfaces
    let interface_paths = class.class_info.interfaces.iter().map(|interface_index| {
        match class.constants.get(*interface_index as usize).unwrap() {
            &Constant::Class(ref path) => path.clone(),
            it => panic!("Unexpected constant value: {:?}", it),
        }
    }).collect();

    Some((class, super_class_path, interface_paths))
}

/// ClassHierarchy struct
#[derive(Debug)]
pub struct ClassHierarchy {
    hierarchy_cache: HashMap<String, Option<HierarchyClassInfo>>,
}

impl ClassHierarchy {
    pub fn new() -> ClassHierarchy {
        let hierarchy_cache = HashMap::new();

        ClassHierarchy {
            hierarchy_cache,
        }
    }

    pub fn hierarchy_iter<'a>(vm: &'a mut Vm, class_path: &String) -> HierarchyIterator<'a> {
        HierarchyIterator {
            vm,
            next_class_path: Some(class_path.clone()),
        }
    }
}