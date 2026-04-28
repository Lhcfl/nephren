use crate::commands::Exec;
use crate::context::Context;
use anyhow::bail;
use clap::Args;
use log::error;

#[derive(Debug, Args)]
pub struct Switch {
    id: String,
}

impl Exec for Switch {
    fn exec(self, ctx: Context) -> anyhow::Result<()> {
        error!("not implemented!");
        panic!("bad implement");
    }
}
