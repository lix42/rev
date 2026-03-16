# Git Repo Detection

## Goal

Detect whether the current working directory is inside a git repository. If yes, extract repo root, current branch, and HEAD commit. If no, display a friendly error message and exit.

## Approach

- Use the `git2` crate to open the repository from the current directory (`Repository::discover`).
- Canonicalize the repo root path (resolve symlinks, collapse `.`/`..`) for consistent session identity.
- Extract current branch name (or `None` if detached HEAD).
- Extract HEAD commit SHA.
- On failure (not a git repo), print the friendly error from design spec section 5.4 and exit with a non-zero code.

## Design

Define a `RepoInfo` struct in `src/git/mod.rs`:

```rust
pub struct RepoInfo {
    pub repo_root: PathBuf,   // canonicalized
    pub head_sha: String,
    pub branch: Option<String>,
}

pub fn open_repo(path: &Path) -> Result<RepoInfo>;
```

## How to Verify

1. Run `cargo test` — unit test that `open_repo` returns correct info for the test repo itself (the `rx` project).
2. Run `rx` inside the `rx` repo — should not error.
3. Run `rx` in `/tmp` (no git repo) — should print the friendly error and exit cleanly.

## Dependencies

None — this is a foundational task.
