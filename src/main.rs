mod agent;
mod error;
mod llm;
mod metadata;

use agent::MusicAgent;
use clap::Parser;
use error::Result;
use metadata::reader;

#[derive(Parser, Debug)]
#[command(name = "music-agent")]
#[command(about = "AI-powered music metadata analyzer", long_about = None)]
struct Args {
    /// Path to the MP3 file to analyze
    #[arg(value_name = "FILE")]
    file: String,

    /// LLM model to use (default: llama3.2)
    #[arg(short, long, default_value = "llama3.2")]
    model: String,

    /// Ollama server URL (default: http://localhost:11434)
    #[arg(short, long, default_value = "http://localhost:11434")]
    ollama_url: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("🎵 Music Library Agent v0.1.0");
    println!("{}\n", "=".repeat(62));

    // Step 1: Read metadata from file
    println!("📖 Reading metadata from: {}", args.file);
    let metadata = reader::read_metadata(&args.file)?;

    // Step 2: Create LLM client
    println!("🤖 Connecting to Ollama ({})...", args.ollama_url);
    let llm_client = llm::ollama::OllamaClient::new(&args.ollama_url)
        .with_model(&args.model);

    // Step 3: Create agent and analyze
    let agent = MusicAgent::new(Box::new(llm_client));
    let report = agent.analyze_track(&metadata).await?;

    // Step 4: Display results
    report.display();

    Ok(())
}

