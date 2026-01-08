use crate::error::{AgentError, Result};
use crate::metadata::TrackMetadata;
use id3::{Tag, TagLike, Version};
use std::fs;
use std::path::{Path, PathBuf};

/// Writes ID3 metadata to a NEW copy of an MP3 file (never overwrites original)
pub fn write_metadata_safely(original_file: &str, metadata: &TrackMetadata) -> Result<String> {
    let original_path = Path::new(original_file);

    // Verify original file exists
    if !original_path.exists() {
        return Err(AgentError::FileRead(format!(
            "File not found: {}",
            original_file
        )));
    }

    // Create output path with .updated.mp3 suffix
    let output_path = create_output_path(original_path);

    // Copy original to new file
    fs::copy(original_path, &output_path)
        .map_err(|e| AgentError::FileRead(format!("Failed to create output file: {}", e)))?;

    // Read existing tag or create new one
    let mut tag = Tag::read_from_path(&output_path).unwrap_or_else(|_| Tag::new());

    // Update fields if provided
    if let Some(ref artist) = metadata.artist {
        tag.set_artist(artist);
    }

    if let Some(ref title) = metadata.title {
        tag.set_title(title);
    }

    if let Some(ref album) = metadata.album {
        tag.set_album(album);
    }

    if let Some(year) = metadata.year {
        tag.set_year(year);
    }

    if let Some(ref genre) = metadata.genre {
        tag.set_genre(genre);
    }

    if let Some(track) = metadata.track_number {
        tag.set_track(track);
    }

    if let Some(ref album_artist) = metadata.album_artist {
        tag.set_album_artist(album_artist);
    }

    // Write to the NEW file with ID3v2.4
    tag.write_to_path(&output_path, Version::Id3v24)
        .map_err(|e| AgentError::MetadataParse(format!("Failed to write ID3 tags: {}", e)))?;

    Ok(output_path.to_string_lossy().to_string())
}

/// Create a safe output path that doesn't overwrite the original
/// Saves to public/updated/ directory
fn create_output_path(original: &Path) -> PathBuf {
    let stem = original.file_stem().unwrap_or_default();

    // Determine the public/updated directory
    let updated_dir = if let Some(parent) = original.parent() {
        // If file is in public/originals/, use public/updated/
        if parent.ends_with("originals") {
            parent.parent().unwrap_or(parent).join("updated")
        } else if parent.ends_with("public") {
            parent.join("updated")
        } else {
            // For other locations, create updated/ subdirectory
            parent.join("updated")
        }
    } else {
        PathBuf::from("public/updated")
    };

    // Ensure the directory exists
    let _ = fs::create_dir_all(&updated_dir);

    // Create filename like "02 Friend of the Devil.mp3" (no .updated suffix needed since it's in updated/)
    let mut output_path = updated_dir.join(format!("{}.mp3", stem.to_string_lossy()));

    // If file already exists, add number suffix
    let mut counter = 1;
    while output_path.exists() {
        output_path = updated_dir.join(format!("{}-{}.mp3", stem.to_string_lossy(), counter));
        counter += 1;
    }

    output_path
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_output_path_creation() {
        let path = Path::new("test/song.mp3");
        let output = create_output_path(path);
        assert!(output.to_string_lossy().contains(".updated.mp3"));
    }
}
