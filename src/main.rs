use crate::commands::Exec;
use clap::Parser;
use simple_logger::SimpleLogger;

mod commands;
mod models;

#[derive(Parser, Debug)]
struct Args {
    #[command(subcommand)]
    cmd: commands::Command,
}

fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_colors(true)
        .init()
        .expect("failed to init the logger");

    let args = Args::parse();
    println!("Hello, world!");
    println!("{args:?}");

    args.cmd.exec()
}
