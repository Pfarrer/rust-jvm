#[macro_use]
extern crate prettytable;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{level_filters::LevelFilter, Level};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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
        #[clap(long, required = false, default_value = "ERROR")]
        vm_init_log_level: Level,

        #[clap(long, required = false, default_value = "INFO")]
        vm_exec_log_level: Level,

        #[clap(required = true)]
        main_class: String,

        #[clap(required = true, parse(from_os_str))]
        class_paths: Vec<PathBuf>,
    },
}

fn main() {
    let set_log_level_fn = tracing_init();
    better_panic::install();

    let args = Args::parse();
    match args.command {
        Command::ListClasses { class_paths } => list_classes::run(class_paths),
        Command::Execute {
            main_class,
            class_paths,
            vm_init_log_level,
            vm_exec_log_level,
        } => execute::run(
            main_class,
            class_paths,
            vm_init_log_level,
            vm_exec_log_level,
            set_log_level_fn,
        ),
    };
}

fn tracing_init() -> impl Fn(Level) {
    let inital_log_level_filter = LevelFilter::INFO;
    let (filter, reload_handle) = tracing_subscriber::reload::Layer::new(inital_log_level_filter);

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_ansi(true))
        .with(filter)
        .init();

    move |level: Level| {
        reload_handle
            .modify(|layer| {
                *layer = LevelFilter::from_level(level);
            })
            .unwrap()
    }
}
