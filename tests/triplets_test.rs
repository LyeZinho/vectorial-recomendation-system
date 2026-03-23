use anime_harvester::triplets::{RelationType, Triplet};
use uuid::Uuid;

#[test]
fn test_relation_type_display() {
    assert_eq!(RelationType::ProducedBy.to_string(), "ProducedBy");
    assert_eq!(RelationType::HasGenre.to_string(), "HasGenre");
    assert_eq!(RelationType::FriendsWith.to_string(), "FriendsWith");
}

#[test]
fn test_triplet_creation() {
    let subject = Uuid::new_v4();
    let object_str = "Action".to_string();

    let triplet = Triplet {
        subject,
        predicate: RelationType::HasGenre,
        object: object_str,
    };

    assert_eq!(triplet.subject, subject);
    assert_eq!(triplet.predicate, RelationType::HasGenre);
    assert_eq!(triplet.object, "Action");
}
