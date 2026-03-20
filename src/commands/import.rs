use colored::Colorize;

/// Runs the `bento import` command.
/// Imports an external archive file into the bento vault.
pub fn run(
    archive: &str,
    name: Option<&str>,
    tag: Option<&str>,
    algo: Option<&str>,
) -> anyhow::Result<()> {
    let archive_path = std::path::PathBuf::from(archive);
    if !archive_path.exists() {
        return Err(anyhow::anyhow!("File not found: {}", archive));
    }

    // Guess project name from filename if not provided
    let filename = archive_path
        .file_name()
        .unwrap()
        .to_string_lossy()
        .to_string();
    let project_name = name
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            // Strip extensions like .tar.zst, .tar.gz, etc.
            filename
                .strip_suffix(".tar.zst")
                .or_else(|| filename.strip_suffix(".tar.gz"))
                .or_else(|| filename.strip_suffix(".tar.bz2"))
                .or_else(|| filename.strip_suffix(".tar.xz"))
                .or_else(|| filename.strip_suffix(".tar.lz4"))
                .or_else(|| filename.strip_suffix(".tar.sz"))
                .or_else(|| filename.strip_suffix(".tar.br"))
                .unwrap_or(&filename)
                .to_string()
        });

    let tag = tag.unwrap_or("imported");

    // Guess algorithm from extension if not provided
    let algo = algo
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            if filename.ends_with(".tar.zst") {
                "zstd"
            } else if filename.ends_with(".tar.gz") || filename.ends_with(".tgz") {
                "gzip"
            } else if filename.ends_with(".tar.bz2") {
                "bzip2"
            } else if filename.ends_with(".tar.xz") {
                "xz"
            } else if filename.ends_with(".tar.lz4") {
                "lz4"
            } else if filename.ends_with(".tar.sz") {
                "snappy"
            } else if filename.ends_with(".tar.br") {
                "brotli"
            } else {
                "zstd"
            }
            .to_string()
        });

    // Copy archive into vault
    let vault = crate::vault::paths::vault_dir()?;
    let dest = vault.join(&filename);

    if dest.exists() {
        return Err(anyhow::anyhow!(
            "Archive '{}' already exists in vault",
            filename
        ));
    }

    std::fs::copy(&archive_path, &dest)?;

    // Log to index
    crate::vault::index::add_entry(crate::vault::index::IndexEntry {
        project_name: project_name.clone(),
        tag: tag.to_string(),
        timestamp: chrono::Utc::now(),
        archive_path: dest,
        algorithm: algo,
        original_path: archive_path,
        original_size: None,
    })?;

    println!(
        "{} Imported '{}' with tag '{}'",
        "Done!".green().bold(),
        project_name.cyan(),
        tag.cyan()
    );
    Ok(())
}
