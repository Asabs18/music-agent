pub mod reader;
pub mod writer;

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackMetadata {
    pub file_path: String,
    pub artist: Option<String>,
    pub title: Option<String>,
    pub album: Option<String>,
    pub year: Option<i32>,
    pub genre: Option<String>,
    pub track_number: Option<u32>,
    pub album_artist: Option<String>,
    pub duration_seconds: Option<u32>,
}

impl TrackMetadata {
    /// Returns true if critical metadata is missing
    pub fn has_missing_critical_fields(&self) -> bool {
        self.artist.is_none() || self.title.is_none()
    }

    /// Returns a list of missing fields
    pub fn missing_fields(&self) -> Vec<&str> {
        let mut missing = Vec::new();
        if self.artist.is_none() {
            missing.push("artist");
        }
        if self.title.is_none() {
            missing.push("title");
        }
        if self.album.is_none() {
            missing.push("album");
        }
        if self.year.is_none() {
            missing.push("year");
        }
        if self.genre.is_none() {
            missing.push("genre");
        }
        missing
    }

    /// Format metadata for LLM prompt
    pub fn to_prompt_format(&self) -> String {
        format!(
            r#"File: {}

Current Metadata:
- Artist: {}
- Title: {}
- Album: {}
- Year: {}
- Genre: {}
- Track Number: {}
- Album Artist: {}
- Duration: {} seconds

Missing Fields: {}"#,
            self.file_path,
            self.artist.as_deref().unwrap_or("(missing)"),
            self.title.as_deref().unwrap_or("(missing)"),
            self.album.as_deref().unwrap_or("(missing)"),
            self.year
                .map(|y| y.to_string())
                .unwrap_or_else(|| "(missing)".to_string()),
            self.genre.as_deref().unwrap_or("(missing)"),
            self.track_number
                .map(|t| t.to_string())
                .unwrap_or_else(|| "(missing)".to_string()),
            self.album_artist.as_deref().unwrap_or("(missing)"),
            self.duration_seconds
                .map(|d| d.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            if self.missing_fields().is_empty() {
                "None".to_string()
            } else {
                self.missing_fields().join(", ")
            }
        )
    }
}

impl fmt::Display for TrackMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "🎵 {}\n   Artist: {}\n   Album: {}\n   Year: {}\n   Genre: {}",
            self.title.as_deref().unwrap_or("Unknown Title"),
            self.artist.as_deref().unwrap_or("Unknown Artist"),
            self.album.as_deref().unwrap_or("Unknown Album"),
            self.year
                .map(|y| y.to_string())
                .unwrap_or_else(|| "Unknown".to_string()),
            self.genre.as_deref().unwrap_or("Unknown"),
        )
    }
}
