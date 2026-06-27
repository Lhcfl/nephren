use anyhow::bail;

use crate::models::{
    node::Node,
    subscription::{Subscription, SubscriptionId},
};

pub trait ConfigAction {
    fn find_next_id(&self) -> SubscriptionId;
}

impl ConfigAction for Vec<Subscription> {
    fn find_next_id(&self) -> SubscriptionId {
        self.iter().map(|x| x.id).max().unwrap_or_default().next()
    }
}
