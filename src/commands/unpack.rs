use colored::Colorize;
use indicatif::{ProgressBar, ProgressStyle};

/// Runs the `bento unpack` command.
pub fn run(name: &str) -> anyhow::Result<()> {
    let entry = crate::vault::index::find_by_name_or_tag(name)?
        .ok_or_else(|| anyhow::anyhow!("No project named or tagged '{}' found in vault", name))?;

    let workspace = crate::vault::paths::workspace_dir()?;
    let dest = workspace.join(&entry.project_name);
    std::fs::create_dir_all(&dest)?;

    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .unwrap(),
    );
    spinner.set_message(format!("Unpacking {}...", entry.project_name.bold()));
    spinner.enable_steady_tick(std::time::Duration::from_millis(80));

    crate::vault::archive::extract(&entry.archive_path, &dest, &entry.algorithm)?;

    spinner.finish_and_clear();

    eprintln!(
        "{} Unpacked '{}' to {}",
        "Done!".green().bold(),
        entry.project_name.cyan(),
        dest.display().to_string().dimmed()
    );
    println!("__bento_cd:{}", dest.display());

    Ok(())
}
