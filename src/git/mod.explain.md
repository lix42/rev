# Breakdown: `src/git/mod.rs`

```rust
//! Git integration (git2 crate).

mod repo;

pub use repo::{open_repo, RepoInfo};
```

This file is small but demonstrates several important Rust concepts. Let's go line by line.

---

## Line 1: `//! Git integration (git2 crate).`

This is a **module-level doc comment**. The `//!` (note the `!`) means "document the thing that *contains* me" — in this case, the `git` module itself. Compare with `///` which documents the thing *after* it (a function, struct, etc.).

If you ran `cargo doc`, this text would appear as the description of the `git` module.

---

## Line 3: `mod repo;`

This tells Rust: "there's a submodule called `repo`, and its code lives in `src/git/repo.rs`."

Rust's module system maps to the filesystem. Since this file is `src/git/mod.rs`, it's the "root" of the `git` module. When it says `mod repo;`, the compiler looks for one of:
- `src/git/repo.rs` (what we have)
- `src/git/repo/mod.rs` (alternative for nested modules)

Without `pub`, `repo` is **private** — code outside the `git` module can't directly access `git::repo::anything`. This is intentional: we control what's exposed via the next line.

---

## Line 5: `pub use repo::{open_repo, RepoInfo};`

This is a **re-export**. It does two things:

1. **`use repo::{open_repo, RepoInfo}`** — imports these two names from the private `repo` module into scope
2. **`pub`** — makes them publicly visible to anyone who uses the `git` module

The result: outside code writes `git::open_repo()` and `git::RepoInfo`, as if they were defined right here. They don't need to know that internally the code lives in `repo.rs`. This is the **facade pattern** — `mod.rs` is a thin "front door" that decides what the module's public API looks like.

The `{open_repo, RepoInfo}` syntax is a **use group** — a shorthand for importing multiple items from the same path in one statement.

---

## Why this pattern?

This file follows the project convention from CLAUDE.md: "Module files (`mod.rs`) should only contain re-exports and glue." The real logic lives in `repo.rs`. This keeps `mod.rs` as a table of contents — you can glance at it and immediately see the module's public API without scrolling through implementation code.

As the `git` module grows (e.g., adding diff computation), you'd add more submodules:

```rust
mod repo;
mod diff;  // future: src/git/diff.rs

pub use repo::{open_repo, RepoInfo};
pub use diff::compute_diff;  // future
```

---

## Key Rust concepts in this file

| Concept | Syntax | What it does |
|---|---|---|
| Module-level doc comment | `//!` | Documents the containing module |
| Item doc comment | `///` | Documents the next item (not used here, but contrast) |
| Module declaration | `mod repo;` | Declares a submodule, compiler looks for `repo.rs` |
| Visibility | `pub` vs no modifier | `pub` = public, no modifier = private (module-scoped) |
| Re-export | `pub use` | Makes an item from a submodule part of this module's public API |
| Use group | `{open_repo, RepoInfo}` | Import multiple items from one path in a single statement |
