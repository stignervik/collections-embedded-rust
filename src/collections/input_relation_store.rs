extern crate alloc;
use crate::core::types::UnitId;
use alloc::vec::Vec;

pub type Key = UnitId;
pub type Value = UnitId;

#[derive(Debug)]
pub struct InputRelation {
    pub key: Key,
    pub value: Value,
}

#[derive(Debug)]
pub struct InputRelationStore {
    pub store: Vec<InputRelation>,
}

impl Default for InputRelationStore {
    fn default() -> Self {
        Self::new()
    }
}

impl InputRelationStore {
    pub fn new() -> Self {
        Self { store: Vec::new() }
    }

    pub fn push(&mut self, key: Key, value: Value) {
        self.store.push(InputRelation { key, value });
        self.store.sort_by(|a, b| a.key.cmp(&b.key));
    }

    pub fn fetch_inputs(&mut self, key: Key) -> Vec<Value> {
        let mut inputs = Vec::new();
        for input_relation in self.store.iter() {
            if input_relation.key == key {
                inputs.push(input_relation.value);
            }
        }
        inputs
    }

    pub fn remove(&mut self, key: Key, value: Value) {
        self.store.retain(|x| x.key != key || x.value != value);
    }
}

#[cfg(test)]
mod tinpuests {
    use super::*;

    #[test]
    fn test_input_relation_store() {
        let mut input_relation_store = InputRelationStore::new();
        assert_eq!(input_relation_store.store.len(), 0);
        input_relation_store.push(1, 1);
        assert_eq!(input_relation_store.store.len(), 1);
        input_relation_store.push(3, 1);
        input_relation_store.push(2, 1);
        input_relation_store.push(2, 2);
        input_relation_store.push(2, 3);

        println!("Store: {:?}", input_relation_store.store);
        println!(
            "Inputs for unit with unit id = 2 {:?}",
            input_relation_store.fetch_inputs(2)
        );
    }
}
