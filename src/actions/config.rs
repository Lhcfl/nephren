use anyhow::bail;

use crate::models::{
    node::Node,
    subscription::{Subscription, SubscriptionId},
};

pub trait ConfigAction {
    fn find_next_id(&self) -> SubscriptionId;

    async fn pull(&mut self, id: SubscriptionId) -> anyhow::Result<(Vec<Node>, usize)>;
}

impl ConfigAction for Vec<Subscription> {
    fn find_next_id(&self) -> SubscriptionId {
        self.iter().map(|x| x.id).max().unwrap_or_default().next()
    }

    async fn pull(&mut self, id: SubscriptionId) -> anyhow::Result<(Vec<Node>, usize)> {
        let Some(sub) = self.iter().find(|x| x.id == id) else {
            bail!("No such subscription: {id}");
        };

        sub.pull().await
    }
}
