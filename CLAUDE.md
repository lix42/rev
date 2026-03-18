# CLAUDE.md

## Rules

### Use verified docs, not memory
For any crate or library, always fetch current docs via Context7 rather than relying on training data. Rust ecosystem moves fast — don't guess at APIs.

### Think critically
Don't rubber-stamp plans or reasoning — push back with a short explanation when something seems off.

### Plan before acting
For meaningful changes (not typos/formatting), propose what you'll change and why first. Follow the plan once agreed.

### Use latest stable dependencies
When adding or updating crates, use the latest stable version. Check crates.io if unsure. Don't pin to old versions without a reason.

### Code organization
- Prefer free functions over methods when the function doesn't need `self`.
- Keep structs as data + minimal orchestration. Extract logic into standalone functions in dedicated modules.
- Module files (`mod.rs`) should only contain re-exports and glue — put real logic in named files.
- `types.rs` or the module's data structs go at the top of the module; logic lives alongside in separate files.

### Error handling
Use `anyhow::Result` for application-level errors. Use `thiserror` for library-style errors that cross module boundaries and need matching. Don't `unwrap()` in non-test code.

### Unsafe code
No `unsafe` unless absolutely necessary and justified in a comment.

## Project Overview

`rev` is a TUI-native code review tool for AI agent coding workflows. It renders diffs with syntax highlighting and provides a structured annotation layer (inline and global comments) that can be exported in a format AI agents understand.

See `docs/design-spec.md` for the full design specification.

## Architecture

```
src/
├── main.rs           # CLI entry point (clap)
├── app.rs            # Top-level state machine, event loop
├── diff/             # Pure diff logic (similar, syntect, word-level diffs)
├── review/           # Session CRUD, comment model, export, line-drift resolver
├── watcher/          # File system watcher (notify crate, tokio task)
├── ui/               # Ratatui widgets (file panel, diff view, comment panel, etc.)
├── input/            # Keyboard handling, keymaps, vi-style modal system
├── git/              # Git integration (git2 crate)
├── mcp/              # MCP server (Phase 5)
└── config/           # Config loading (~/.config/rev/config.toml)
```

## Tech Stack

- **Language:** Rust (edition 2021)
- **TUI:** Ratatui + Crossterm
- **Diff:** similar (line + word level)
- **Syntax highlighting:** syntect
- **File watching:** notify
- **Async:** Tokio
- **Git:** git2 (libgit2)
- **CLI:** clap (derive)
- **Serialization:** serde + serde_json
- **Config:** toml + dirs
- **IDs:** uuid (v4)
- **Time:** chrono

## Commands

```bash
# System dependencies (macOS)
# brew install cmake pkg-config libgit2 oniguruma

# Check compilation
cargo check

# Run
cargo run

# Run with subcommand
cargo run -- export --format text --status open

# Test
cargo test

# Test a specific module
# cargo test review::comment

# Lint
cargo clippy -- -D warnings

# Format
cargo fmt
```

## CI & Hooks

- **Pre-commit hook:** `.githooks/pre-commit` runs `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` before each commit.
- **GitHub Actions:** `.github/workflows/ci.yml` runs the same checks on push/PR to `main`.
- **Activate hooks locally:** `git config core.hooksPath .githooks`

## Gotchas

- **System C deps required**: `git2` needs `libgit2`/`cmake`, `syntect` needs `oniguruma`. Install via `brew install cmake pkg-config libgit2 oniguruma` on macOS.
- **Session storage**: Review sessions persist as JSON at `~/.local/share/rev/sessions/`.
- **Config location**: User config lives at `~/.config/rev/config.toml`.
- **syntect grammars**: Bundled at compile time — adding language support means rebuilding.
