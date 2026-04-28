#[feature(try_blocks)]
use std::path::PathBuf;

use crate::{commands::Exec, context::Context};
use clap::{Arg, Parser, builder::Str};
use simple_logger::SimpleLogger;

mod commands;
mod context;
mod models;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: commands::Command,

    #[arg(short, long)]
    config_path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .init()
        .expect("failed to init the logger");

    let args = Args::parse();
    println!("{args:?}");

    let Args { cmd, config_path } = args;

    let ctx = Context { config_path };

    cmd.exec(ctx)
}
