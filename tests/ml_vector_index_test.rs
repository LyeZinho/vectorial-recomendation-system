use anime_harvester::ml::HNSWIndex;

#[test]
fn test_hnsw_insert_and_search() {
    let mut index = HNSWIndex::new(10, 32);

    let vec1 = vec![1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let vec2 = vec![1.0, 0.1, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let vec3 = vec![0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0];

    index.insert("anime_1".to_string(), vec1.clone()).unwrap();
    index.insert("anime_2".to_string(), vec2.clone()).unwrap();
    index.insert("anime_3".to_string(), vec3.clone()).unwrap();

    let results = index.search(&vec1, 2).unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].0, "anime_1");
}

#[test]
fn test_hnsw_cosine_similarity() {
    let vec_a = vec![1.0, 0.0, 0.0];
    let vec_b = vec![1.0, 0.0, 0.0];
    let vec_c = vec![0.0, 1.0, 0.0];

    let sim_identical = HNSWIndex::cosine_similarity(&vec_a, &vec_b);
    let sim_orthogonal = HNSWIndex::cosine_similarity(&vec_a, &vec_c);

    assert!((sim_identical - 1.0).abs() < 0.01);
    assert!(sim_orthogonal < 0.01);
}

#[test]
fn test_hnsw_insert_validates_dimension() {
    let mut index = HNSWIndex::new(5, 32);
    let wrong_dim_vec = vec![1.0, 2.0, 3.0];

    let result = index.insert("bad".to_string(), wrong_dim_vec);
    assert!(result.is_err());
}

#[test]
fn test_hnsw_search_validates_query_dimension() {
    let mut index = HNSWIndex::new(5, 32);
    index.insert("test".to_string(), vec![1.0; 5]).unwrap();

    let wrong_query = vec![1.0, 2.0];
    let result = index.search(&wrong_query, 1);
    assert!(result.is_err());
}

#[test]
fn test_hnsw_methods() {
    let mut index = HNSWIndex::new(3, 32);
    assert!(index.is_empty());
    assert_eq!(index.len(), 0);

    index.insert("a".to_string(), vec![1.0, 0.0, 0.0]).unwrap();
    assert!(!index.is_empty());
    assert_eq!(index.len(), 1);
    assert!(index.get("a").is_some());
    assert!(index.get("b").is_none());
}
