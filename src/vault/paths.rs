use std::path::PathBuf;

/// Returns the base bento directory 
/// Creates the directory if it doesn't exist.
pub fn bento_dir() -> anyhow::Result<PathBuf> {
    let home = dirs::home_dir()
        .ok_or_else(|| anyhow::anyhow!("Could not determine home directory"))?;
    let path = home.join(".bento");
    std::fs::create_dir_all(&path)?;

    Ok(path)
}

/// Returns the vault directory (~/.bento/vault/).
/// Creates the directory if it doesn't exist.
pub fn vault_dir() -> anyhow::Result<PathBuf> {
    let path = bento_dir()?.join("vault");
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

/// Returns the workspace directory (~/.bento/workspace/).
/// Creates the directory if it doesn't exist.
pub fn workspace_dir() -> anyhow::Result<PathBuf> {
    let path = bento_dir()?.join("workspace");
    std::fs::create_dir_all(&path)?;
    Ok(path)
}

/// Returns the path to the config file (~/.bento/config.json).
pub fn config_file() -> anyhow::Result<PathBuf> {
    let path = bento_dir()?.join("config.json");
    Ok(path)
}
pub fn index_file() -> anyhow::Result<PathBuf> {
    let path = bento_dir()?.join("index.json");
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bento_dir_is_under_home() {
        let dir = bento_dir().unwrap();
        let home = dirs::home_dir().unwrap();
        assert!(dir.starts_with(&home));
        assert!(dir.ends_with(".bento"));
        assert!(dir.exists());
    }

    #[test]
    fn vault_dir_creates_directory() {
        let dir = vault_dir().unwrap();
        assert!(dir.ends_with("vault"));
        assert!(dir.exists());
    }

    #[test]
    fn workspace_dir_creates_directory() {
        let dir = workspace_dir().unwrap();
        assert!(dir.ends_with("workspace"));
        assert!(dir.exists());
    }

    #[test]
    fn index_file_path_correct() {
        let path = index_file().unwrap();
        assert!(path.to_string_lossy().ends_with("index.json"));
    }

    #[test]
    fn config_file_path_correct() {
        let path = config_file().unwrap();
        assert!(path.to_string_lossy().ends_with("config.json"));
    }
}
