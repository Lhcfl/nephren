use crate::commands::Exec;
use crate::context::Context;
use anyhow::{Ok, bail};
use clap::Args;

#[derive(Debug, Args)]
pub struct Activate {
    pub id_or_name: String,
}

impl Exec for Activate {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let mut config = ctx.load_config()?;
        let matched_index = config.find_node(&self.id_or_name);
        if let Some(index) = matched_index {
            config.active_node = Some(config.nodes[index].id);
            config.save()?;

            let node = &config.nodes[index];
            println!("Activated node {} ({})", node.name, node.id);
            Ok(())
        } else {
            bail!("node not found: {}", self.id_or_name);
        }
    }
}
