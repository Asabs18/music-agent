# Music Library Agent

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

An intelligent Rust-based agent system that uses LLMs to analyze, organize, and enhance MP3 metadata in your music library.

## Overview

Music Library Agent demonstrates modern agentic AI patterns in Rust, combining async programming, LLM orchestration, and practical file management to solve real-world music library organization problems. The system uses a trait-based architecture that allows seamless switching between local and cloud-based LLM providers.

## Features

### ✅ Implemented (v0.2.0)

- **🎵 MP3 Metadata Analysis** - Read and parse ID3 tags from MP3 files
- **🤖 Local LLM Integration** - Ollama support for free, private analysis
- **📊 Intelligent Reporting** - AI-powered assessment of metadata quality
- **🔍 Missing Data Detection** - Automatically identify incomplete or suspicious tags
- **💡 Smart Suggestions** - Get actionable recommendations for metadata improvements
- **📝 Safe Metadata Writing** - Apply corrections to NEW files (never overwrites originals)
- **📋 JSON-based Review System** - Review AI suggestions before applying
- **🗂️ Organized Directory Structure** - Separate folders for originals, suggestions, and updated files
- **⚡ Async Architecture** - Built on Tokio for efficient concurrent operations
- **🎯 Extensible Design** - Trait-based LLM abstraction for easy provider switching
- **🛡️ Robust Error Handling** - Clear, contextual error messages

### 🚧 In Development

- **🌐 Cloud LLM Support** - Claude and OpenAI integration (Phase 2)
- **📁 Batch Processing** - Analyze entire directories in parallel (Phase 3)
- **🖥️ GUI Interface** - User-friendly graphical interface

### 🔮 Planned Features

- **🎭 Genre Classification** - Intelligent genre tagging using LLM reasoning
- **🔍 Duplicate Detection** - Find and manage duplicate tracks
- **🎼 MusicBrainz Integration** - Authoritative metadata lookups
- **🎨 Album Art Management** - Download and embed cover art
- **📋 Smart Playlists** - Natural language playlist generation
- **🗂️ Library Organization** - Automated file renaming and folder structure

## Quick Start

### Prerequisites

1. **Rust** (1.70 or later)
   ```powershell
   # Install from https://rustup.rs
   ```

2. **Ollama** (for local LLM)
   ```powershell
   # Download from https://ollama.com/download
   # After installation:
   ollama pull llama3.2
   ```

### Installation

```powershell
# Clone the repository
git clone https://github.com/yourusername/music-agent
cd music-agent

# Build the project
cargo build --release
```

## Usage

The Music Library Agent operates in three modes, using an organized directory structure for safety and clarity.

### Directory Structure

```
public/
├── originals/          # Original MP3 files (never modified)
│   ├── 01 American Pie.mp3
│   ├── 02 Friend of the Devil.mp3
│   └── ...
│
├── suggestions/        # AI-generated suggestions in JSON format
│   ├── 01 American Pie.suggestions.json
│   ├── 02 Friend of the Devil.suggestions.json
│   └── ...
│
└── updated/           # Modified MP3s with applied suggestions
    ├── 02 Friend of the Devil.mp3
    └── ...
```

**Safety Features:**
- ✅ Original files in `originals/` are **never modified**
- ✅ Suggestions saved to JSON for **human review**
- ✅ Updated files created in separate `updated/` directory
- ✅ No overwrites - unique filenames if duplicates exist

---

### Mode 1: Analysis Mode

Get AI-powered analysis of your MP3 metadata without making any changes.

```powershell
cargo run --release -- "public/originals/02 Friend of the Devil.mp3"
```

**What it does:**
- Reads ID3 metadata from the file
- Sends metadata to Ollama for AI analysis
- Displays quality assessment, missing fields, and recommendations
- **No files are modified**

**Example Output:**
```
🎵 Music Library Agent v0.2.0
==============================================================

📖 Reading metadata from: public/originals/02 Friend of the Devil.mp3
🤖 Connecting to Ollama (http://localhost:11434)...
🔍 Analyzing track with Ollama...

==============================================================
📊 ANALYSIS REPORT
==============================================================

🎵 Friend of the Devil
   Artist: Grateful Dead
   Album: American Beauty
   Year: Unknown
   Genre: Unknown

🤖 AI Analysis:
--------------------------------------------------------------
**Assessment**: Medium - Most metadata is complete

**Issues**:
- Year is missing (album released in 1970)
- Genre would help with organization

**Suggestions**:
- Add year: 1970
- Add genre: Folk Rock

**Confidence**: High
--------------------------------------------------------------

💡 Tip: Use --suggestions flag to get structured changes
```

