extern crate alloc;
use crate::core::types::UnitId;
use alloc::vec::Vec;

#[derive(Debug)]
pub struct UnitIdStore {
    pub unit_ids: Vec<UnitId>,
}

impl Default for UnitIdStore {
    fn default() -> Self {
        Self::new()
    }
}

impl UnitIdStore {
    pub fn new() -> Self {
        Self {
            unit_ids: Vec::new(),
        }
    }

    pub fn add(&mut self, unit_id: UnitId) {
        self.unit_ids.push(unit_id);
    }

    pub fn remove(&mut self, unit_id: UnitId) {
        self.unit_ids.retain(|&x| x != unit_id);
    }

    pub fn contains(&self, unit_id: UnitId) -> bool {
        self.unit_ids.contains(&unit_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unit_id_store() {
        let mut unit_id_store = UnitIdStore::new();
        assert_eq!(unit_id_store.unit_ids.len(), 0);
        unit_id_store.add(1);
        assert_eq!(unit_id_store.unit_ids.len(), 1);
        assert_eq!(unit_id_store.unit_ids[0], 1);
        unit_id_store.add(2);
        assert_eq!(unit_id_store.unit_ids.len(), 2);
        assert_eq!(unit_id_store.unit_ids[0], 1);
        assert_eq!(unit_id_store.unit_ids[1], 2);
        unit_id_store.remove(1);
        assert_eq!(unit_id_store.unit_ids.len(), 1);
        assert_eq!(unit_id_store.unit_ids[0], 2);
        unit_id_store.remove(2);
        assert_eq!(unit_id_store.unit_ids.len(), 0);
    }
}
