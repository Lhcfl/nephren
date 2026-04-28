use crate::commands::Exec;
use crate::context::Context;
use anyhow::bail;
use clap::Args;

#[derive(Debug, Args)]
pub struct Remove {
    id_or_name: String,
}

impl Exec for Remove {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let mut config = ctx.load_config()?;

        let matched_index = config.find_subscription(&self.id_or_name);

        if let Some(index) = matched_index {
            let matched = config.subscriptions.remove(index);
            config.save()?;
            println!("successfully removed subscription: {matched:#?}");
        } else {
            bail!("subscription not found: {}", self.id_or_name);
        }

        Ok(())
    }
}
