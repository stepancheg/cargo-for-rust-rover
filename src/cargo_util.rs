use std::process::Command;

use anyhow::Context;

/// Runs `cargo metadata --format-version=1` with `cargo_bin` (includes dependencies).
pub(crate) fn fetch_cargo_metadata(cargo_bin: &str) -> anyhow::Result<cargo_metadata::Metadata> {
    let output = Command::new(cargo_bin)
        .args(["metadata", "--format-version=1"])
        .output()
        .with_context(|| format!("failed to run {cargo_bin} metadata"))?;
    anyhow::ensure!(
        output.status.success(),
        "{cargo_bin} metadata failed with status {}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr),
    );
    serde_json::from_slice(&output.stdout).context("failed to parse cargo metadata json")
}
