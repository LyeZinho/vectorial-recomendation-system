//! CLI entry point
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "anime-harvester")]
#[command(about = "Autonomous anime data ingestion pipeline", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run full harvest pipeline
    Harvest {
        #[arg(long)]
        #[arg(default_value = "false")]
        dry_run: bool,
    },
    /// Show harvest log summary
    Status,
    /// Export token sentences for Anime2Vec
    ExportSentences,
    /// Save graph to disk
    ExportGraph,
    /// Show node/edge counts
    Stats,
}

fn main() {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    match cli.command {
        Commands::Harvest { dry_run } => {
            println!("Harvest (dry_run={})", dry_run);
        }
        Commands::Status => {
            println!("Status");
        }
        Commands::ExportSentences => {
            println!("Export sentences");
        }
        Commands::ExportGraph => {
            println!("Export graph");
        }
        Commands::Stats => {
            println!("Stats");
        }
    }
}
