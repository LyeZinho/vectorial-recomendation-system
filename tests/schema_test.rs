use anime_harvester::schema::synonyms::SynonymDict;

#[test]
fn test_synonym_dict_recognizes_anime_core_columns() {
    let dict = SynonymDict::new();

    assert_eq!(
        dict.classify_column("title"),
        Some("anime_title".to_string())
    );
    assert_eq!(
        dict.classify_column("Title"),
        Some("anime_title".to_string())
    );
    assert_eq!(
        dict.classify_column("Anime Title"),
        Some("anime_title".to_string())
    );

    assert_eq!(
        dict.classify_column("score"),
        Some("anime_score".to_string())
    );
    assert_eq!(
        dict.classify_column("rating"),
        Some("anime_score".to_string())
    );
    assert_eq!(
        dict.classify_column("mean_score"),
        Some("anime_score".to_string())
    );
}

#[test]
fn test_synonym_dict_recognizes_person_columns() {
    let dict = SynonymDict::new();

    assert_eq!(
        dict.classify_column("name"),
        Some("person_name".to_string())
    );
    assert_eq!(
        dict.classify_column("given_name"),
        Some("person_given_name".to_string())
    );
    assert_eq!(
        dict.classify_column("family_name"),
        Some("person_family_name".to_string())
    );
    assert_eq!(
        dict.classify_column("birthday"),
        Some("person_birthday".to_string())
    );
}

#[test]
fn test_synonym_dict_returns_none_for_unknown() {
    let dict = SynonymDict::new();
    assert_eq!(dict.classify_column("xyz_unknown_field"), None);
}
