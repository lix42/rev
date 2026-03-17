# File Watcher

## Goal

Watch source files for changes and trigger diff recomputation automatically, with debouncing for rapid saves.

## Approach

- Use `notify` crate with `notify-debouncer-mini` on a tokio task.
- Watch the repo working directory for file modifications.
- Batch events within the debounce window (default 200ms from config).
- Send batched `FileChanged` events via `tokio::sync::mpsc` to the main event loop.
- On receiving events, recompute diffs for affected files.
- Preserve scroll position and cursor location.
- Show a brief "Reloaded" indicator in the status bar.

## Design

```rust
// In src/watcher/mod.rs
pub async fn watch(
    repo_root: PathBuf,
    debounce_ms: u64,
    tx: mpsc::Sender<Vec<PathBuf>>,
) -> Result<()>;
```

The main event loop integrates this channel alongside crossterm events.

## How to Verify

1. Launch `rev`, edit a watched file externally — diff view updates automatically.
2. Rapid saves (e.g., agent writing multiple files) are batched into a single update.
3. Scroll position is preserved after reload.
4. Status bar shows "Reloaded" briefly.
5. Ignored files (`.git/`, build artifacts) don't trigger reloads.

## Dependencies

- [tui-app-skeleton](tui-app-skeleton.md)
- [git-diff-computation](git-diff-computation.md)
