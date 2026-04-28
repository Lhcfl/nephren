use crate::commands::Exec;
use crate::context::Context;
use anyhow::bail;
use clap::{Args, ValueEnum};
use log::error;
use tabled::Table;

#[derive(Debug, Clone, Copy, ValueEnum)]
enum ListStyle {
    Table,
    Json,
    Rust,
}

#[derive(Debug, Args)]
pub struct List {
    #[arg(short, long, default_value = "table")]
    style: ListStyle,
}

impl Exec for List {
    fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let config = ctx.load_config()?;

        match self.style {
            ListStyle::Table => {
                let table = Table::new(&config.subscriptions);
                println!("{table}");
            }
            ListStyle::Json => {
                println!("{}", serde_json::to_string_pretty(&config.subscriptions)?);
                return Ok(());
            }
            ListStyle::Rust => {
                println!("{:#?}", config.subscriptions);
                return Ok(());
            }
        }

        Ok(())
    }
}
