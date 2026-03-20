use colored::Colorize;
use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, Color, ContentArrangement, Table};

use crate::vault::index::IndexEntry;

/// Runs the `bento list` command.
pub fn run() -> anyhow::Result<()> {
    let entries = crate::vault::index::load_entries()?;

    if entries.is_empty() {
        println!("{}", "No projects archived yet.".dimmed());
        return Ok(());
    }

    print_table(&entries);
    Ok(())
}

/// Prints a slice of index entries as a formatted table.
pub fn print_table(entries: &[IndexEntry]) {
    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("#").fg(Color::White),
            Cell::new("Project").fg(Color::White),
            Cell::new("Tag").fg(Color::White),
            Cell::new("Date").fg(Color::White),
            Cell::new("Algorithm").fg(Color::White),
            Cell::new("Size").fg(Color::White),
        ]);

    for (i, entry) in entries.iter().enumerate() {
        let archive_size = entry
            .archive_path
            .metadata()
            .map(|m| crate::vault::archive::format_size(m.len()))
            .unwrap_or_else(|_| "?".to_string());

        table.add_row(vec![
            Cell::new(i + 1),
            Cell::new(&entry.project_name).fg(Color::Cyan),
            Cell::new(&entry.tag).fg(Color::Green),
            Cell::new(entry.timestamp.format("%Y-%m-%d").to_string()),
            Cell::new(&entry.algorithm),
            Cell::new(archive_size).fg(Color::Yellow),
        ]);
    }

    println!("{table}");
}
