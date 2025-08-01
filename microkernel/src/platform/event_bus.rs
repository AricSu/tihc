//! The EventBus handles the publishing and subscribing of events between plugins.
//! Plugins can publish events to notify other plugins of state changes or actions.
use std::any::Any;
use std::collections::HashMap;

pub struct EventBus {
    subscribers: HashMap<String, Vec<Box<dyn Fn(&dyn Any)>>>,
}

impl EventBus {
    /// Creates a new EventBus.
    pub fn new() -> Self {
        EventBus {
            subscribers: HashMap::new(),
        }
    }

    /// Subscribes a handler to an event name.
    pub fn subscribe(&mut self, event_name: &str, subscriber: Box<dyn Fn(&dyn Any)>) {
        self.subscribers
            .entry(event_name.to_string())
            .or_insert_with(Vec::new)
            .push(subscriber);
    }

    /// Publishes an event to all subscribers of the event name.
    pub fn publish(&self, event_name: &str, event: &dyn Any) {
        if let Some(subscribers) = self.subscribers.get(event_name) {
            for subscriber in subscribers {
                subscriber(event);
            }
        }
    }
}
