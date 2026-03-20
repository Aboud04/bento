use colored::Colorize;

/// Runs the `bento rename` command.
/// Renames a project in the vault — updates the index entry and the archive filename.
pub fn run(old_name: &str, new_name: &str) -> anyhow::Result<()> {
    let entry = crate::vault::index::find_by_name_or_tag(old_name)?
        .ok_or_else(|| anyhow::anyhow!("No project named or tagged '{}' found", old_name))?;

    // Build new archive filename by replacing the old name in the path
    let old_filename = entry
        .archive_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let new_filename = old_filename.replacen(&entry.project_name, new_name, 1);
    let new_archive_path = entry.archive_path.with_file_name(&new_filename);

    // Rename the archive file on disk
    if entry.archive_path.exists() {
        std::fs::rename(&entry.archive_path, &new_archive_path)?;
    }

    // Update the index: remove old, add new with updated name and path
    crate::vault::index::remove_entry(&entry.project_name)?;
    crate::vault::index::add_entry(crate::vault::index::IndexEntry {
        project_name: new_name.to_string(),
        archive_path: new_archive_path,
        ..entry
    })?;

    println!(
        "{} Renamed '{}' to '{}'",
        "Done!".green().bold(),
        old_name.cyan(),
        new_name.cyan()
    );
    Ok(())
}
