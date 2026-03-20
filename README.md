# <img src="https://em-content.zobj.net/source/apple/391/bento-box_1f371.png" width="32" height="32" alt="bento"> Bento

**A local project vault for developers.** Compress, stash, and restore project folders from the command line.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![Platform](https://img.shields.io/badge/Platform-Linux%20%7C%20Windows-lightgrey.svg)]()

---

Bento is a cross-platform CLI tool that acts as a **project archiver**. It packages entire project directories into compressed archives, stores them in a hidden vault on your machine, and maintains a searchable index of everything you've stashed. When you need a project back, one command unpacks it and drops you right into the directory.

**The problem it solves:** Developer machines accumulate dozens of inactive project folders that waste disk space and clutter your workspace. You don't want to delete them, but you don't need them taking up room either. Bento compresses them away and brings them back instantly.

## Features

- **One-command archiving** — Package any project folder into a compressed archive
- **7 compression algorithms** — zstd (default), gzip, bzip2, xz, lz4, snappy, brotli
- **Searchable vault** — Browse and search all archived projects by name or tag
- **Instant restore** — Unpack any project and auto-`cd` into its directory
- **Tab completion** — Autocomplete project names on delete, unpack, info, export, and rename
- **Space tracking** — See how much disk space you've saved with `bento stats`
- **GitHub integration** — Optionally push to GitHub before archiving (via `gh` CLI)
- **Pretty output** — Colored tables, progress spinners, and confirmation prompts
- **Configurable defaults** — Set your preferred compression algorithm once
- **Cross-platform** — Works on Linux and Windows with platform-appropriate paths

## Installation

### Prebuilt binaries (recommended)

Download the latest binary from [GitHub Releases](https://github.com/Aboud04/bento/releases/latest):

**Linux:**
```bash
curl -fsSL https://github.com/Aboud04/bento/releases/latest/download/bento-linux-x86_64.tar.gz | tar xz -C ~/.local/bin/
```

**macOS:**
```bash
curl -fsSL https://github.com/Aboud04/bento/releases/latest/download/bento-macos-aarch64.tar.gz | tar xz -C /usr/local/bin/
```

**Windows:** Download `bento-windows-x86_64.zip` from the [Releases page](https://github.com/Aboud04/bento/releases/latest) and add it to your PATH.

### From source

```bash
git clone https://github.com/Aboud04/bento.git
cd bento
cargo install --path .
```

This installs two binaries: `bento` and `bt` (shorthand alias).

### Shell integration

After installing, run this once to enable auto-cd and tab completion:

```bash
bento init
source ~/.bashrc   # or ~/.zshrc
```

<details>
<summary><strong>What does this do?</strong></summary>

It appends a shell function and completion script to your shell config. The function intercepts bento's output to handle `cd` after unpack (since a child process can't change the parent shell's directory). The completion script enables tab-completion for commands, project names, and algorithm names. This is the same pattern used by [zoxide](https://github.com/ajeetdsouza/zoxide) and other tools.

To remove it: `bento uninit`

</details>

## Quick Start

```bash
# Navigate to a project you want to archive
cd ~/projects/old-side-project

# Pack it — compresses, vaults, and removes the original
bento pack "v1.0-stable"

# See everything in your vault
bento list

# Search by name or tag
bento search "side-project"

# Bring it back
bento unpack old-side-project
# → You're now inside ~/.bento/workspace/old-side-project/
```

## Commands

### Core

| Command | Description |
|---------|-------------|
| `bento pack "<tag>"` | Compress and archive the current directory |
| `bento unpack <name>` | Restore a project and cd into it |
| `bento list` | Show all archived projects |
| `bento search <query>` | Search by project name or tag |
| `bento info <name>` | Detailed view of a single archive |

### Management

| Command | Description |
|---------|-------------|
| `bento delete <name>` | Remove an archive from the vault |
| `bento rename <old> <new>` | Rename a project in the vault |
| `bento export <name> <path>` | Extract to a specific directory |
| `bento import <file>` | Import an external archive into the vault |
| `bento clean` | Remove unpacked workspace copies to free space |

### Configuration

| Command | Description |
|---------|-------------|
| `bento config` | Show current configuration |
| `bento config --algo <algo>` | Set default compression algorithm |
| `bento stats` | Vault overview — total size, space saved, per-algorithm breakdown |
| `bento history` | Timeline of all pack operations |
| `bento init` | Install shell integration (auto-cd + tab completion) |
| `bento uninit` | Remove shell integration |

### Pack options

```bash
bento pack "release-1.0"                    # default algorithm (zstd)
bento pack "backup" --algo bzip2            # specific algorithm
bento pack "final" --repo                   # push to GitHub first
bento pack "quick" --force                  # skip delete confirmation
```

## Compression Algorithms

Tested on 137 MB of real data (log files, CSV, JSON, source code):

| Algorithm | Archive Size | Ratio | Speed | Best For |
|-----------|-------------|-------|-------|----------|
| **bzip2** | 8.6 MB | 6.3% | Slow | Maximum compression |
| **xz** | 9.0 MB | 6.6% | Slow | Maximum compression |
| **brotli** | 13.3 MB | 9.7% | Medium | Web-optimized |
| **zstd** (default) | 14.2 MB | 10.4% | Fast | Best speed/ratio balance |
| **gzip** | 15.8 MB | 11.5% | Medium | Universal compatibility |
| **snappy** | 27.3 MB | 19.9% | Very fast | Speed-critical |
| **lz4** | 27.6 MB | 20.1% | Very fast | Speed-critical |

Set your preferred default:

```bash
bento config --algo lz4    # if you want speed
bento config --algo bzip2  # if you want smallest archives
```

## How It Works

```
~/.bento/
├── vault/          # Compressed archives (.tar.zst, .tar.gz, etc.)
├── workspace/      # Unpacked projects (restored here)
├── index.json      # Master index of all archived projects
└── config.json     # User configuration (default algorithm, etc.)
```

1. **Pack**: `tar` the directory → pipe through compressor → write to `vault/` → log in `index.json` → delete original
2. **List/Search**: Read and display `index.json`
3. **Unpack**: Look up in `index.json` → decompress from `vault/` to `workspace/` → shell wrapper runs `cd`

## Project Structure

```
src/
├── main.rs              # CLI entry point and command routing
├── config.rs            # Configuration management
├── commands/
│   ├── pack.rs          # bento pack
│   ├── list.rs          # bento list
│   ├── search.rs        # bento search
│   ├── unpack.rs        # bento unpack
│   ├── stats.rs         # bento stats
│   ├── info.rs          # bento info
│   ├── rename.rs        # bento rename
│   ├── export.rs        # bento export
│   ├── import.rs        # bento import
│   ├── history.rs       # bento history
│   └── clean.rs         # bento clean
├── vault/
│   ├── paths.rs         # Cross-platform path resolution
│   ├── archive.rs       # Compression/decompression (7 algorithms)
│   ├── index.rs         # JSON index read/write
│   └── github.rs        # GitHub integration via gh CLI
└── shell/
    └── wrapper.rs       # Shell wrapper and completion generation
tests/
└── integration.rs       # 80 tests — unit + integration
```

## Requirements

- **Rust** 1.75 or later
- **`gh` CLI** (optional) — only needed for `--repo` flag
- **Linux** or **Windows**

## Development

```bash
cargo run -- list               # run in development
cargo test                      # run all 80 tests
cargo clippy -- -D warnings     # lint
cargo fmt                       # format
```

## Contributing

Contributions are welcome. Please open an issue first to discuss what you'd like to change.

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-feature`)
3. Commit your changes
4. Open a Pull Request

## License

MIT — see [LICENSE](LICENSE) for details.

## Acknowledgements

- [clap](https://github.com/clap-rs/clap) — CLI argument parsing
- [zstd](https://facebook.github.io/zstd/) — Fast real-time compression
- [comfy-table](https://github.com/nukesor/comfy-table) — Terminal table rendering
- [indicatif](https://github.com/console-rs/indicatif) — Progress bars
- [zoxide](https://github.com/ajeetdsouza/zoxide) — Inspiration for the shell wrapper pattern
