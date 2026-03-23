//! In-memory knowledge graph storage
use std::collections::HashMap;
use uuid::Uuid;

pub struct KnowledgeGraph {
    forward_edges: HashMap<Uuid, Vec<(Uuid, String, f64)>>,
    reverse_index: HashMap<Uuid, Vec<(Uuid, String)>>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        KnowledgeGraph {
            forward_edges: HashMap::new(),
            reverse_index: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, from: Uuid, to: Uuid, relation: String, weight: f64) {
        self.forward_edges
            .entry(from)
            .or_insert_with(Vec::new)
            .push((to, relation.clone(), weight));

        self.reverse_index
            .entry(to)
            .or_insert_with(Vec::new)
            .push((from, relation));
    }
}

impl Default for KnowledgeGraph {
    fn default() -> Self {
        Self::new()
    }
}
