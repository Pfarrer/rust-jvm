#[macro_use]
extern crate prettytable;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod list_classes;

#[derive(Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[clap(arg_required_else_help = true)]
    ListClasses {
        #[clap(required = true, parse(from_os_str))]
        class_paths: Vec<PathBuf>,
    },
}

fn main() {
    let args = Args::parse();

    // let classloader = Classloader::
    match args.command {
        Command::ListClasses { class_paths } => list_classes::run(class_paths),
    };
}
