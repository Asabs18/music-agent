use crate::error::{AgentError, Result};
use crate::metadata::TrackMetadata;
use id3::{Tag, TagLike};
use std::path::Path;

/// Reads ID3 metadata from an MP3 file
pub fn read_metadata(file_path: &str) -> Result<TrackMetadata> {
    let path = Path::new(file_path);

    // Verify file exists
    if !path.exists() {
        return Err(AgentError::FileRead(format!(
            "File not found: {}",
            file_path
        )));
    }

    // Verify it's an MP3
    if path.extension().and_then(|s| s.to_str()) != Some("mp3") {
        return Err(AgentError::FileRead(format!(
            "Not an MP3 file: {}",
            file_path
        )));
    }

    // Read ID3 tags
    let tag = Tag::read_from_path(path).map_err(|e| {
        AgentError::MetadataParse(format!("Failed to read ID3 tags from {}: {}", file_path, e))
    })?;

    // Extract metadata
    Ok(TrackMetadata {
        file_path: file_path.to_string(),
        artist: tag.artist().map(|s| s.to_string()),
        title: tag.title().map(|s| s.to_string()),
        album: tag.album().map(|s| s.to_string()),
        year: tag.year(),
        genre: tag.genre().map(|s| s.to_string()),
        track_number: tag.track(),
        album_artist: tag.album_artist().map(|s| s.to_string()),
        duration_seconds: tag.duration(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_metadata("nonexistent.mp3");
        assert!(result.is_err());
    }

    #[test]
    fn test_read_non_mp3() {
        let result = read_metadata("Cargo.toml");
        assert!(result.is_err());
    }
}
