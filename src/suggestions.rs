use crate::error::{AgentError, Result};
use crate::metadata::TrackMetadata;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a suggested metadata change
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetadataSuggestion {
    pub field: String,
    pub current_value: Option<String>,
    pub suggested_value: String,
    pub confidence: String, // "High", "Medium", "Low"
    pub reason: String,
}

/// Collection of suggestions for a track
#[derive(Debug, Serialize, Deserialize)]
pub struct SuggestionsReport {
    pub file_path: String,
    pub timestamp: String,
    pub current_metadata: TrackMetadata,
    pub suggestions: Vec<MetadataSuggestion>,
    pub llm_analysis: String,
    pub should_apply: bool, // Whether user should apply changes
}

impl SuggestionsReport {
    pub fn new(
        file_path: String,
        current_metadata: TrackMetadata,
        suggestions: Vec<MetadataSuggestion>,
        llm_analysis: String,
    ) -> Self {
        Self {
            file_path: file_path.clone(),
            timestamp: chrono::Local::now().to_rfc3339(),
            current_metadata,
            suggestions,
            llm_analysis,
            should_apply: false,
        }
    }

    /// Save suggestions to a JSON file in public/suggestions/ directory
    pub fn save_to_file(&self) -> Result<String> {
        let path = Path::new(&self.file_path);
        let file_stem = path.file_stem().unwrap_or_default();

        // Determine the public/suggestions directory
        let suggestions_dir = if let Some(parent) = path.parent() {
            // If file is in public/originals/, use public/suggestions/
            if parent.ends_with("originals") {
                parent.parent().unwrap_or(parent).join("suggestions")
            } else if parent.ends_with("public") {
                parent.join("suggestions")
            } else {
                // For other locations, create suggestions/ subdirectory
                parent.join("suggestions")
            }
        } else {
            PathBuf::from("public/suggestions")
        };

        // Ensure the directory exists
        fs::create_dir_all(&suggestions_dir).map_err(|e| {
            AgentError::FileRead(format!("Failed to create suggestions directory: {}", e))
        })?;

        let suggestions_path =
            suggestions_dir.join(format!("{}.suggestions.json", file_stem.to_string_lossy()));

        let json = serde_json::to_string_pretty(self).map_err(|e| {
            AgentError::MetadataParse(format!("Failed to serialize suggestions: {}", e))
        })?;

        fs::write(&suggestions_path, json).map_err(|e| {
            AgentError::FileRead(format!("Failed to write suggestions file: {}", e))
        })?;

        Ok(suggestions_path.to_string_lossy().to_string())
    }

    /// Load suggestions from a JSON file
    pub fn load_from_file(file_path: &str) -> Result<Self> {
        let json = fs::read_to_string(file_path)
            .map_err(|e| AgentError::FileRead(format!("Failed to read suggestions file: {}", e)))?;

        serde_json::from_str(&json)
            .map_err(|e| AgentError::MetadataParse(format!("Failed to parse suggestions: {}", e)))
    }

    /// Apply suggestions to create updated metadata
    pub fn apply_suggestions(&self) -> TrackMetadata {
        let mut updated = self.current_metadata.clone();

        for suggestion in &self.suggestions {
            match suggestion.field.as_str() {
                "artist" => updated.artist = Some(suggestion.suggested_value.clone()),
                "title" => updated.title = Some(suggestion.suggested_value.clone()),
                "album" => updated.album = Some(suggestion.suggested_value.clone()),
                "year" => {
                    if let Ok(year) = suggestion.suggested_value.parse::<i32>() {
                        updated.year = Some(year);
                    }
                }
                "genre" => updated.genre = Some(suggestion.suggested_value.clone()),
                "album_artist" => updated.album_artist = Some(suggestion.suggested_value.clone()),
                "track_number" => {
                    if let Ok(track) = suggestion.suggested_value.parse::<u32>() {
                        updated.track_number = Some(track);
                    }
                }
                _ => {}
            }
        }

        updated
    }

    /// Display suggestions in a user-friendly format
    pub fn display(&self) {
        println!("\n{}", "=".repeat(62));
        println!("💡 SUGGESTED CHANGES");
        println!("{}", "=".repeat(62));

        if self.suggestions.is_empty() {
            println!("✅ No changes suggested - metadata looks good!");
            return;
        }

        for (i, suggestion) in self.suggestions.iter().enumerate() {
            println!(
                "\n{}. {} (Confidence: {})",
                i + 1,
                suggestion.field.to_uppercase(),
                suggestion.confidence
            );
            println!("   Current:  {:?}", suggestion.current_value);
            println!("   Suggested: {}", suggestion.suggested_value);
            println!("   Reason: {}", suggestion.reason);
        }

        println!("\n{}", "-".repeat(62));
        println!("📝 LLM Analysis Summary:");
        println!(
            "{}",
            self.llm_analysis
                .lines()
                .take(3)
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}
