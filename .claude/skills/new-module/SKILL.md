---
name: new-module
description: Scaffold a new Rust module following project code organization conventions
disable-model-invocation: true
---

# New Module

Scaffold a new Rust module directory following the project's code organization rules from CLAUDE.md.

## Arguments

- `$ARGUMENTS` — module path under `src/` (e.g., `src/search` or `search`)

## Instructions

1. Normalize the path: strip leading `src/` if present, then use `src/<name>/` as the directory.
2. Create the following files:

### `src/<name>/mod.rs`
Only re-exports and glue — no real logic:
```rust
mod types;

pub use types::*;
```

### `src/<name>/types.rs`
Data structs for the module with a TODO placeholder:
```rust
// TODO: Define data types for the <name> module
```

3. Add `mod <name>;` to `src/main.rs` (or the appropriate parent module).
4. Run `cargo check` to verify compilation.
5. Report what was created and suggest next steps.
