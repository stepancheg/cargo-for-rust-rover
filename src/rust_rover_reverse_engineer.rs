use std::collections::HashSet;

/// If RustRover runs a workspace-wide compile (`check` / `build` / `clippy` with
/// `--workspace` and no `-p` / `--package`), returns patched cargo args limited to
/// `packages` (drops `--workspace`, filters `--features`, appends `-p` for each).
/// Otherwise returns `None`.
///
/// `args[0]` is argv0 (wrapper path); cargo subcommand starts at `args[1]`.
pub(crate) fn maybe_patch_workspace_to_packages(
    name: &str,
    args: &[String],
    packages: &[String],
) -> Option<Vec<String>> {
    if name != "cargo" {
        return None;
    }
    let cargo_args = args.get(1..)?;
    let subcommand = cargo_args.first().map(String::as_str)?;
    if !matches!(subcommand, "check" | "build" | "clippy") {
        return None;
    }
    let mut has_workspace = false;
    let mut has_package = false;
    let mut i = 1;
    while i < cargo_args.len() {
        let arg = cargo_args[i].as_str();
        match arg {
            "--workspace" => has_workspace = true,
            "-p" | "--package" => {
                has_package = true;
                i += 1;
            }
            s if s.starts_with("--package=") => has_package = true,
            _ => {}
        }
        i += 1;
    }
    if !has_workspace || has_package {
        return None;
    }
    Some(patch_workspace_to_packages(cargo_args, packages))
}

fn patch_workspace_to_packages(cargo_args: &[String], packages: &[String]) -> Vec<String> {
    let package_set: HashSet<&str> = packages.iter().map(String::as_str).collect();
    let mut out = Vec::with_capacity(cargo_args.len() + packages.len() * 2);
    let mut i = 0;
    while i < cargo_args.len() {
        let arg = cargo_args[i].as_str();
        if arg == "--workspace" {
            i += 1;
            continue;
        }
        if arg == "--features" || arg == "-F" {
            if let Some(feats) = cargo_args.get(i + 1) {
                let filtered = filter_features(feats, &package_set);
                if !filtered.is_empty() {
                    out.push(arg.to_owned());
                    out.push(filtered);
                }
                i += 2;
                continue;
            }
        }
        if let Some(rest) = arg.strip_prefix("--features=") {
            let filtered = filter_features(rest, &package_set);
            if !filtered.is_empty() {
                out.push(format!("--features={filtered}"));
            }
            i += 1;
            continue;
        }
        out.push(cargo_args[i].clone());
        i += 1;
    }
    for package in packages {
        out.push("-p".to_owned());
        out.push(package.clone());
    }
    out
}

/// Keeps only `package/feature` entries whose package is in `packages`.
fn filter_features(feats: &str, packages: &HashSet<&str>) -> String {
    feats
        .split(',')
        .filter(|f| {
            f.split_once('/')
                .map(|(pkg, _)| packages.contains(pkg))
                .unwrap_or(false)
        })
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use crate::rust_rover_reverse_engineer::maybe_patch_workspace_to_packages;

    /// Typical RustRover workspace `cargo check` flag layout.
    #[test]
    fn log_workspace_check_returns_patched_args() {
        let args: Vec<String> = [
            "/path/to/cargo-for-rust-rover",
            "check",
            "--color=always",
            "--message-format",
            "json-diagnostic-rendered-ansi",
            "--all-targets",
            "--workspace",
            "--features",
            "my-app/default,other-crate/default",
            "--keep-going",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        let packages = vec!["my-app".to_owned()];
        let patched = maybe_patch_workspace_to_packages("cargo", &args, &packages);
        let expected: Vec<String> = [
            "check",
            "--color=always",
            "--message-format",
            "json-diagnostic-rendered-ansi",
            "--all-targets",
            "--features",
            "my-app/default",
            "--keep-going",
            "-p",
            "my-app",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        assert_eq!(Some(expected), patched);
    }

    #[test]
    fn non_workspace_returns_none() {
        let args: Vec<String> = [
            "/path/to/cargo-for-rust-rover",
            "metadata",
            "--format-version",
            "1",
        ]
        .into_iter()
        .map(str::to_owned)
        .collect();
        assert_eq!(
            None,
            maybe_patch_workspace_to_packages("cargo", &args, &["my-app".to_owned()])
        );
    }
}
