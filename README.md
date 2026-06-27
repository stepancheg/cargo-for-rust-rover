# cargo-for-rust-rover

## Problem

RustRover likes to type-check the whole workspace at once
(`cargo check --workspace`, and the same pattern for `build` /
`clippy`). In a big monorepo that also builds native code, that is
painfully slow when you only need a few crates. Tracked in the
JetBrains issue tracker as [RUST-19682][RUST-19682].

## Solution

A `cargo` / `rustc` wrapper. On workspace-wide commands it strips
`--workspace`, limits the run to packages matching configured name
prefixes with `-p`, and hands off to the real tools in
`~/.cargo/bin/`. Activity is written to
`~/.cargo-for-rust-rover.log`.

## Configuration

In RustRover, set **Settings → Rust → Toolchain location** to this
repo's `bin/` directory so the IDE uses the wrappers there.

In the Cargo workspace root, add `.cargo-for-rust-rover.toml`:

```toml
package_prefixes = ["my-app", "my-lib"]
```

Only packages whose names start with one of those prefixes get
`-p` on workspace-wide `check` / `build` / `clippy`.

## Related

- [cargo-subspace][CARGO-SUBSPACE] — related approach for large
  workspaces, aimed at rust-analyzer rather than RustRover.

[RUST-19682]: https://youtrack.jetbrains.com/issue/RUST-19682/RR-indexing-is-slow-on-large-workspaces
[CARGO-SUBSPACE]: https://github.com/ethowitz/cargo-subspace