**Options:**
```powershell
# Use a different Ollama model
cargo run --release -- --model mistral "public/originals/song.mp3"

# Connect to remote Ollama instance
cargo run --release -- --ollama-url http://192.168.1.100:11434 "public/originals/song.mp3"
```

---

### Mode 2: Suggestions Mode

Generate a JSON file with structured suggestions that you can review before applying.

```powershell
cargo run --release -- --suggestions "public/originals/02 Friend of the Devil.mp3"
```

**What it does:**
- Analyzes metadata using AI
- Extracts structured suggestions (field-by-field)
- Saves to `public/suggestions/02 Friend of the Devil.suggestions.json`
- **Original file remains untouched**

**Example Output:**
```
🎵 Music Library Agent v0.2.0
==============================================================

📖 Reading metadata from: public/originals/02 Friend of the Devil.mp3
🤖 Connecting to Ollama (http://localhost:11434)...
🔍 Analyzing track with Ollama...

==============================================================
💡 SUGGESTED CHANGES
==============================================================

1. YEAR (Confidence: High)
   Current:  None
   Suggested: 1970
   Reason: American Beauty album was released in 1970

2. GENRE (Confidence: High)
   Current:  None
   Suggested: Folk Rock
   Reason: This song exemplifies the Grateful Dead's folk rock style

--------------------------------------------------------------

💾 Suggestions saved to: public\suggestions\02 Friend of the Devil.suggestions.json

💡 To apply these changes, run:
   cargo run --release -- --apply "public\suggestions\02 Friend of the Devil.suggestions.json"

⚠️  This will create a NEW file (never overwrites original!)
```

**Suggestions File Format:**
```json
{
  "file_path": "public/originals/02 Friend of the Devil.mp3",
  "timestamp": "2026-01-08T15:37:19.517275200-05:00",
  "current_metadata": {
    "artist": "Grateful Dead",
    "title": "Friend of the Devil",
    "album": "American Beauty",
    "year": null,
    "genre": null,
    "track_number": 2
  },
  "suggestions": [
    {
      "field": "year",
      "current_value": null,
      "suggested_value": "1970",
      "confidence": "High",
      "reason": "American Beauty album was released in 1970"
    },
    {
      "field": "genre",
      "current_value": null,
      "suggested_value": "Folk Rock",
      "confidence": "High",
      "reason": "This song exemplifies the Grateful Dead's folk rock style"
    }
  ]
}
```

---

### Mode 3: Apply Mode

Apply suggestions from a JSON file to create an updated MP3 with corrected metadata.

```powershell
cargo run --release -- --apply "public/suggestions/02 Friend of the Devil.suggestions.json"
```

**What it does:**
- Reads the suggestions JSON file
- Applies changes to create updated metadata
- Copies original to `public/updated/02 Friend of the Devil.mp3`
- Writes new metadata to the copy
- **Original file in `originals/` remains completely untouched**

**Example Output:**
```
🎵 Music Library Agent v0.2.0
==============================================================

📂 Loading suggestions from: public/suggestions/02 Friend of the Devil.suggestions.json

📋 Suggestions to apply:
  1. year → 1970
  2. genre → Folk Rock

✍️  Writing updated metadata to NEW file...

✅ SUCCESS!
   Original file: public/originals/02 Friend of the Devil.mp3 (unchanged)
   Updated file:  public\updated\02 Friend of the Devil.mp3

💡 Compare the files and keep the one you prefer!
```

**Verification:**

You can verify the changes were applied by analyzing the updated file:

```powershell
cargo run --release -- "public/updated/02 Friend of the Devil.mp3"
```

---

### Complete Workflow Example

```powershell
# 1. Place your MP3s in public/originals/
Move-Item "C:\Music\*.mp3" "public\originals\"

# 2. Analyze a file (read-only)
cargo run --release -- "public/originals/song.mp3"

# 3. Generate suggestions for review
cargo run --release -- --suggestions "public/originals/song.mp3"

# 4. Review the JSON file manually
code "public\suggestions\song.suggestions.json"

# 5. Apply suggestions to create updated file
cargo run --release -- --apply "public\suggestions\song.suggestions.json"

# 6. Compare original vs updated
cargo run --release -- "public/originals/song.mp3"
cargo run --release -- "public/updated/song.mp3"
```

---

### Command Reference

```powershell
# Show all available options
cargo run --release -- --help

# Analysis mode (read-only)
cargo run --release -- <FILE>

# Suggestions mode (creates JSON)
cargo run --release -- --suggestions <FILE>

# Apply mode (creates updated MP3)
cargo run --release -- --apply <SUGGESTIONS_FILE>

# Custom model
cargo run --release -- --model <MODEL> <FILE>

# Custom Ollama server
cargo run --release -- --ollama-url <URL> <FILE>
```



