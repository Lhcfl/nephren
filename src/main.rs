use std::path::PathBuf;

use crate::{commands::Exec, context::Context};
use clap::Parser;
use simple_logger::SimpleLogger;

mod commands;
mod context;
mod inter;
mod models;
mod parse;
mod theme;

///                    _                    
///   _ __   ___ _ __ | |__  _ __ ___ _ __  
///  | '_ \ / _ \ '_ \| '_ \| '__/ _ \ '_ \ 
///  | | | |  __/ |_) | | | | | |  __/ | | |
///  |_| |_|\___| .__/|_| |_|_|  \___|_| |_|
///             |_|                         
///
/// nephren is a command-line v2ray config switch tool.
#[derive(Parser, Debug)]
#[clap(styles = theme::CARGO_STYLING)]
struct Args {
    #[command(subcommand)]
    cmd: commands::Command,

    #[arg(short, long)]
    config_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .init()
        .expect("failed to init the logger");

    let args = Args::parse();
    println!("{args:?}");

    let Args { cmd, config_path } = args;

    let ctx = Context { config_path };

    cmd.exec(ctx).await
}
