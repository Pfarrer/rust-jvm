use model::api::Classloader;
use prettytable::{format, Table};
use std::path::PathBuf;

pub fn run(class_paths: Vec<PathBuf>) {
    let parser = parser::ClassfileParser {};
    let classloader = loader::classloader_for_paths(class_paths, &parser).unwrap();

    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_NO_LINESEP_WITH_TITLE);
    table.set_titles(row!["Class", "Major.Minor"]);

    for classname in classloader.list_classes() {
        let class = classloader.get_class(classname).unwrap();
        table.add_row(row![
            classname,
            format!("{}.{}", class.version.major, class.version.minor)
        ]);
    }

    table.printstd();
}
