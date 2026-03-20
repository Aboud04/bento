use colored::Colorize;

/// Runs the `bento clean` command.
/// Removes all unpacked project copies from ~/.bento/workspace/ to free disk space.
pub fn run(force: bool) -> anyhow::Result<()> {
    let workspace = crate::vault::paths::workspace_dir()?;

    let entries: Vec<_> = std::fs::read_dir(&workspace)?
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_dir())
        .collect();

    if entries.is_empty() {
        println!("{}", "Workspace is already clean.".dimmed());
        return Ok(());
    }

    // Calculate total size
    let mut total_size: u64 = 0;
    let mut names = Vec::new();
    for entry in &entries {
        if let Ok(size) = crate::vault::archive::dir_size(&entry.path()) {
            total_size += size;
        }
        names.push(
            entry
                .file_name()
                .to_string_lossy()
                .to_string(),
        );
    }

    println!(
        "Found {} unpacked project(s) using {}:",
        entries.len().to_string().cyan(),
        crate::vault::archive::format_size(total_size).yellow()
    );
    for name in &names {
        println!("  - {}", name.cyan());
    }

    if !force {
        let confirm = dialoguer::Confirm::new()
            .with_prompt("Remove all unpacked copies?")
            .default(false)
            .interact()?;

        if !confirm {
            println!("{}", "Cancelled.".yellow());
            return Ok(());
        }
    }

    for entry in &entries {
        std::fs::remove_dir_all(entry.path())?;
    }

    println!(
        "{} Cleaned workspace, freed {}",
        "Done!".green().bold(),
        crate::vault::archive::format_size(total_size).green()
    );
    Ok(())
}