## Project Architecture

### Current Structure (v0.1.0)

```
music-agent/
├── src/
│   ├── main.rs              # ✅ CLI entry point with clap argument parsing
│   ├── agent.rs             # ✅ Core agent loop: observe → think → report
│   ├── error.rs             # ✅ Custom error types with thiserror
│   ├── llm/
│   │   ├── mod.rs           # ✅ LLM client trait abstraction
│   │   └── ollama.rs        # ✅ Ollama implementation (local, free)
│   └── metadata/
│       ├── mod.rs           # ✅ TrackMetadata struct with validation
│       └── reader.rs        # ✅ ID3 tag reading with error handling
├── target/                  # Build artifacts
├── Cargo.toml              # ✅ Dependencies configured
├── README.md               # ✅ Project documentation
└── QUICKSTART.md           # ✅ Setup and usage guide
```

### Design Principles

**1. Trait-Based Abstraction**
```rust
#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn generate(&self, prompt: &str) -> Result<String>;
    fn provider_name(&self) -> &str;
}
```

The `LLMClient` trait allows seamless switching between:
- ✅ **Ollama** (local, free) - Current implementation
- 🚧 **Claude** (Anthropic) - Planned for Phase 2
- 🚧 **OpenAI** (GPT-4) - 2.0)

```
music-agent/
├── src/
│   ├── main.rs              # ✅ CLI with three modes (analyze, suggestions, apply)
│   ├── agent.rs             # ✅ Core agent: analyze_track, analyze_with_suggestions
│   ├── error.rs             # ✅ Custom error types with thiserror
│   ├── suggestions.rs       # ✅ Suggestions system with JSON serialization
│   ├── llm/
│   │   ├── mod.rs           # ✅ LLM client trait abstraction
│   │   └── ollama.rs        # ✅ Ollama implementation (local, free)
│   └── metadata/
│       ├── mod.rs           # ✅ TrackMetadata struct with validation
│       ├── reader.rs        # ✅ ID3 tag reading with error handling
│       └── writer.rs        # ✅ Safe metadata writing (creates new files)
├── public/
│   ├── originals/           # ✅ Original MP3 files (never modified)
│   ├── suggestions/         # ✅ AI-generated suggestions (JSON)
│   ├── updated/             # ✅ Updated MP3 files with new metadata
│   └── README.md            # ✅ Directory structure documentation
├── target/                  # Build artifacts
├── Cargo.toml              # ✅ Dependencies configured
├── README.md               # ✅ This file - comprehensive
┌─────────────┐
│   Report    │  Display structured results
└─────────────┘
```

This pattern will expand in Phase 2 to include tool selection and action execution.

**3. Error Handling**

Comprehensive error types cover all failure modes:
- `FileRead` - File not found or inaccessible
- `MetadataParse` - Invalid or corrupted ID3 tags
- `LlmRequest` - Network or API failures
- `LlmResponse` - Invalid or unexpected LLM output

Each error includes context for debugging.

## Technology Stack

### Core Dependencies

| Crate | Version | Purpose |
|-------|---------|---------|
| `tokio` | 1.42 | Async runtime for concurrent operations |
| `reqwest` | 0.12 | HTTP client for LLM API calls |
| `id3` | 1.14 | MP3 metadata reading/writing |
| `serde` | 1.0 | Serialization for API requests |
| `anyhow` | 1.0 | Error context and reporting |
| `thiserror` | 2.0 | Custom error type derivation |
| `async-trait` | 0.1 | Async methods in traits |
| `clap` | 4.5 | Command-line argument parsing |

### External Services

- **Ollama** - Local LLM inference (llama3.2, mistral, etc.)
- 🔮 **MusicBrainz** - Canonical music metadata (planned)
- 🔮 **AcoustID** - Audio fingerprinting (planned)


## Development Roadmap

### ✅ Phase 1: MVP (Complete - v0.1.0)

**Goals Achieved:**
- ✅ Single-file analysis workflow
- ✅ Local LLM integration with Ollama
- ✅ ID3 metadata reading
- ✅ Structured agent pattern
- ✅ Robust error handling
- ✅ CLI with helpful options

**Deliverables:**
- Working agent that analyzes individual MP3 files
- Extensible architecture ready for growthand JSON files |
| `serde_json` | 1.0 | JSON serialization for suggestions |
| `chrono` | 0.4 | Timestamps for suggestion files 
- Comprehensive documentation

