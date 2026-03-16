# Session Persistence

## Goal

Persist review sessions as JSON files so comments survive across `rx` launches.

## Approach

- Store sessions at `~/.local/share/rx/sessions/<session_hash>.json`.
- Session hash = `sha256(canonical_repo_root + base_sha + branch_name)` (design spec 9.1; note: section 4.6 says `base_ref` but 9.1 clarifies refs are resolved to concrete SHAs — use `base_sha`).
- On launch, check if a session exists for the current context and load it.
- Auto-save after every comment change (debounced to avoid excessive writes).
- Use `dirs` crate for `~/.local/share/rx/` path.
- Create the directory structure if it doesn't exist.

## Design

```rust
pub fn session_path(repo_root: &Path, base_sha: &str, branch: Option<&str>) -> PathBuf;
pub fn load_session(path: &Path) -> Result<Option<ReviewSession>>;
pub fn save_session(session: &ReviewSession) -> Result<()>;
```

## How to Verify

1. Start `rx`, add a comment, quit. Restart `rx` — the comment is still there.
2. Session file exists at the expected path with valid JSON.
3. Session file can be manually inspected and is human-readable.
4. Different branches create different session files.
5. Accessing the same repo via a symlink resolves to the same session.

## Dependencies

- [git-repo-detection](git-repo-detection.md)
