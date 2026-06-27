mod cargo_util;
mod cfg;
mod logger;
mod rust_rover_reverse_engineer;

use std::env;
use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::Command;

use anyhow::Context;

use crate::cfg::Config;
use crate::logger::Logger;

fn extract_packages_by_prefixes(
    metadata: &cargo_metadata::Metadata,
    prefixes: &[String],
) -> Vec<String> {
    let mut packages: Vec<String> = metadata
        .packages
        .iter()
        .map(|p| p.name.to_string())
        .filter(|n| prefixes.iter().any(|p| n.starts_with(p.as_str())))
        .collect();
    packages.sort();
    packages
}

fn maybe_overwrite_rust_rover_cargo_check_workspace(
    name: &str,
    args: &[String],
    cargo_bin: &str,
    cargo_args: &[String],
    log: &mut Logger,
) -> anyhow::Result<Vec<String>> {
    // Cheap detect (no packages needed when not RR workspace compile).
    if rust_rover_reverse_engineer::maybe_patch_workspace_to_packages(name, args, &[]).is_none() {
        return Ok(cargo_args.to_vec());
    }
    let metadata = cargo_util::fetch_cargo_metadata(cargo_bin)?;
    let Some(config) = Config::load(&metadata.workspace_root, log)? else {
        return Ok(cargo_args.to_vec());
    };
    let packages = extract_packages_by_prefixes(&metadata, &config.package_prefixes);
    anyhow::ensure!(
        !packages.is_empty(),
        "no packages matching package_prefixes {:?} in cargo metadata",
        config.package_prefixes
    );
    let patched =
        rust_rover_reverse_engineer::maybe_patch_workspace_to_packages(name, args, &packages)
            .expect("RR workspace compile already detected");
    let patched_quoted = shlex::try_join(patched.iter().map(String::as_str))
        .context("failed to shlex-quote patched args")?;
    log.log(format_args!(
        "patched package_prefixes={:?} args={patched_quoted}",
        config.package_prefixes
    ));
    Ok(patched)
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let argv0 =
        env::var("CARGO_FOR_RUST_ROVER_ARGV0").context("CARGO_FOR_RUST_ROVER_ARGV0 is not set")?;
    anyhow::ensure!(!argv0.is_empty(), "CARGO_FOR_RUST_ROVER_ARGV0 is empty");
    let name = Path::new(&argv0)
        .file_name()
        .and_then(|s| s.to_str())
        .context("CARGO_FOR_RUST_ROVER_ARGV0 has no file name")?
        .to_owned();
    anyhow::ensure!(
        !name.is_empty(),
        "CARGO_FOR_RUST_ROVER_ARGV0 file name is empty"
    );
    let home = env::var("HOME").context("HOME is not set")?;
    let log_path = format!("{home}/.cargo-for-rust-rover.log");
    let mut log = Logger::open(&log_path);
    let args_quoted =
        shlex::try_join(args.iter().map(String::as_str)).context("failed to shlex-quote args")?;
    log.log(format_args!("name={name} args={args_quoted}"));
    let target = format!("{home}/.cargo/bin/{name}");
    let cargo_args = args.get(1..).unwrap_or(&[]).to_vec();
    let cargo_args = maybe_overwrite_rust_rover_cargo_check_workspace(
        &name,
        &args,
        &target,
        &cargo_args,
        &mut log,
    )?;
    let err = Command::new(&target).arg0(&name).args(&cargo_args).exec();
    Err(err).with_context(|| format!("failed to exec {target}"))
}