---

### 🚧 Phase 2: Tools & Actions (In Progress)

**Goal:** Transform from read-only analysis to action-taking agent

**Planned Features:**
- 📝 **MetadataWriter Tool** - Write corrected tags back to MP3 files
- 🤖 **Cloud LLM Support** - Add Claude and OpenAI implementations
- 💬 **Interactive Mode** - User approval for suggested changes
- 🔄 **Undo/Rollback** - Safe operations with backup capability
- 🎯 **Confidence Scoring** - Agent decides when to act vs. ask

**Architecture Changes:**
```rust
#[async_trait]
trait Tool {
    async fn execute(&self, input: ToolInput) -> Result<ToolOutput>;
    fn name(&self) -> &str;
    fn description(&self) -> &str;
---

### ✅ Phase 2: Tools & Actions (Complete - v0.2.0)

**Goals Achieved:**
- ✅ **Safe Metadata Writing** - Write corrected tags to NEW files (never overwrites)
- ✅ **Suggestions System** - JSON-based review workflow
- ✅ **Organized Directory Structure** - Separate folders for originals/suggestions/updated
- ✅ **Three Operating Modes** - Analyze, Generate Suggestions, Apply Changes
- ✅ **File Safety** - Original files protected, unique naming for duplicates

**Key Implementation:**
```rust
// Safe writing - creates new file in updated/ directory
pub fn write_metadata_safely(original_file: &str, metadata: &TrackMetadata) -> Result<String>

// Suggestions saved to JSON for review
pub struct SuggestionsReport {
    pub file_path: String,
    pub suggestions: Vec<MetadataSuggestion>,
    pub llm_analysis: String,
}
```

---

### 🚧 Phase 2.5: Enhanced UX (In Progress)

**Goal:** Improve usability and add graphical interface

**Planned Features:**
- 🖥️ **GUI Interface** - User-friendly graphical interface (egui, iced, or Tauri)
- 🤖 **Cloud LLM Support** - Add Claude and OpenAI implementations
- 💬 **Interactive Mode** - User approval prompts in terminal
- 🔄 **Better Parser** - Improved LLM response parsing for suggestions batch/
│   ├── processor.rs      # Parallel file processing
│   └── progress.rs       # Progress tracking
└── storage/
    └── cache.rs          # API response caching
```

**Timeline:** 2-3 weeks

---

### 🔮 Phase 4: Intelligence Layer (Planned)

**Goal:** Advanced AI-powered features for library organization

**Features:**

**4.1: Genre Classification**
- LLM-based genre analysis with hierarchical taxonomy
- Handles multi-genre artists and edge cases
- User-customizable genre definitions

**4.2: Library Organization**
- Automated file renaming and folder structuring
- Flexible naming templates: `{Artist}/{Album}/{Track} - {Title}.mp3`
- Handles compilations, multi-disc albums, featured artists
- Atomic operations (all-or-nothing moves)

**4.3: Duplicate Detection**
- Multiple detection strategies:
  - Exact match (file hash)
  - Metadata match (artist/title/album)
  - Fuzzy match (similar filenames)
  - Acoustic fingerprinting
- Interactive review interface
- Safe deletion with recovery

**4.4: Smart Playlists & Recommendations**
- Natural language playlist creation
- Similarity-based recommendations
- Mood and context awareness
- Export to M3U/PLS formats

**Timeline:** 3-4 weeks

---

### 🔮 Phase 5: Production Polish (Planned)

**Goal:** Production-quality tool with excellent UX

**Infrastructure:**
- ⚙️ **Configuration System** - TOML config for preferences and API keys
- 📝 **Structured Logging** - Using `tracing` crate
- 📊 **Metrics** - Track success rates and performance
- 🧪 **Comprehensive Testing** - Unit, integration, and property-based tests

**User Experience:**
- 🖥️ **TUI Interface** - Full-screen terminal UI with `ratatui`
- 🌐 **Web Dashboard** - Optional web UI for remote management
- 🔔 **Notifications** - Desktop alerts for long-running jobs
- ↩️ **Full Undo System** - Complete operation history

**Deployment:**
- 📦 Cross-platform binaries (Windows, macOS, Linux)
- 🐳 Docker image for isolated execution
- 🔄 CI/CD pipeline with GitHub Actions

**Timeline:** 2-3 weeks



## License

MIT License - See [LICENSE](LICENSE) file for details

## Acknowledgments

- **Anthropic Claude** - AI assistance and future LLM integration
- **Ollama** - Free local LLM inference
- **MusicBrainz** - Music metadata database (planned integration)
- **Rust Community** - Excellent crates and documentation