use std::fs;
use std::path::Path;

use anyhow::Context;
use serde::Deserialize;

use crate::logger::Logger;

/// Contents of `.cargo-for-rust-rover.toml` in the workspace root.
#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    /// Package name prefixes that receive `-p` on workspace-wide runs.
    pub(crate) package_prefixes: Vec<String>,
}

impl Config {
    /// Reads and parses `{workspace_root}/.cargo-for-rust-rover.toml`.
    ///
    /// If the file is missing, logs and returns `Ok(None)` so the caller can
    /// continue without filtering.
    pub(crate) fn load(
        workspace_root: impl AsRef<Path>,
        log: &mut Logger,
    ) -> anyhow::Result<Option<Config>> {
        let path = workspace_root.as_ref().join(".cargo-for-rust-rover.toml");
        if !path.is_file() {
            log.log(format_args!(
                "config not found at {}, continuing without filtering",
                path.display()
            ));
            return Ok(None);
        }
        let text = fs::read_to_string(&path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        let config: Config =
            toml::from_str(&text).with_context(|| format!("failed to parse {}", path.display()))?;
        anyhow::ensure!(
            !config.package_prefixes.is_empty(),
            "{}: package_prefixes must be non-empty",
            path.display()
        );
        Ok(Some(config))
    }
}
