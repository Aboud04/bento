use colored::Colorize;

/// Runs the `bento search` command.
pub fn run(query: &str) -> anyhow::Result<()> {
    let entries = crate::vault::index::load_entries()?;
    let query_lower = query.to_lowercase();
    let matches: Vec<_> = entries.into_iter()
        .filter(|e| {
            e.project_name.to_lowercase().contains(&query_lower)
            || e.tag.to_lowercase().contains(&query_lower)
        })
        .collect();

    if matches.is_empty() {
        println!("{} No projects matching '{}'", "Not found.".yellow(), query.cyan());
    } else {
        println!(
            "{} {} result(s) for '{}'",
            "Found".green().bold(),
            matches.len(),
            query.cyan()
        );
        crate::commands::list::print_table(&matches);
    }
    Ok(())
}
