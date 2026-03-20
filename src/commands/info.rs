use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, Color, ContentArrangement, Table};

/// Runs the `bento info` command.
/// Shows detailed information about a single archived project.
pub fn run(name: &str) -> anyhow::Result<()> {
    let entry = crate::vault::index::find_by_name_or_tag(name)?
        .ok_or_else(|| anyhow::anyhow!("No project named or tagged '{}' found", name))?;

    let archive_size = entry
        .archive_path
        .metadata()
        .map(|m| m.len())
        .unwrap_or(0);

    let mut table = Table::new();
    table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic);

    table.add_row(vec![
        Cell::new("Project").fg(Color::White),
        Cell::new(&entry.project_name).fg(Color::Cyan),
    ]);
    table.add_row(vec![
        Cell::new("Tag").fg(Color::White),
        Cell::new(&entry.tag).fg(Color::Green),
    ]);
    table.add_row(vec![
        Cell::new("Date").fg(Color::White),
        Cell::new(entry.timestamp.format("%Y-%m-%d %H:%M:%S UTC").to_string()),
    ]);
    table.add_row(vec![
        Cell::new("Algorithm").fg(Color::White),
        Cell::new(&entry.algorithm).fg(Color::Cyan),
    ]);
    table.add_row(vec![
        Cell::new("Archive path").fg(Color::White),
        Cell::new(entry.archive_path.display().to_string()).fg(Color::White),
    ]);
    table.add_row(vec![
        Cell::new("Archive size").fg(Color::White),
        Cell::new(crate::vault::archive::format_size(archive_size)).fg(Color::Yellow),
    ]);
    table.add_row(vec![
        Cell::new("Original path").fg(Color::White),
        Cell::new(entry.original_path.display().to_string()).fg(Color::White),
    ]);

    if let Some(orig_size) = entry.original_size {
        let ratio = if orig_size > 0 {
            (archive_size as f64 / orig_size as f64) * 100.0
        } else {
            0.0
        };

        table.add_row(vec![
            Cell::new("Original size").fg(Color::White),
            Cell::new(crate::vault::archive::format_size(orig_size)),
        ]);
        table.add_row(vec![
            Cell::new("Compression ratio").fg(Color::White),
            Cell::new(format!("{:.1}%", ratio)).fg(Color::Green),
        ]);
    }

    println!("{table}");
    Ok(())
}
