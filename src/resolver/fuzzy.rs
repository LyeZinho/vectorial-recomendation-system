//! Tier 3 - Fuzzy name matching with Jaro-Winkler
use strsim::jaro_winkler;

pub struct FuzzyMatcher {
    threshold: f64,
}

impl FuzzyMatcher {
    pub fn new(threshold: f64) -> Self {
        FuzzyMatcher { threshold }
    }

    pub fn match_names(&self, name1: &str, name2: &str) -> bool {
        jaro_winkler(name1, name2) >= self.threshold
    }
}

impl Default for FuzzyMatcher {
    fn default() -> Self {
        FuzzyMatcher { threshold: 0.85 }
    }
}
