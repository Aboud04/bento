use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_algo")]
    pub default_algo: String,
}

fn default_algo() -> String {
    "zstd".to_string()
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_algo: default_algo(),
        }
    }
}

/// Loads the config from ~/.bento/config.json.
/// Returns default config if file doesn't exist.
pub fn load_config() -> anyhow::Result<Config> {
    let path = crate::vault::paths::config_file()?;
    if !path.exists() {
        return Ok(Config::default());
    }
    let contents = std::fs::read_to_string(&path)?;
    let config: Config = serde_json::from_str(&contents)?;
    Ok(config)
}

/// Saves the config to ~/.bento/config.json.
pub fn save_config(config: &Config) -> anyhow::Result<()> {
    let path = crate::vault::paths::config_file()?;
    let json = serde_json::to_string_pretty(config)?;
    std::fs::write(path, json)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_is_zstd() {
        let config = Config::default();
        assert_eq!(config.default_algo, "zstd");
    }

    #[test]
    fn config_serialization_roundtrip() {
        let config = Config {
            default_algo: "lz4".to_string(),
        };
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.default_algo, "lz4");
    }

    #[test]
    fn config_missing_field_uses_default() {
        let json = "{}";
        let config: Config = serde_json::from_str(json).unwrap();
        assert_eq!(config.default_algo, "zstd");
    }
}
