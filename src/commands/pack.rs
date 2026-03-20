use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

/// Default directories to exclude from archives.
#[allow(dead_code)]
const EXCLUDE_DIRS: &[&str] = &[".git", "target", "node_modules", ".venv", "__pycache__", ".tox"];

/// Runs the `bento pack` command.
pub fn run(tag: &str, algo: Option<&str>, repo: bool, force: bool) -> anyhow::Result<()> {
    let project_dir = std::env::current_dir()?;
    let project_name = project_dir
        .file_name()
        .ok_or_else(|| anyhow::anyhow!("Could not determine project name from current directory"))?
        .to_string_lossy()
        .to_string();

    // Read default algo from config if not specified
    let algo = match algo {
        Some(a) => a.to_string(),
        None => crate::config::load_config()?.default_algo,
    };

    if repo {
        crate::vault::github::create_and_push(&project_dir, &project_name)?;
    }

    // Calculate original size before compressing
    let original_size = crate::vault::archive::dir_size(&project_dir)?;

    let timestamp = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let ext = crate::vault::archive::get_extension(&algo);
    let filename = format!("{project_name}_{tag}_{timestamp}{ext}");

    // Compress with progress spinner
    let dest_file = std::env::temp_dir().join(&filename);
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!(
        "Packing {} with {}...",
        project_name.bold(),
        algo.cyan()
    ));
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    crate::vault::archive::compress(&project_dir, &dest_file, &algo)?;

    spinner.finish_and_clear();

    // Move archive into the vault
    let vault = crate::vault::paths::vault_dir()?;
    let final_path = vault.join(&filename);
    if std::fs::rename(&dest_file, &final_path).is_err() {
        std::fs::copy(&dest_file, &final_path)?;
        std::fs::remove_file(&dest_file)?;
    }

    let archive_size = std::fs::metadata(&final_path)?.len();

    // Log to index
    crate::vault::index::add_entry(crate::vault::index::IndexEntry {
        project_name: project_name.clone(),
        tag: tag.to_string(),
        timestamp: chrono::Utc::now(),
        archive_path: final_path,
        algorithm: algo.to_string(),
        original_path: project_dir.clone(),
        original_size: Some(original_size),
    })?;

    // Confirm before deleting
    if !force {
        let confirm = dialoguer::Confirm::new()
            .with_prompt(format!(
                "Delete original folder {}?",
                project_dir.display().to_string().yellow()
            ))
            .default(true)
            .interact()?;

        if !confirm {
            println!(
                "{} Archive saved but original folder kept.",
                "Skipped.".yellow()
            );
            return Ok(());
        }
    }

    // cd up before deleting
    if let Some(parent) = project_dir.parent() {
        std::env::set_current_dir(parent)?;
    }
    std::fs::remove_dir_all(&project_dir)?;

    println!(
        "{} '{}' archived with tag '{}' ({} -> {})",
        "Done!".green().bold(),
        project_name.cyan(),
        tag.cyan(),
        crate::vault::archive::format_size(original_size).dimmed(),
        crate::vault::archive::format_size(archive_size).green(),
    );
    Ok(())
}
