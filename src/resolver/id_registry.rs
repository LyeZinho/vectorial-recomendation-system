//! External ID registry - Tier 1 entity resolution
use std::collections::HashMap;
use uuid::Uuid;

pub struct IdRegistry {
    external_to_internal: HashMap<(String, String), Uuid>,
    internal_to_external: HashMap<Uuid, Vec<(String, String)>>,
}

impl IdRegistry {
    pub fn new() -> Self {
        IdRegistry {
            external_to_internal: HashMap::new(),
            internal_to_external: HashMap::new(),
        }
    }

    pub fn register_id(&mut self, source: &str, external_id: &str, internal_id: Uuid) {
        let key = (source.to_string(), external_id.to_string());
        self.external_to_internal.insert(key.clone(), internal_id);

        self.internal_to_external
            .entry(internal_id)
            .or_insert_with(Vec::new)
            .push((source.to_string(), external_id.to_string()));
    }

    pub fn lookup_external_id(&self, source: &str, external_id: &str) -> Option<Uuid> {
        let key = (source.to_string(), external_id.to_string());
        self.external_to_internal.get(&key).copied()
    }

    pub fn get_external_ids(&self, internal_id: &Uuid) -> Vec<(String, String)> {
        self.internal_to_external
            .get(internal_id)
            .cloned()
            .unwrap_or_default()
    }
}

impl Default for IdRegistry {
    fn default() -> Self {
        Self::new()
    }
}
