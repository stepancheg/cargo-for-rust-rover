# Agents

## Rust code style

- Use `pub(crate)` for items shared within the crate; prefer private
  (no `pub`) over `pub(crate)` where possible
- Use `"{x}"` instead of `"{}"` for format strings where `x` is the
  variable name
- Do not use `map_or`, use `map(..).unwrap_or(..)` instead
- When deriving `Display`, use qualified name (e.g.,
  `derive_more::Display`)
- Do not use `super` keyword; use imports (`use crate::...`) instead
- In `assert_eq!`, expected value is LHS (e.g.,
  `assert_eq!(expected, actual)`)
- Do not use re-exports (`pub use`); import items from their defining
  module path instead
- Do not use `.parse()`; use `T::from_str(..)` instead
- Do not use `str::to_string`; use `str::to_owned` instead
- Place imports at the module top, not inside functions (except imports
  needed for `cfg-if`)
- End comments (including doc comments) with a trailing dot
- Do not build structs with `Self { ... }`; name the type explicitly
  (e.g. `Logger { ... }`)

## Other

- `cargo fmt` is `cargo +nightly fmt`
- Do not build; only run `cargo check`
- No more than 80 characters per line in Markdown files
