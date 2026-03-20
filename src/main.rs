mod commands;
mod config;
mod shell;
mod vault;

use clap::{CommandFactory, Parser, Subcommand};
use colored::Colorize;

#[derive(Parser)]
#[command(
    name = "bento",
    about = "A local project vault CLI — compress, stash, and restore project folders.",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Compress and stash the current project directory
    Pack {
        /// A tag/version label for the archive (e.g. "v1.0")
        tag: String,

        /// Compression algorithm to use (uses config default if not set)
        #[arg(short, long)]
        algo: Option<String>,

        /// Create a GitHub repo and push before archiving
        #[arg(long)]
        repo: bool,

        /// Skip confirmation prompt before deleting original
        #[arg(short, long)]
        force: bool,
    },

    /// List all archived projects
    List,

    /// Search archived projects by name or tag
    Search {
        /// The search query string
        query: String,
    },

    /// Restore an archived project to the workspace
    Unpack {
        /// Project name or tag to unpack
        name: String,
    },

    /// Show vault statistics and space saved
    Stats,

    /// Set bento configuration options
    Config {
        /// Set the default compression algorithm
        #[arg(long)]
        algo: Option<String>,
    },

    /// Install shell integration (auto-cd + tab completions)
    Init,

    /// Remove bento shell integration
    Uninit,

    /// Remove an archive from the vault
    Delete {
        /// Project name to delete
        name: String,
    },

    /// Rename a project in the vault
    Rename {
        /// Current project name
        old: String,
        /// New project name
        new: String,
    },

    /// Show detailed info about an archived project
    Info {
        /// Project name or tag
        name: String,
    },

    /// Export an archived project to a specific directory
    Export {
        /// Project name or tag
        name: String,
        /// Destination directory path
        path: String,
    },

    /// Import an external archive file into the vault
    Import {
        /// Path to the archive file
        archive: String,

        /// Project name (guessed from filename if omitted)
        #[arg(short, long)]
        name: Option<String>,

        /// Tag for the imported project
        #[arg(short, long)]
        tag: Option<String>,

        /// Compression algorithm (guessed from extension if omitted)
        #[arg(short, long)]
        algo: Option<String>,
    },

    /// Show a timeline of all pack operations
    History,

    /// Remove all unpacked copies from the workspace to free space
    Clean {
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },

    /// Output project names for shell completion (hidden)
    #[command(hide = true)]
    ListProjects,
}

/// Returns the CLI command definition for use by shell completion generation.
pub fn build_cli() -> clap::Command {
    Cli::command()
}

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Pack {
            tag,
            algo,
            repo,
            force,
        } => commands::pack::run(&tag, algo.as_deref(), repo, force),

        Commands::List => commands::list::run(),

        Commands::Search { query } => commands::search::run(&query),

        Commands::Unpack { name } => commands::unpack::run(&name),

        Commands::Stats => commands::stats::run(),

        Commands::Config { algo } => {
            let mut cfg = config::load_config().unwrap_or_default();
            if let Some(algo) = algo {
                let valid = ["zstd", "gzip", "bzip2", "xz", "lz4", "snappy", "brotli"];
                if !valid.contains(&algo.as_str()) {
                    eprintln!(
                        "{} Unknown algorithm '{}'. Valid: {}",
                        "Error:".red().bold(),
                        algo.yellow(),
                        valid.join(", ").dimmed()
                    );
                    std::process::exit(1);
                }
                cfg.default_algo = algo.clone();
                config::save_config(&cfg).unwrap();
                println!(
                    "{} Default algorithm set to '{}'",
                    "Done!".green().bold(),
                    algo.cyan()
                );
            } else {
                println!("Current config:");
                println!("  default_algo: {}", cfg.default_algo.cyan());
            }
            Ok(())
        }

        Commands::Init => shell::wrapper::install_wrapper(),

        Commands::Uninit => shell::wrapper::uninstall_wrapper(),

        Commands::Delete { name } => delete_project(&name),

        Commands::Rename { old, new } => commands::rename::run(&old, &new),

        Commands::Info { name } => commands::info::run(&name),

        Commands::Export { name, path } => commands::export::run(&name, &path),

        Commands::Import {
            archive,
            name,
            tag,
            algo,
        } => commands::import::run(&archive, name.as_deref(), tag.as_deref(), algo.as_deref()),

        Commands::History => commands::history::run(),

        Commands::Clean { force } => commands::clean::run(force),

        Commands::ListProjects => {
            if let Ok(entries) = vault::index::load_entries() {
                for entry in entries {
                    println!("{}", entry.project_name);
                }
            }
            Ok(())
        }
    };

    if let Err(e) = result {
        eprintln!("{} {e:?}", "Error:".red().bold());
        std::process::exit(1);
    }
}

fn delete_project(name: &str) -> anyhow::Result<()> {
    let entry = vault::index::find_by_name_or_tag(name)?
        .ok_or_else(|| anyhow::anyhow!("No project named or tagged '{}' found", name))?;

    if entry.archive_path.exists() {
        std::fs::remove_file(&entry.archive_path)?;
    }
    vault::index::remove_entry(&entry.project_name)?;

    println!(
        "{} Removed '{}' from vault",
        "Done!".green().bold(),
        entry.project_name.cyan()
    );
    Ok(())
}
