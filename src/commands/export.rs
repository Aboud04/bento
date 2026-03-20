use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

/// Runs the `bento export` command.
/// Extracts an archived project to a specific user-chosen directory.
pub fn run(name: &str, dest: &str) -> anyhow::Result<()> {
    let entry = crate::vault::index::find_by_name_or_tag(name)?
        .ok_or_else(|| anyhow::anyhow!("No project named or tagged '{}' found", name))?;

    let dest_path = std::path::PathBuf::from(dest);
    std::fs::create_dir_all(&dest_path)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!(
        "Exporting {} to {}...",
        entry.project_name.bold(),
        dest_path.display()
    ));
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    crate::vault::archive::extract(&entry.archive_path, &dest_path, &entry.algorithm)?;

    spinner.finish_and_clear();

    println!(
        "{} Exported '{}' to {}",
        "Done!".green().bold(),
        entry.project_name.cyan(),
        dest_path.display().to_string().dimmed()
    );
    Ok(())
}
