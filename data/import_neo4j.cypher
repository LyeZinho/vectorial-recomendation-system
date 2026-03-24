// Clear existing data
MATCH (n) DETACH DELETE n;

// Create Anime nodes with properties
CREATE (:Anime {id: 1, name: "Attack on Titan", genre: "Action,Dark", rating: 4.5, year: 2013})
CREATE (:Anime {id: 2, name: "Death Note", genre: "Thriller,Supernatural", rating: 4.7, year: 2006})
CREATE (:Anime {id: 3, name: "Demon Slayer", genre: "Action,Adventure", rating: 4.6, year: 2019})
CREATE (:Anime {id: 4, name: "Jujutsu Kaisen", genre: "Action,Supernatural", rating: 4.5, year: 2020})
CREATE (:Anime {id: 5, name: "Naruto", genre: "Action,Adventure", rating: 4.4, year: 2002})
CREATE (:Anime {id: 6, name: "One Punch Man", genre: "Action,Comedy", rating: 4.3, year: 2015})
CREATE (:Anime {id: 7, name: "Mob Psycho 100", genre: "Action,Comedy", rating: 4.4, year: 2016})
CREATE (:Anime {id: 8, name: "Steins;Gate", genre: "Sci-Fi,Thriller", rating: 4.8, year: 2011})
CREATE (:Anime {id: 9, name: "Evangelion", genre: "Sci-Fi,Drama", rating: 4.6, year: 1995})
CREATE (:Anime {id: 10, name: "Cowboy Bebop", genre: "Sci-Fi,Action", rating: 4.7, year: 1998});

// Create relationships
MATCH (a:Anime {id: 1}), (b:Anime {id: 3})
CREATE (a)-[:RELATED_TO {weight: 0.85}]->(b);

MATCH (a:Anime {id: 2}), (b:Anime {id: 8})
CREATE (a)-[:RELATED_TO {weight: 0.80}]->(b);

MATCH (a:Anime {id: 3}), (b:Anime {id: 4})
CREATE (a)-[:RELATED_TO {weight: 0.82}]->(b);

MATCH (a:Anime {id: 1}), (b:Anime {id: 5})
CREATE (a)-[:SIMILAR {weight: 0.75}]->(b);

MATCH (a:Anime {id: 6}), (b:Anime {id: 7})
CREATE (a)-[:SIMILAR {weight: 0.78}]->(b);

MATCH (a:Anime {id: 5}), (b:Anime {id: 7})
CREATE (a)-[:INFLUENCED_BY {weight: 0.70}]->(b);

MATCH (a:Anime {id: 8}), (b:Anime {id: 9})
CREATE (a)-[:SIMILAR {weight: 0.72}]->(b);

MATCH (a:Anime {id: 10}), (b:Anime {id: 1})
CREATE (a)-[:INFLUENCED_BY {weight: 0.68}]->(b);

// Create indices for fast queries
CREATE INDEX FOR (a:Anime) ON (a.id);
CREATE INDEX FOR (a:Anime) ON (a.name);
CREATE INDEX FOR (a:Anime) ON (a.genre);

// Verify import
MATCH (n:Anime) RETURN count(n) as anime_count;
MATCH ()-[r]->() RETURN count(r) as relationship_count;
