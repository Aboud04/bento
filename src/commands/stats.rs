use colored::Colorize;
use comfy_table::{presets::UTF8_FULL_CONDENSED, Cell, Color, ContentArrangement, Table};

/// Runs the `bento stats` command.
pub fn run() -> anyhow::Result<()> {
    let entries = crate::vault::index::load_entries()?;

    if entries.is_empty() {
        println!("{}", "No projects archived yet.".dimmed());
        return Ok(());
    }

    let total_archives = entries.len();

    let mut total_archive_size: u64 = 0;
    let mut total_original_size: u64 = 0;
    let mut algo_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

    for entry in &entries {
        if let Ok(meta) = std::fs::metadata(&entry.archive_path) {
            total_archive_size += meta.len();
        }
        if let Some(orig) = entry.original_size {
            total_original_size += orig;
        }
        *algo_counts.entry(entry.algorithm.clone()).or_insert(0) += 1;
    }

    let space_saved = total_original_size.saturating_sub(total_archive_size);

    // Overview
    println!("{}", "Vault Overview".bold().underline());
    println!();

    let mut overview = Table::new();
    overview
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .add_row(vec![
            Cell::new("Total archives").fg(Color::White),
            Cell::new(total_archives).fg(Color::Cyan),
        ])
        .add_row(vec![
            Cell::new("Vault size").fg(Color::White),
            Cell::new(crate::vault::archive::format_size(total_archive_size)).fg(Color::Yellow),
        ]);

    if total_original_size > 0 {
        let ratio = if total_original_size > 0 {
            (total_archive_size as f64 / total_original_size as f64) * 100.0
        } else {
            0.0
        };

        overview.add_row(vec![
            Cell::new("Original size").fg(Color::White),
            Cell::new(crate::vault::archive::format_size(total_original_size)).fg(Color::White),
        ]);
        overview.add_row(vec![
            Cell::new("Space saved").fg(Color::White),
            Cell::new(format!(
                "{} ({:.1}% reduction)",
                crate::vault::archive::format_size(space_saved),
                100.0 - ratio
            ))
            .fg(Color::Green),
        ]);
    }

    println!("{overview}");
    println!();

    // Per-algorithm breakdown
    println!("{}", "By Algorithm".bold().underline());
    println!();

    let mut algo_table = Table::new();
    algo_table
        .load_preset(UTF8_FULL_CONDENSED)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Algorithm").fg(Color::White),
            Cell::new("Count").fg(Color::White),
        ]);

    let mut sorted_algos: Vec<_> = algo_counts.into_iter().collect();
    sorted_algos.sort_by(|a, b| b.1.cmp(&a.1));

    for (algo, count) in sorted_algos {
        algo_table.add_row(vec![
            Cell::new(&algo).fg(Color::Cyan),
            Cell::new(count),
        ]);
    }

    println!("{algo_table}");

    Ok(())
}
