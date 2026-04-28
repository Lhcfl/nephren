use crate::context::Context;
use crate::{commands::Exec, models::subscription::SubscriptionId};
use anyhow::bail;
use clap::{Args, ValueEnum};
use log::error;
use tabled::settings::Style;
use tabled::{Table, Tabled};

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

#[derive(Tabled)]
struct SubscriptionRow {
    id: SubscriptionId,
    name: String,
    url: String,
    description: String,
    enable_update: &'static str,
}

impl Exec for List {
    fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let config = ctx.load_config()?;

        match self.style {
            ListStyle::Table => {
                let mut table =
                    Table::new(config.subscriptions.iter().map(|row| SubscriptionRow {
                        id: row.id,
                        name: row.name.clone(),
                        url: row.url.clone().unwrap_or_else(|| "<none>".to_string()),
                        description: row.description.clone(),
                        enable_update: if row.enable_update { "√" } else { "" },
                    }));

                table.with(Style::modern_rounded());

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
