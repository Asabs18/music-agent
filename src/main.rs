mod agent;
mod error;
mod llm;
mod metadata;
mod suggestions;

use agent::MusicAgent;
use clap::Parser;
use error::Result;
use metadata::{reader, writer};
use suggestions::SuggestionsReport;

#[derive(Parser, Debug)]
#[command(name = "music-agent")]
#[command(about = "AI-powered music metadata analyzer", long_about = None)]
struct Args {
    /// Path to the MP3 file to analyze
    #[arg(value_name = "FILE", required_unless_present = "apply")]
    file: Option<String>,

    /// LLM model to use (default: llama3.2)
    #[arg(short, long, default_value = "llama3.2")]
    model: String,

    /// Ollama server URL (default: http://localhost:11434)
    #[arg(short, long, default_value = "http://localhost:11434")]
    ollama_url: String,

    /// Generate suggestions file instead of just analysis
    #[arg(short, long)]
    suggestions: bool,

    /// Apply suggestions from a .suggestions.json file (creates new .updated.mp3)
    #[arg(short, long, value_name = "SUGGESTIONS_FILE")]
    apply: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    println!("🎵 Music Library Agent v0.2.0");
    println!("{}\n", "=".repeat(62));

    // Mode 1: Apply suggestions from JSON file
    if let Some(suggestions_file) = args.apply {
        return apply_suggestions_mode(&suggestions_file);
    }

    // Step 1: Read metadata from file
    let file_path = args.file.expect("FILE is required for analysis mode");
    println!("📖 Reading metadata from: {}", file_path);
    let metadata = reader::read_metadata(&file_path)?;

    // Step 2: Create LLM client
    println!("🤖 Connecting to Ollama ({})...", args.ollama_url);
    let llm_client = llm::ollama::OllamaClient::new(&args.ollama_url).with_model(&args.model);

    // Step 3: Create agent and analyze
    let agent = MusicAgent::new(Box::new(llm_client));

    // Mode 2: Generate suggestions
    if args.suggestions {
        let suggestions_report = agent.analyze_with_suggestions(&metadata).await?;
        suggestions_report.display();

        let file_path = suggestions_report.save_to_file()?;
        println!("\n💾 Suggestions saved to: {}", file_path);
        println!("\n💡 To apply these changes, run:");
        println!("   cargo run --release -- --apply \"{}\"", file_path);
        println!("\n⚠️  This will create a NEW file (never overwrites original!)");
        return Ok(());
    }

    // Mode 3: Original analysis mode
    let report = agent.analyze_track(&metadata).await?;
    report.display();

    println!("\n💡 Tip: Use --suggestions flag to get structured changes");

    Ok(())
}

/// Apply suggestions from a JSON file to create updated MP3
fn apply_suggestions_mode(suggestions_file: &str) -> Result<()> {
    println!("📂 Loading suggestions from: {}", suggestions_file);
    let suggestions = SuggestionsReport::load_from_file(suggestions_file)?;

    println!("\n📋 Suggestions to apply:");
    for (i, suggestion) in suggestions.suggestions.iter().enumerate() {
        println!(
            "  {}. {} → {}",
            i + 1,
            suggestion.field,
            suggestion.suggested_value
        );
    }

    if suggestions.suggestions.is_empty() {
        println!("\n✅ No suggestions to apply!");
        return Ok(());
    }

    // Apply suggestions to create updated metadata
    let updated_metadata = suggestions.apply_suggestions();

    // Write to NEW file (never overwrites original)
    println!("\n✍️  Writing updated metadata to NEW file...");
    let output_file = writer::write_metadata_safely(&suggestions.file_path, &updated_metadata)?;

    println!("\n✅ SUCCESS!");
    println!("   Original file: {} (unchanged)", suggestions.file_path);
    println!("   Updated file:  {}", output_file);
    println!("\n💡 Compare the files and keep the one you prefer!");

    Ok(())
}
