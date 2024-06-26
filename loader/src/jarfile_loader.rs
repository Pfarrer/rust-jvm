use anyhow::anyhow;
use anyhow::Result;
use model::api::Classloader;
use model::api::Parser;
use model::class::JvmClass;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};
use zip::ZipArchive;

pub struct JarfileLoader {
    class_cache: HashMap<String, JvmClass>,
}

impl JarfileLoader {
    pub fn open(path: impl AsRef<Path>, parser: &impl Parser) -> Result<JarfileLoader> {
        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let mut archive = zip::ZipArchive::new(reader)?;

        let class_cache = parse_classfiles(&mut archive, parser)?
            .iter()
            .map(|(path, jvm_class)| {
                (
                    path.with_extension("").to_string_lossy().into_owned(),
                    jvm_class.to_owned(),
                )
            })
            .collect();

        Ok(JarfileLoader { class_cache })
    }
}

impl Classloader for JarfileLoader {
    fn list_classes(&self) -> Vec<&str> {
        self.class_cache.keys().map(|name| name.as_ref()).collect()
    }

    fn get_class(&self, classpath: &str) -> Option<&JvmClass> {
        self.class_cache.get(classpath)
    }
}

fn parse_classfiles(
    archive: &mut ZipArchive<BufReader<File>>,
    parser: &impl Parser,
) -> Result<Vec<(PathBuf, JvmClass)>> {
    (0..archive.len())
        .filter_map(|i| {
            let mut file = archive.by_index(i).unwrap();
            let path = file.enclosed_name()?.to_path_buf();

            let path_extension = path.extension();
            if path_extension.is_none() || path_extension.unwrap() != "class" {
                return None;
            }
            Some((path, parser.parse(&mut file)))
        })
        .map(|(path, class_result)| {
            class_result
                .map(|class| (path, class))
                .map_err(|err| anyhow!(err))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use model::class::JvmClass;
    use std::io::Read;

    struct MockParser {
        jvm_class: JvmClass,
    }
    impl Parser for MockParser {
        fn parse<T: Read>(&self, _: &mut T) -> Result<JvmClass> {
            Ok(self.jvm_class.clone())
        }
    }

    #[test]
    fn open_hello_world_jar() {
        let jvm_class: JvmClass = Default::default();
        let mock_parser = MockParser { jvm_class };

        let loader = JarfileLoader::open(self::testdata_path(), &mock_parser).unwrap();
        assert_eq!(1, loader.class_cache.len());
        loader.get_class("com/github/sushantmimani/App").unwrap();

        let result = loader.get_class("no/valid/Cp");
        assert!(result.is_none());
    }

    fn testdata_path() -> std::path::PathBuf {
        let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
        Path::new(&manifest_dir)
            .join("testdata")
            .join("HelloWorld-0.6.5.jar")
            .to_owned()
    }
}
