use anime_harvester::resolver::IdRegistry;
use uuid::Uuid;

#[test]
fn test_id_registry_exact_match() {
    let mut registry = IdRegistry::new();
    let internal_id = Uuid::new_v4();

    registry.register_id("mal_id", "5114", internal_id);
    let found = registry.lookup_external_id("mal_id", "5114");
    assert_eq!(found, Some(internal_id));
}

#[test]
fn test_id_registry_merges_multiple_sources() {
    let mut registry = IdRegistry::new();
    let internal_id = Uuid::new_v4();

    registry.register_id("mal_id", "5114", internal_id);
    registry.register_id("anidb_id", "2763", internal_id);

    assert_eq!(
        registry.lookup_external_id("mal_id", "5114"),
        Some(internal_id)
    );
    assert_eq!(
        registry.lookup_external_id("anidb_id", "2763"),
        Some(internal_id)
    );
}

#[test]
fn test_id_registry_returns_none_for_unknown() {
    let registry = IdRegistry::new();
    assert_eq!(registry.lookup_external_id("unknown_source", "999"), None);
}
