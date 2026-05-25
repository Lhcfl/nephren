use clap::Subcommand;
use enum_dispatch::enum_dispatch;

use crate::context::Context;

mod nodes;
mod subscriptions;
mod switch;

pub use nodes::Nodes;
pub use subscriptions::Subscription;
pub use switch::Switch;

#[enum_dispatch]
pub trait Exec {
    /// execute the command
    async fn exec(self, ctx: Context) -> anyhow::Result<()>;
}

#[derive(Subcommand, Debug)]
#[enum_dispatch(Exec)]
pub enum Command {
    /// Switch configs
    Switch(Switch),

    /// Manage subscriptions
    #[command(visible_aliases(["sub", "s"]))]
    Subscriptions(Subscription),

    /// Manage nodes
    #[command(visible_aliases(["n"]))]
    Nodes(Nodes),
}
