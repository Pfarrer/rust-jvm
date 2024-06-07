#[macro_use]
extern crate prettytable;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod execute;
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

    #[clap(arg_required_else_help = true)]
    Execute {
        #[clap(required = true)]
        main_class: String,

        #[clap(required = true, parse(from_os_str))]
        class_paths: Vec<PathBuf>,
    },
}

fn main() {
    env_logger::init();
    better_panic::install();

    let args = Args::parse();

    match args.command {
        Command::ListClasses { class_paths } => list_classes::run(class_paths),
        Command::Execute {
            main_class,
            class_paths,
        } => execute::run(main_class, class_paths),
    };
}
