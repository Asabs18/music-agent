# Music Library Agent

[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

An intelligent Rust-based agent system that uses LLMs to analyze, organize, and enhance MP3 metadata in your music library.

## Overview

Music Library Agent demonstrates modern agentic AI patterns in Rust, combining async programming, LLM orchestration, and practical file management to solve real-world music library organization problems. The system uses a trait-based architecture that allows seamless switching between local and cloud-based LLM providers.

## Features

### ✅ Implemented (MVP - v0.1.0)

- **🎵 MP3 Metadata Analysis** - Read and parse ID3 tags from MP3 files
- **🤖 Local LLM Integration** - Ollama support for free, private analysis
- **📊 Intelligent Reporting** - AI-powered assessment of metadata quality
- **🔍 Missing Data Detection** - Automatically identify incomplete or suspicious tags
- **💡 Smart Suggestions** - Get actionable recommendations for metadata improvements
- **⚡ Async Architecture** - Built on Tokio for efficient concurrent operations
- **🎯 Extensible Design** - Trait-based LLM abstraction for easy provider switching
- **🛡️ Robust Error Handling** - Clear, contextual error messages

### 🚧 In Development

- **📝 Metadata Writing** - Apply corrections back to MP3 files (Phase 2)
- **🌐 Cloud LLM Support** - Claude and OpenAI integration (Phase 2)
- **📁 Batch Processing** - Analyze entire directories in parallel (Phase 3)

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

### Usage

Analyze a single MP3 file:

```powershell
cargo run -- path\to\your\song.mp3
```

Or use the release binary:

```powershell
.\target\release\music-agent.exe path\to\your\song.mp3
```

#### Options

```powershell
# Use a different Ollama model
cargo run -- --model mistral path\to\song.mp3

# Connect to remote Ollama instance
cargo run -- --ollama-url http://192.168.1.100:11434 path\to\song.mp3

# Show help
cargo run -- --help
```

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
- Consider adding album artist for compilation compatibility
- Track number would help with playlist ordering

**Confidence**: High
--------------------------------------------------------------

✅ Metadata appears complete
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
- 🚧 **OpenAI** (GPT-4) - Planned for Phase 2

**2. Agent Pattern**

The agent follows a simple but extensible workflow:

```
┌─────────────┐
│  Observe    │  Read metadata from MP3
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Think     │  Send to LLM for analysis
└──────┬──────┘
       │
       ▼
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
- Extensible architecture ready for growth
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
}

struct Agent {
    llm: Box<dyn LLMClient>,
    tools: Vec<Box<dyn Tool>>,
    memory: AgentMemory,
}
```

**Timeline:** 2-3 weeks

---

### 🔮 Phase 3: Scale & External Data (Planned)

**Goal:** Process entire libraries efficiently with authoritative data sources

**Features:**
- 📁 **Batch Processing** - Analyze 100+ files with parallel execution
- 🌐 **MusicBrainz Integration** - Lookup canonical metadata
- 🎵 **Audio Fingerprinting** - Identify tracks by audio content (AcoustID)
- 💾 **Smart Caching** - Cache API responses to avoid re-lookups
- 📊 **Progress Reporting** - Real-time progress bars with `indicatif`
- 🔁 **Resume Capability** - Save state and resume interrupted jobs

**New Modules:**
```
src/
├── integrations/
│   ├── musicbrainz.rs    # MusicBrainz API client
│   ├── acoustid.rs       # Audio fingerprinting
│   └── coverart.rs       # Album art downloads
├── batch/
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