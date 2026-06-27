use std::path::PathBuf;

use crate::{commands::Exec, context::Context};
use clap::Parser;
use simple_logger::SimpleLogger;

mod actions;
mod commands;
mod context;
mod models;
mod share;
mod theme;

#[derive(Parser, Debug)]
#[clap(
    styles = theme::CARGO_STYLING,
    about = r"
                  _                    
 _ __   ___ _ __ | |__  _ __ ___ _ __  
| '_ \ / _ \ '_ \| '_ \| '__/ _ \ '_ \
| | | |  __/ |_) | | | | | |  __/ | | |
|_| |_|\___| .__/|_| |_|_|  \___|_| |_|
            |_|                         

nephren is a command-line v2ray config switch tool.
    "
)]
struct Args {
    #[command(subcommand)]
    cmd: commands::Command,

    #[arg(short, long)]
    state_path: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    SimpleLogger::new()
        .with_colors(true)
        .without_timestamps()
        .init()
        .expect("failed to init the logger");

    let Args { cmd, state_path } = Args::parse();
    let ctx = Context { state_path };
    cmd.exec(ctx).await
}
