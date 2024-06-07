use anyhow::{anyhow, Context, Result};
use glob::glob;
use log::debug;
use model::api::Parser;
use model::class::JvmClass;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use model::api::Classloader;

pub struct ClassfileLoader {
    class_cache: HashMap<String, JvmClass>,
}

impl ClassfileLoader {
    pub fn open(path: impl AsRef<Path>, parser: &impl Parser) -> Result<ClassfileLoader> {
        let class_cache = find_all_classfile_paths(path.as_ref())?
            .iter()
            .filter(|file_path| !file_path.ends_with("module-info.class"))
            .map(|file_path| {
                let file = File::open(file_path)?;
                let mut reader = BufReader::new(file);

                let file_path_no_ext = file_path.with_extension("");
                let classpath = abs_to_rel_path(path.as_ref(), &file_path_no_ext);

                debug!("Parsing classfile {}", file_path.display());

                parser
                    .parse(&mut reader)
                    .with_context(|| format!("parse classfile {}", file_path.display()))
                    .map(|class| (classpath, class))
            })
            .collect::<Result<_>>()?;

        Ok(ClassfileLoader { class_cache })
    }
}

impl Classloader for ClassfileLoader {
    fn list_classes(&self) -> Vec<&str> {
        self.class_cache.keys().map(|name| name.as_ref()).collect()
    }

    fn get_class(&self, classpath: &str) -> Option<&JvmClass> {
        self.class_cache.get(classpath)
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

    let paths: Vec<PathBuf> = glob(&fullpath)?.filter_map(Result::ok).collect();

    Ok(paths)
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
        loader.get_class("fundamentals/Empty").unwrap();

        let result = loader.get_class("no/valid/Cp");
        assert!(result.is_none());
    }

    fn testdata_path() -> std::path::PathBuf {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&manifest_dir).join("testdata").to_owned()
    }
}
