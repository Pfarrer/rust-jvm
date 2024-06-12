use anyhow::{anyhow, Result};
use glob::glob;
use log::{debug, trace};
use model::prelude::*;
use parser::ClassfileParser;
use std::cell::OnceCell;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

pub struct ClassfileLoader {
    class_cache: HashMap<String, (PathBuf, OnceCell<JvmClass>)>,
}

impl ClassfileLoader {
    pub fn open(path: impl AsRef<Path>, parser: &impl Parser) -> Result<ClassfileLoader> {
        let class_cache = find_all_classfile_paths(path.as_ref())?
            .iter()
            .filter(|file_path| !file_path.ends_with("module-info.class"))
            .map(|file_path| {
                trace!("Found classfile {}", file_path.display());

                let file_path_no_ext = file_path.with_extension("");
                let classpath = abs_to_rel_path(path.as_ref(), &file_path_no_ext);

                (classpath, (file_path.clone(), OnceCell::new()))
            })
            .collect();

        Ok(ClassfileLoader { class_cache })
    }
}

impl Classloader for ClassfileLoader {
    fn list_classes(&self) -> Vec<&str> {
        self.class_cache.keys().map(|name| name.as_ref()).collect()
    }

    fn get_class(&self, classpath: &str) -> Option<&JvmClass> {
        self.class_cache
        .get(classpath)
        .map(|(file_path, cell)| {
            let file = File::open(file_path).unwrap();
            let mut reader = BufReader::new(file);
            
            cell.get_or_init(|| {
                debug!("Parsing classfile {}...", file_path.display());

                ClassfileParser {}
                    .parse(&mut reader)
                    .unwrap()
            })
        })
    }
}

fn find_all_classfile_paths(path: &Path) -> Result<Vec<PathBuf>> {
    let fullpath = [
        path.as_os_str()
            .to_str()
            .ok_or(anyhow!("Invalid path given: {:?}", path))?,
        "**/*.class",
    ]
    .join(&format!("{}", std::path::MAIN_SEPARATOR));

    Ok(glob(&fullpath)?.filter_map(Result::ok).collect())
}

fn abs_to_rel_path(base_path: &Path, file_path: &Path) -> String {
    file_path
        .strip_prefix(base_path)
        .unwrap()
        .to_string_lossy()
        .into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::class::JvmClass;
    use std::io::Read;

    struct MockParser {
        jvm_class: JvmClass,
    }
    impl model::api::Parser for MockParser {
        fn parse<T: Read>(&self, _: &mut T) -> Result<JvmClass> {
            Ok(self.jvm_class.clone())
        }
    }

    #[test]
    fn get_fundamentals_empty_class() {
        let jvm_class: JvmClass = Default::default();
        let mock_parser = MockParser { jvm_class };

        let loader = ClassfileLoader::open(self::testdata_path(), &mock_parser).unwrap();
        assert_eq!(1, loader.class_cache.len());
        loader.get_class("fundamentals/EmptyClass").unwrap();

        let result = loader.get_class("no/valid/Cp");
        assert!(result.is_none());
    }

    fn testdata_path() -> std::path::PathBuf {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&manifest_dir).join("testdata").to_owned()
    }
}
