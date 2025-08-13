use std::iter::Iterator;

use model::prelude::*;

use crate::vm_thread::VmTheadImpl;

pub type HierarchyClassInfo = (JvmClass, String, Vec<String>);

pub struct HierarchyIterator<'a, 'b> {
    vm_thread: &'a mut VmThread<'b>,
    current_class_path: Option<String>,
}

impl<'a, 'b> HierarchyIterator<'a, 'b> {
    pub fn hierarchy_iter(
        vm_thread: &'a mut VmThread<'b>,
        class_path: &String,
    ) -> HierarchyIterator<'a, 'b> {
        HierarchyIterator {
            vm_thread,
            current_class_path: Some(class_path.clone()),
        }
    }
}

impl<'a, 'b> Iterator for HierarchyIterator<'a, 'b> {
    type Item = HierarchyClassInfo;

    fn next(&mut self) -> Option<HierarchyClassInfo> {
        match self.current_class_path {
            Some(ref current_class_path) => {
                let current_class = self.vm_thread.load_and_clinit_class(current_class_path);
                let current_hierarchy_info = make_hierarchy_class_info(&current_class);

                match &current_hierarchy_info {
                    &Some((_, ref super_class_path, _)) => {
                        self.current_class_path = Some(super_class_path.clone())
                    }
                    &None => self.current_class_path = None,
                }

                current_hierarchy_info.or(Some((current_class, "".to_owned(), Vec::new())))
            }
            None => None,
        }
    }
}

fn make_hierarchy_class_info(class: &JvmClass) -> Option<HierarchyClassInfo> {
    // Find super class
    let super_class_path = match class.super_class {
        Some(ref path) => path.clone(),
        None => return None,
    };

    Some((class.clone(), super_class_path, class.interfaces.clone()))
}

// #[derive(Debug)]
// pub struct ClassHierarchy {
//     hierarchy_cache: HashMap<String, Option<HierarchyClassInfo>>,
// }
//
// impl ClassHierarchy {
//     pub fn new() -> ClassHierarchy {
//         ClassHierarchy {
//             hierarchy_cache: HashMap::new(),
//         }
//     }
//
//     pub fn hierarchy_iter<'a>(vm: &'a Vm, class_path: &String) -> HierarchyIterator<'a> {
//         HierarchyIterator {
//             vm,
//             current_class_path: Some(class_path.clone()),
//         }
//     }
// }
//
// pub type HierarchyClassInfo = (JvmClass, String, Vec<String>);
//
// pub struct HierarchyIterator<'a> {
//     vm: &'a Vm,
//     current_class_path: Option<String>,
// }
//
// impl<'a> Iterator for HierarchyIterator<'a> {
//     type Item = HierarchyClassInfo;
//
//     fn next(&mut self) -> Option<HierarchyClassInfo> {
//         self.current_class_path.map(|ref current_class_path| {
//             let current_class = self.vm.load_and_clinit_class(current_class_path);
//             let current_hierarchy_info = self.vm.class_hierarchy.hierarchy_cache
//                 .entry(current_class_path.clone())
//                 .or_insert_with(|| make_hierarchy_class_info(current_class))
//                 .to_owned();
//
//             match &current_hierarchy_info {
//                 &Some((_, ref super_class_path, _)) => self.current_class_path = Some(super_class_path.clone()),
//                 &None => self.current_class_path = None
//             }
//
//             if current_hierarchy_info.is_some() {
//                 return current_hierarchy_info
//             } else {
//                 let current_class = self.vm.load_and_clinit_class(current_class_path).unwrap();
//                 Some((current_class, "".to_owned(), Vec::new()))
//             }
//         }).flatten()
//     }
// }
//
// fn make_hierarchy_class_info(class: JvmClass) -> Option<HierarchyClassInfo> {
//     // Find super class
//     let super_class_path = match class.class_info.super_class {
//         Some(ref path) => path.clone(),
//         None => return None,
//     };
//
//     // Also get all interfaces
//     let interface_paths = class.class_info.interfaces.iter().map(|interface_index| {
//         match class.constants.get(*interface_index as usize).unwrap() {
//             &ClassConstant::Class(ref path) => path.clone(),
//             it => panic!("Unexpected constant value: {:?}", it),
//         }
//     }).collect();
//
//     Some((class, super_class_path, interface_paths))
// }
