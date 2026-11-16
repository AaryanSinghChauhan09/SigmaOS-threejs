// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-threejs: threejs component model.
//!
//! Real implementation: threejs_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThreejsItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThreejsError {
    EmptyThreejsItemName,
    DuplicateThreejsItem(String),
    EmptyEventSubject,
}

impl fmt::Display for ThreejsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyThreejsItemName => write!(f, "threejs_item name must not be empty"),
            Self::DuplicateThreejsItem(id) => write!(f, "duplicate threejs_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ThreejsError {}

pub struct Threejs {
    pub name: String,
    threejs_items: BTreeMap<String, ThreejsItem>,
    events: BTreeMap<String, Event>,
}

impl Threejs {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            threejs_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_threejs_item(&mut self, item: ThreejsItem) -> Result<(), ThreejsError> {
        if item.name.trim().is_empty() {
            return Err(ThreejsError::EmptyThreejsItemName);
        }
        let id = item.id.clone();
        if self.threejs_items.contains_key(&id) {
            return Err(ThreejsError::DuplicateThreejsItem(id.clone()));
        }
        self.threejs_items.insert(id, item);
        Ok(())
    }

    pub fn threejs_item_count(&self) -> usize {
        self.threejs_items.len()
    }

    pub fn get_threejs_item(&self, id: &str) -> Option<&ThreejsItem> {
        self.threejs_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ThreejsError> {
        if event.subject.trim().is_empty() {
            return Err(ThreejsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_threejs_item(id: &str, name: &str) -> ThreejsItem {
        ThreejsItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Threejs::new("Threejs");
        assert_eq!(m.name, "Threejs");
        assert_eq!(m.threejs_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_threejs_item() {
        let mut m = Threejs::new("Threejs");
        m.add_threejs_item(sample_threejs_item("a1", "Alpha")).unwrap();
        assert_eq!(m.threejs_item_count(), 1);
        assert!(m.get_threejs_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Threejs::new("Threejs");
        assert!(m.add_threejs_item(sample_threejs_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Threejs::new("Threejs");
        m.add_threejs_item(sample_threejs_item("x", "One")).unwrap();
        assert!(m.add_threejs_item(sample_threejs_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Threejs::new("Threejs");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
