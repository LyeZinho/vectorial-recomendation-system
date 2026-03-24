use anime_harvester::ml::SkipGramTrainer;

#[test]
fn test_skip_gram_training_output_shape() {
    let walks = vec![vec!["a".to_string(), "b".to_string(), "c".to_string()]];

    let trainer = SkipGramTrainer::new(10, 0.025, 2, 1);
    let embeddings = trainer.train_mock(walks).unwrap();

    assert_eq!(embeddings.len(), 3); // 3 unique tokens
    for (_, vec) in embeddings {
        assert_eq!(vec.len(), 10); // embedding_dim = 10
    }
}

#[test]
fn test_skip_gram_embedding_initialization() {
    let trainer = SkipGramTrainer::new(128, 0.025, 5, 5);
    assert_eq!(trainer.embedding_dim, 128);
    assert_eq!(trainer.window_size, 5);
}
