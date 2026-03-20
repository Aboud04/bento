/// Bash/Zsh shell wrapper function.
/// Includes auto-cd on unpack and dynamic project name completions.
const BASH_WRAPPER: &str = r#"
bento() {
    local output
    output=$(command bento "$@")
    if [[ "$output" == *"__bento_cd:"* ]]; then
        local dir="${output#*__bento_cd:}"
        cd "$dir" || return
    else
        echo "$output"
    fi
}

_bento_completions() {
    local cur prev commands
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    commands="pack list search unpack stats config init delete rename info export import history clean help"

    case "$prev" in
        bento|bt)
            COMPREPLY=($(compgen -W "$commands" -- "$cur"))
            return
            ;;
        delete|unpack|info|rename|export)
            local projects
            projects=$(command bento list-projects 2>/dev/null)
            COMPREPLY=($(compgen -W "$projects" -- "$cur"))
            return
            ;;
        --algo|-a)
            COMPREPLY=($(compgen -W "zstd gzip bzip2 xz lz4 snappy brotli" -- "$cur"))
            return
            ;;
    esac

    if [[ "$cur" == -* ]]; then
        case "${COMP_WORDS[1]}" in
            pack)
                COMPREPLY=($(compgen -W "--algo --repo --force --help" -- "$cur"))
                ;;
            config)
                COMPREPLY=($(compgen -W "--algo --help" -- "$cur"))
                ;;
            clean)
                COMPREPLY=($(compgen -W "--force --help" -- "$cur"))
                ;;
            import)
                COMPREPLY=($(compgen -W "--name --tag --algo --help" -- "$cur"))
                ;;
            *)
                COMPREPLY=($(compgen -W "--help" -- "$cur"))
                ;;
        esac
    fi
}

complete -F _bento_completions bento
complete -F _bento_completions bt
"#;

/// PowerShell wrapper function — same logic in PowerShell syntax.
#[allow(dead_code)]
const POWERSHELL_WRAPPER: &str = r#"
# >>> bento >>>
function bento {
    $output = & (Get-Command bento -CommandType Application | Select-Object -First 1).Source @args
    if ($output -match '__bento_cd:(.+)') {
        Set-Location $Matches[1]
    } else {
        $output
    }
}
# <<< bento <<<
"#;

/// Installs the shell wrapper and completions into the user's shell config file.
///
/// Detects the shell from $SHELL, picks the right config file
/// (~/.bashrc, ~/.zshrc, etc.), checks if already installed via
/// marker comments, and appends wrapper + completions.
pub fn install_wrapper() -> anyhow::Result<()> {
    use std::io::Write;

    let shell = std::env::var("SHELL").unwrap_or_default();

    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    let (config_path, wrapper) = if shell.contains("zsh") {
        (home.join(".zshrc"), BASH_WRAPPER)
    } else if shell.contains("bash") {
        (home.join(".bashrc"), BASH_WRAPPER)
    } else if shell.contains("fish") {
        (home.join(".config/fish/config.fish"), BASH_WRAPPER)
    } else {
        return Err(anyhow::anyhow!("Unsupported shell: {shell}. Manually add the wrapper to your shell config."));
    };

    let contents = std::fs::read_to_string(&config_path).unwrap_or_default();
    if contents.contains("# >>> bento >>>") {
        println!("Bento shell integration already installed.");
        println!("To reinstall, first run: bento uninit");
        return Ok(());
    }

    let block = format!("\n# >>> bento >>>\n{}\n# <<< bento <<<\n", wrapper.trim());

    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(&config_path)?;
    file.write_all(block.as_bytes())?;

    println!("Bento shell integration installed (auto-cd + tab completions).");
    println!("Run: source {}", config_path.display());
    Ok(())
}

/// Removes the bento wrapper block (between marker comments) from the shell config.
pub fn uninstall_wrapper() -> anyhow::Result<()> {
    let shell = std::env::var("SHELL").unwrap_or_default();

    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;

    let config_path = if shell.contains("zsh") {
        home.join(".zshrc")
    } else if shell.contains("bash") {
        home.join(".bashrc")
    } else if shell.contains("fish") {
        home.join(".config/fish/config.fish")
    } else {
        return Err(anyhow::anyhow!("Unsupported shell: {shell}"));
    };

    let contents = std::fs::read_to_string(&config_path)?;

    let start_marker = "# >>> bento >>>";
    let end_marker = "# <<< bento <<<";

    let Some(start) = contents.find(start_marker) else {
        println!("No shell wrapper found to remove.");
        return Ok(());
    };

    let Some(end) = contents.find(end_marker) else {
        return Err(anyhow::anyhow!("Found start marker but no end marker — config may be corrupted"));
    };

    let mut cleaned = String::new();
    cleaned.push_str(contents[..start].trim_end_matches('\n'));
    cleaned.push_str(&contents[end + end_marker.len()..]);

    std::fs::write(&config_path, cleaned)?;
    println!("Shell wrapper removed from {}", config_path.display());
    Ok(())
}
