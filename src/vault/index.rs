use std::path::PathBuf;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub project_name: String,
    pub tag: String,
    pub timestamp: DateTime<Utc>,
    pub archive_path: PathBuf,
    pub algorithm: String,
    pub original_path: PathBuf,
    /// Original directory size in bytes (None for entries created before this field existed)
    #[serde(default)]
    pub original_size: Option<u64>,
}

/// Loads all entries from ~/.bento/index.json.
/// Returns an empty Vec if the file doesn't exist yet.
pub fn load_entries() -> anyhow::Result<Vec<IndexEntry>> {
    let path = crate::vault::paths::index_file()?;
    if !path.exists() { return Ok(vec![]);}
    
    let contents = std::fs::read_to_string(&path)?;
    let entries: Vec<IndexEntry> = serde_json::from_str(&contents)?;
    Ok(entries)
}

/// Appends a new entry to the index and writes it back to disk.
pub fn add_entry(entry: IndexEntry) -> anyhow::Result<()> {
    let mut entries = load_entries()?;
    entries.push(entry);
    let json = serde_json::to_string_pretty(&entries)?;
    std::fs::write(crate::vault::paths::index_file()?, json)?;
    Ok(())
}

/// Finds the most recent entry matching `name` by project_name or tag (case-insensitive).
pub fn find_by_name_or_tag(name: &str) -> anyhow::Result<Option<IndexEntry>> {
    let entries = load_entries()?;
    let name_lower = name.to_lowercase();
    let result = entries.into_iter()
        .filter(|e| {
            e.project_name.to_lowercase() == name_lower
                || e.tag.to_lowercase() == name_lower
        })
        .max_by_key(|e| e.timestamp);

    Ok(result)
}

/// Finds the most recent entry matching `name` by project_name only (case-insensitive).
#[allow(dead_code)]
pub fn find_by_name(name: &str) -> anyhow::Result<Option<IndexEntry>> {
    let entries = load_entries()?;
    let result = entries.into_iter()
        .filter(|e| e.project_name.to_lowercase() == name.to_lowercase())
        .max_by_key(|e| e.timestamp);

    Ok(result)
}

/// Removes all entries matching `project_name` and writes the index back.
pub fn remove_entry(project_name: &str) -> anyhow::Result<()> {
    let mut entries = load_entries()?;
    entries.retain(|e| e.project_name.to_lowercase() != project_name.to_lowercase());
    let json = serde_json::to_string_pretty(&entries)?;
    std::fs::write(crate::vault::paths::index_file()?, json)?;
    Ok(())
  
}
