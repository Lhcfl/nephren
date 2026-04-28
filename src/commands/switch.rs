use crate::commands::Exec;
use crate::context::Context;
use clap::Args;
use log::error;

#[derive(Debug, Args)]
pub struct Switch {
    id: String,
}

impl Exec for Switch {
    async fn exec(self, _ctx: Context) -> anyhow::Result<()> {
        error!("not implemented!");
        panic!("bad implement");
    }
}
