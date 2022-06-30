use model::api::Parser;
use model::prelude::*;
use parser::*;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

fn testdata_path() -> std::path::PathBuf {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    Path::new(&manifest_dir).join("tests/testdata").to_owned()
}

pub fn parse_class_in_testdata(file_name: &str) -> JvmClass {
    let pathbuf = testdata_path().join(file_name);
    let file = File::open(pathbuf).unwrap();
    let mut reader = BufReader::new(file);

    ClassfileParser {}.parse(&mut reader)
}
