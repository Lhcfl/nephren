use crate::commands::{self, Exec};
use crate::context::Context;
use clap::Args;

#[derive(Debug, Args)]
pub struct Switch {
    id: String,
}

impl Exec for Switch {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        commands::nodes::Activate {
            id_or_name: self.id,
        }
        .exec(ctx)
        .await
    }
}
