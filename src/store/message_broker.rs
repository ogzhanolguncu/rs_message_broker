use std::collections::HashMap;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, RwLock};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Subject(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubscriptionId(pub u16);

#[derive(Debug)]
pub struct MessageBrokerStore {
    data: Arc<RwLock<HashMap<Subject, HashMap<SubscriptionId, TcpStream>>>>,
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
        stream: TcpStream,
    ) -> Result<bool, &'static str> {
        let mut data = self.data.write().unwrap();
        let subscribers = data.entry(subject).or_insert_with(HashMap::new);
        Ok(subscribers
            .insert(sub, stream.try_clone().unwrap())
            .is_none())
    }

    pub fn remove_subscription(&self, sub: SubscriptionId) -> Result<bool, &'static str> {
        let mut data = self.data.write().unwrap();
        let mut removed = false;

        for subscribers in data.values_mut() {
            if subscribers.remove(&sub).is_some() {
                removed = true;
            }
        }

        if removed {
            Ok(true)
        } else {
            Err("Subscription not found")
        }
    }

    pub fn publish_to_sub(&self, subject: Subject, message: String) {
        if let Ok(data) = self.data.read() {
            if let Some(subscribers) = data.get(&subject) {
                for (sid, stream) in subscribers {
                    let formatted_message = format!(
                        "-MSG {} {} {}\r\n{}\r\n",
                        subject.0,
                        sid.0,
                        message.len(),
                        message
                    );
                    let mut stream = stream.try_clone().unwrap();
                    let _ = stream.write_all(formatted_message.as_bytes());
                }
            }
        }
    }
}
