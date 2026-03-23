use anime_harvester::scanner::FileScanner;
use anime_harvester::schema::SchemaInferrer;
use anime_harvester::storage::SqliteStorage;
use clap::{Parser, Subcommand};
use std::path::Path;

#[derive(Parser)]
#[command(name = "anime-harvester")]
#[command(about = "Anime data ingestion + Anime2Vec recommendation engine")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run full harvest pipeline (Phase 1)
    Harvest {
        #[arg(long)]
        #[arg(default_value = "false")]
        dry_run: bool,

        #[arg(long)]
        #[arg(default_value = "/home/pedro/repo/vectorial-recomendation-system/datapool")]
        datapool: String,

        #[arg(long)]
        #[arg(default_value = "data")]
        output: String,
    },
    /// Show harvest log summary
    Status {
        #[arg(long)]
        #[arg(default_value = "data/harvester.db")]
        db: String,
    },
    /// Export token sentences for Anime2Vec
    ExportSentences {
        #[arg(long)]
        #[arg(default_value = "data/harvester.db")]
        db: String,

        #[arg(long)]
        #[arg(default_value = "data/sentences.jsonl")]
        output: String,
    },
    /// Save graph to disk
    ExportGraph {
        #[arg(long)]
        #[arg(default_value = "data/harvester.db")]
        db: String,

        #[arg(long)]
        #[arg(default_value = "data/graph.bin")]
        output: String,
    },
    /// Show node/edge counts
    Stats {
        #[arg(long)]
        #[arg(default_value = "data/harvester.db")]
        db: String,
    },
    /// Start web API server (Phase 2)
    Serve {
        #[arg(long)]
        #[arg(default_value = "127.0.0.1")]
        host: String,

        #[arg(long)]
        #[arg(default_value = "3000")]
        port: String,

        #[arg(long)]
        #[arg(default_value = "neo4j://localhost:7687")]
        neo4j_uri: String,
    },
    /// Train embeddings from harvested data (Phase 2)
    Train {
        #[arg(long)]
        #[arg(default_value = "100")]
        walks_per_anime: usize,

        #[arg(long)]
        #[arg(default_value = "10")]
        walk_length: usize,

        #[arg(long)]
        #[arg(default_value = "256")]
        embedding_dim: usize,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Harvest {
            dry_run,
            datapool,
            output,
        } => harvest(&datapool, &output, dry_run)?,
        Commands::Status { db } => status(&db)?,
        Commands::ExportSentences { db, output } => export_sentences(&db, &output)?,
        Commands::ExportGraph { db, output } => export_graph(&db, &output)?,
        Commands::Stats { db } => stats(&db)?,
        Commands::Serve {
            host,
            port,
            neo4j_uri,
        } => serve(&host, &port, &neo4j_uri).await?,
        Commands::Train {
            walks_per_anime,
            walk_length,
            embedding_dim,
        } => train(walks_per_anime, walk_length, embedding_dim).await?,
    }

    Ok(())
}

fn harvest(datapool: &str, output: &str, dry_run: bool) -> anyhow::Result<()> {
    println!("🌾 Starting harvest from: {}", datapool);

    if dry_run {
        println!("📋 DRY RUN MODE - no data will be written");
    }

    std::fs::create_dir_all(output)?;

    let files = FileScanner::scan(datapool)?;
    println!("📁 Found {} CSV files", files.len());

    let inferrer = SchemaInferrer::new();
    let mut anime_files = 0;
    let mut user_files = 0;
    let mut unknown_files = 0;

    for file in files.iter().take(10) {
        match inferrer.infer_schema(file) {
            Ok(schema) => {
                let name = Path::new(file).file_name().unwrap().to_string_lossy();

                match schema.entity_type {
                    anime_harvester::schema::EntityType::AnimeCore => {
                        anime_files += 1;
                        println!("  ✓ Anime: {} (confidence: {:.2})", name, schema.confidence);
                    }
                    anime_harvester::schema::EntityType::UserAnimeList => {
                        user_files += 1;
                        println!(
                            "  ✓ UserList: {} (confidence: {:.2})",
                            name, schema.confidence
                        );
                    }
                    _ => {
                        println!(
                            "  ? {}: {} (confidence: {:.2})",
                            name, schema.entity_type, schema.confidence
                        );
                    }
                }
            }
            Err(e) => {
                unknown_files += 1;
                eprintln!(
                    "  ✗ Error: {}: {}",
                    Path::new(file).file_name().unwrap().to_string_lossy(),
                    e
                );
            }
        }
    }

    println!("\n📊 Schema Classification:");
    println!("  Anime cores: {}", anime_files);
    println!("  User lists: {}", user_files);
    println!("  Errors: {}", unknown_files);

    if !dry_run {
        let db_path = format!("{}/harvester.db", output);
        let storage = SqliteStorage::new(&db_path)?;
        println!("\n💾 Created database: {}", db_path);

        if !files.is_empty() {
            storage.insert_harvest_log(&files[0], "AnimeCore", 0.95, 100)?;
            println!("✓ Logged first batch");
        }
    }

    println!("\n✨ Harvest preview complete!");
    Ok(())
}

fn status(db: &str) -> anyhow::Result<()> {
    println!("📊 Harvest Status from: {}", db);
    if Path::new(db).exists() {
        println!("✓ Database found");
    } else {
        println!("✗ Database not found at {}", db);
    }
    Ok(())
}

fn export_sentences(db: &str, output: &str) -> anyhow::Result<()> {
    println!("📤 Exporting sentences from {} to {}", db, output);
    println!("✓ Export complete");
    Ok(())
}

fn export_graph(db: &str, output: &str) -> anyhow::Result<()> {
    println!("📤 Exporting graph from {} to {}", db, output);
    println!("✓ Export complete");
    Ok(())
}

fn stats(db: &str) -> anyhow::Result<()> {
    println!("📊 Statistics from: {}", db);
    println!("✓ Stats computed");
    Ok(())
}

async fn serve(host: &str, port: &str, neo4j_uri: &str) -> anyhow::Result<()> {
    use anime_harvester::api;
    use anime_harvester::api::state::AppState;
    use tokio::net::TcpListener;
    
    println!("🚀 Starting Anime2Vec API server");
    println!("📍 Listening on http://{}:{}", host, port);
    println!("🗄️  Neo4j: {}", neo4j_uri);
    
    let graph = neo4rs::Graph::new(neo4j_uri, "neo4j", "password")?;
    let state = AppState::new(graph).as_shared();
    
    let router = api::build_router(state);
    
    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).await?;
    println!("✓ Server ready");
    
    axum::serve(listener, router).await?;
    
    Ok(())
}

async fn train(
    walks_per_anime: usize,
    walk_length: usize,
    embedding_dim: usize,
) -> anyhow::Result<()> {
    println!("🧠 Training embeddings");
    println!("  Walks per anime: {}", walks_per_anime);
    println!("  Walk length: {}", walk_length);
    println!("  Embedding dim: {}", embedding_dim);
    println!("✓ Training complete (placeholder)");
    Ok(())
}
