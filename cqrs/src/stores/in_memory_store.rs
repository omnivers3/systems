use crate::{ EventStore };

pub struct InMemoryEventStore<T> {
    events: std::cell::RefCell<Vec<T>>,
}

impl<T> InMemoryEventStore<T> {
    #[allow(dead_code)]
    pub fn init() -> Self {
        InMemoryEventStore {
            events: std::cell::RefCell::new(Vec::new()),
        }
    }
}

impl<Event: Copy> EventStore<Event> for InMemoryEventStore<Event> {
    // This should probably return Result with store index data and error
    fn store(&self, event: &Event) -> bool {
        self.events.borrow_mut().push(event.clone());
        true
    }

    fn load(&self, handler: &mut dyn FnMut(&Event)) {
        let events = self.events.borrow();
        for event in events.iter() {
            handler(&event)
        }
    }
}