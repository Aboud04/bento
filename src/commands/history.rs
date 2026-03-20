use colored::Colorize;
use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, Color, ContentArrangement, Table};

/// Runs the `bento history` command.
/// Shows a timeline of all pack operations, sorted newest first.
pub fn run() -> anyhow::Result<()> {
    let mut entries = crate::vault::index::load_entries()?;

    if entries.is_empty() {
        println!("{}", "No history yet.".dimmed());
        return Ok(());
    }

    // Sort by timestamp, newest first
    entries.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Date").fg(Color::White),
            Cell::new("Time").fg(Color::White),
            Cell::new("Project").fg(Color::White),
            Cell::new("Tag").fg(Color::White),
            Cell::new("Algorithm").fg(Color::White),
            Cell::new("Original Path").fg(Color::White),
        ]);

    for entry in &entries {
        table.add_row(vec![
            Cell::new(entry.timestamp.format("%Y-%m-%d").to_string()),
            Cell::new(entry.timestamp.format("%H:%M").to_string()).fg(Color::DarkGrey),
            Cell::new(&entry.project_name).fg(Color::Cyan),
            Cell::new(&entry.tag).fg(Color::Green),
            Cell::new(&entry.algorithm),
            Cell::new(entry.original_path.display().to_string()).fg(Color::DarkGrey),
        ]);
    }

    println!("{}", "History (newest first)".bold().underline());
    println!();
    println!("{table}");
    Ok(())
}
