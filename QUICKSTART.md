# Music Agent - Quick Start Guide

## Prerequisites

### 1. Install Ollama

Download and install Ollama from: https://ollama.com/download

For Windows, download the installer and run it.

### 2. Pull the Default Model

After installing Ollama, open a terminal and run:

```powershell
ollama pull llama3.2
```

This downloads the default model (about 2GB). The download happens once and then it's available locally.

### 3. Verify Ollama is Running

Ollama should start automatically. To verify:

```powershell
ollama list
```

You should see `llama3.2` in the list.

## Building the Project

```powershell
cargo build --release
```

## Running the Agent

Analyze a single MP3 file:

```powershell
cargo run -- path\to\your\song.mp3
```

Or with the release build:

```powershell
.\target\release\music-agent.exe path\to\your\song.mp3
```

### Advanced Options

Use a different model (e.g., mistral):

```powershell
# First pull the model
ollama pull mistral

# Then use it
cargo run -- --model mistral path\to\song.mp3
```

Use a custom Ollama server:

```powershell
cargo run -- --ollama-url http://192.168.1.100:11434 path\to\song.mp3
```

See all options:

```powershell
cargo run -- --help
```

## What the Agent Does

The MVP agent:
1. ✅ Reads ID3 metadata from your MP3 file
2. ✅ Sends it to the local LLM (Ollama) for analysis
3. ✅ Provides insights about data quality, missing fields, and suggestions
4. ✅ Displays a structured report

## Example Output

```
🎵 Music Library Agent v0.1.0
==============================================================

📖 Reading metadata from: C:\Music\song.mp3
🤖 Connecting to Ollama (http://localhost:11434)...
🔍 Analyzing track with Ollama...

==============================================================
📊 ANALYSIS REPORT
==============================================================

🎵 Bohemian Rhapsody
   Artist: Queen
   Album: A Night at the Opera
   Year: 1975
   Genre: Rock

🤖 AI Analysis:
--------------------------------------------------------------
**Assessment**: The metadata is well-structured and complete.

**Issues**: None detected. All critical fields are present.

**Suggestions**: 
- Consider adding album artist if this is a compilation
- Track number would help with playlist ordering

**Confidence**: High - This appears to be accurate metadata
--------------------------------------------------------------

✅ Metadata appears complete
```

## Troubleshooting

### "Failed to connect to Ollama"

Make sure Ollama is running:
```powershell
ollama serve
```

### "Model not found"

Pull the model first:
```powershell
ollama pull llama3.2
```

### Slow analysis

First run might be slow as the model loads into memory. Subsequent runs are faster. You can also try a smaller model:
```powershell
ollama pull llama3.2:1b  # Smaller, faster version
cargo run -- --model llama3.2:1b path\to\song.mp3
```

## Next Steps (Future Phases)

The current MVP is read-only. Future phases will add:
- 🔧 Metadata correction (writing back to files)
- 📁 Batch processing (analyze entire directories)
- 🌐 MusicBrainz integration (authoritative metadata lookups)
- 🎭 Genre classification
- 🔍 Duplicate detection
- 📋 Playlist generation

## Switching to Cloud LLMs (Later)

The architecture is ready for cloud APIs. To add Claude or OpenAI:

1. Set environment variable:
```powershell
$env:ANTHROPIC_API_KEY = "your-key-here"
# or
$env:OPENAI_API_KEY = "your-key-here"
```

2. Update `src/llm/mod.rs` to check for keys and create appropriate clients
3. Implement `src/llm/claude.rs` or `src/llm/openai.rs` following the `LLMClient` trait

The agent code doesn't need to change - it just uses the `LLMClient` trait!
