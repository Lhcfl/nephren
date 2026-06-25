use std::collections::HashMap;

use crate::commands::Exec;
use crate::context::Context;
use crate::models::node::NodeId;
use crate::models::protocol::ProtocolInfo;
use clap::{Args, ValueEnum};
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
struct NodeRow {
    id: NodeId,
    kind: String,
    name: String,
    address: String,
    group: String,
}

impl Exec for List {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let config = ctx.load_config()?;

        let mut group_map = HashMap::new();

        for subscription in &config.subscriptions {
            group_map.insert(subscription.id, subscription.name.clone());
        }

        match self.style {
            ListStyle::Table => {
                let mut table = Table::new(config.nodes.iter().map(|row| NodeRow {
                    id: row.id,
                    name: row.name.clone(),
                    kind: row.protocol.kind().to_owned(),
                    address: row.protocol.address(),
                    group: match row.belongs_to {
                        Some(id) => group_map.get(&id).cloned().unwrap_or_default(),
                        None => Default::default(),
                    },
                }));

                table.with(Style::modern_rounded());

                println!("{table}");
            }
            ListStyle::Json => {
                println!("{}", serde_json::to_string_pretty(&config.nodes)?);
                return Ok(());
            }
            ListStyle::Rust => {
                println!("{:#?}", config.nodes);
                return Ok(());
            }
        }

        Ok(())
    }
}
