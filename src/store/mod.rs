use std::collections::{HashMap, HashSet};
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subject(String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubscriptionId(String);

#[derive(Debug, Clone)]
pub struct MessageBrokerStore {
    data: Arc<RwLock<HashMap<Subject, HashSet<SubscriptionId>>>>,
}

impl MessageBrokerStore {
    pub fn new() -> Self {
        MessageBrokerStore {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn add_subscription(
        &self,
        subject: Subject,
        sub: SubscriptionId,
    ) -> Result<bool, &'static str> {
        self.data
            .write()
            .map_err(|_| "Could not acquire data write lock")
            .and_then(|mut data| Ok(data.entry(subject).or_insert_with(HashSet::new).insert(sub)))
    }

    pub fn remove_subscription(
        &self,
        subject: &Subject,
        sub: &SubscriptionId,
    ) -> Result<bool, &'static str> {
        self.data
            .write()
            .map_err(|_| "Could not acquire data write lock")
            .and_then(|mut data| {
                data.get_mut(subject)
                    .ok_or("Could not get subject")
                    .map(|subs| subs.remove(sub))
            })
    }

    pub fn list_subscriptions(
        &self,
        subject: &Subject,
    ) -> Result<Vec<SubscriptionId>, &'static str> {
        match self.data.read() {
            Ok(data) => Ok(data
                .get(subject)
                .cloned()
                .map(|hs| hs.into_iter().collect())
                .unwrap_or_else(Vec::new)),
            Err(_) => Err("Could not acquire data read lock"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_add_subscriber_to_cache() {
        let cache = MessageBrokerStore::new();
        cache
            .add_subscription(Subject("FOO".to_string()), SubscriptionId("10".to_string()))
            .unwrap();
        cache
            .add_subscription(Subject("FOO".to_string()), SubscriptionId("10".to_string()))
            .unwrap();
        cache
            .add_subscription(Subject("FOO".to_string()), SubscriptionId("10".to_string()))
            .unwrap();

        assert_eq!(
            1,
            cache
                .list_subscriptions(&Subject("FOO".to_string()))
                .unwrap()
                .len()
        )
    }

    #[test]
    fn should_remove_subscriber_from_cache() {
        let cache = MessageBrokerStore::new();
        cache
            .add_subscription(Subject("FOO".to_string()), SubscriptionId("10".to_string()))
            .unwrap();
        cache
            .add_subscription(Subject("FOO".to_string()), SubscriptionId("11".to_string()))
            .unwrap();
        cache
            .remove_subscription(
                &Subject("FOO".to_string()),
                &SubscriptionId("10".to_string()),
            )
            .unwrap();

        assert_eq!(
            1,
            cache
                .list_subscriptions(&Subject("FOO".to_string()))
                .unwrap()
                .len()
        )
    }
}

// let mut map: HashMap<&str, String> = HashMap::new();
// let s = "hoho".to_string();

// map.entry("poneyland").or_insert_with(|| s);

// assert_eq!(map["poneyland"], "hoho".to_string());
