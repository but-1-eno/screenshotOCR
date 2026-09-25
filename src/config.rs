use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub hotkey: String,
}

impl AppConfig {
    pub fn load_or_prompt() -> Result<Self> {
        let path = config_path()?;
        if path.exists() {
            let contents = fs::read_to_string(&path)
                .with_context(|| format!("unable to read {}", path.display()))?;
            return serde_json::from_str(&contents)
                .with_context(|| format!("invalid configuration in {}", path.display()));
        }

        println!("First run: enter a global shortcut, for example Ctrl+Shift+S.");
        let mut hotkey = String::new();
        std::io::stdin().read_line(&mut hotkey)?;
        let hotkey = hotkey.trim().to_string();
        if hotkey.is_empty() {
            anyhow::bail!("a shortcut is required");
        }

        let config = Self { hotkey };
        fs::write(&path, serde_json::to_vec_pretty(&config)?)
            .with_context(|| format!("unable to write {}", path.display()))?;
        println!("Shortcut saved to {}", path.display());
        Ok(config)
    }
}

fn config_path() -> Result<PathBuf> {
    let executable = env::current_exe().context("unable to locate executable")?;
    let directory = executable
        .parent()
        .context("executable has no parent directory")?;
    Ok(directory.join("screenshot-ocr.json"))
}

use std::io::BufRead;
