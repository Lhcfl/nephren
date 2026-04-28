use crate::context::Context;
use crate::{commands::Exec, models::subscription::Subscription};
use clap::Args;

#[derive(Debug, Args)]
pub struct Add {
    url: Option<String>,

    #[arg(long)]
    offline: bool,
}

impl Exec for Add {
    fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let mut config = ctx.load_config()?;

        let next_id = config
            .subscriptions
            .iter()
            .map(|x| x.id)
            .max()
            .unwrap_or_default()
            .next();

        config.subscriptions.push(Subscription {
            id: next_id,
            url: self.url,
            description: Default::default(),
            name: format!("sub {}", next_id),
            enable_update: Default::default(),
        });

        config.save()?;

        Ok(())
    }
}
