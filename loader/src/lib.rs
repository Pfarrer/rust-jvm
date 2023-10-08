use anyhow::{bail, Result};
use model::api::{Classloader, Parser};
use std::error::Error;
use std::ffi::OsStr;
use std::path::Path;

mod classfile_loader;
mod composite_loader;
mod jarfile_loader;

pub use classfile_loader::ClassfileLoader;
pub use composite_loader::CompositeLoader;

pub fn classloader_for_paths(
    paths: Vec<impl AsRef<Path>>,
    parser: &impl Parser,
) -> Result<CompositeLoader, Box<dyn Error>> {
    let composite_results: Vec<Result<_, _>> = paths
        .iter()
        .map(|path| classloader_for_path(path, parser))
        .collect();

    let composites = composite_results
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?;

    Ok(CompositeLoader::open(composites))
}

pub fn classloader_for_path(
    path: impl AsRef<Path>,
    parser: &impl Parser,
) -> Result<Box<dyn Classloader>> {
    let path_ref = path.as_ref();

    let classloader: Box<dyn Classloader> = if path_ref.is_dir() {
        Box::new(classloader_for_directory(path, parser)?)
    } else if let Some("jar") = path_ref.extension().and_then(OsStr::to_str) {
        Box::new(classloader_for_jar_file(path, parser)?)
    } else {
        bail!(
            "Unsupported classpath given: {}",
            path_ref.to_str().unwrap_or("Invalid path")
        );
    };

    Ok(classloader)
}

pub fn classloader_for_directory(
    path: impl AsRef<Path>,
    parser: &impl Parser,
) -> Result<CompositeLoader> {
    let classfile_loader = classfile_loader::ClassfileLoader::open(path, parser)?;

    let composites: Vec<Box<dyn Classloader>> = vec![Box::new(classfile_loader)];
    Ok(CompositeLoader::open(composites))
}

pub fn classloader_for_jar_file(
    path: impl AsRef<Path>,
    parser: &impl Parser,
) -> Result<jarfile_loader::JarfileLoader> {
    jarfile_loader::JarfileLoader::open(path, parser)
}
