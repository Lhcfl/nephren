use crate::actions::config::ConfigAction;
use crate::context::Context;
use crate::{commands::Exec, models::subscription::Subscription};
use clap::Args;
use log::info;
use url::Url;

#[derive(Debug, Args)]
pub struct Add {
    /// 订阅 URL 或别名。自动推断是否为 URL
    name_or_url: String,

    /// 不自动爬取该 URL 的内容。
    #[arg(long)]
    offline: bool,
}

impl Exec for Add {
    async fn exec(self, ctx: Context) -> anyhow::Result<()> {
        let mut state = ctx.load_state()?;

        let next_id = state.subscriptions.find_next_id();

        let new_sub = match Url::parse(&self.name_or_url) {
            Ok(url) => {
                info!("successfully parsed {} as a url. ", url);

                Subscription {
                    id: next_id,
                    url: Some(self.name_or_url),
                    description: Default::default(),
                    name: format!("sub {}", next_id),
                    enable_update: Default::default(),
                }
            }
            Err(e) => {
                info!("{} seems like not a url: {e}", self.name_or_url);

                Subscription {
                    id: next_id,
                    url: None,
                    description: Default::default(),
                    name: self.name_or_url,
                    enable_update: Default::default(),
                }
            }
        };

        let sub_id = new_sub.id;
        state.subscriptions.push(new_sub);
        state.save()?;

        info!("saved successfully.");

        let (nodes, faileds) = state
            .subscriptions
            .iter()
            .find(|it| it.id == sub_id)
            .unwrap()
            .pull()
            .await?;

        state.replace_subscription_nodes(sub_id, nodes);
        state.save()?;

        if faileds > 0 {
            log::warn!(
                "{} nodes failed to parse, but the rest have been added",
                faileds
            );
        }

        Ok(())
    }
}
