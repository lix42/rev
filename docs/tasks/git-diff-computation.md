# Git Diff Computation

## Goal

Given a git repo and a comparison mode (e.g., working tree vs HEAD, staged vs HEAD), produce a list of changed files with their diff hunks.

## Approach

- Use `git2` to compute diffs between tree/index/workdir based on the selected `DiffSource`.
- For each changed file, produce a `FileDiff` containing:
  - File path (relative to repo root)
  - Change type (added, modified, deleted, renamed)
  - List of `DiffHunk` with old/new line ranges and change lines
  - Diff stats (+lines, -lines)
- Support the comparison modes from design spec section 5.1:
  - Working tree vs index (`git diff`)
  - Working tree vs HEAD (`git diff HEAD`)
  - Index vs HEAD (`git diff --cached`)
  - Working tree vs arbitrary ref (`git diff <ref>`)

## Design

Core types in `src/diff/engine.rs`:

```rust
pub struct FileDiff {
    pub path: PathBuf,
    pub change_type: ChangeType,
    pub hunks: Vec<DiffHunk>,
    pub stats: DiffStats,
}

pub struct DiffHunk {
    pub old_start: usize,
    pub old_lines: usize,
    pub new_start: usize,
    pub new_lines: usize,
    pub lines: Vec<DiffLine>,
}

pub enum DiffLine {
    Context(String),
    Added(String),
    Removed(String),
}
```

Git diff functions in `src/git/diff.rs` (not `mod.rs` — per CLAUDE.md, mod.rs should only contain re-exports):

```rust
pub fn compute_diff(repo: &Repository, source: &DiffSource) -> Result<Vec<FileDiff>>;
```

## How to Verify

1. In a test repo with known changes, verify the diff output matches `git diff` output.
2. Unit tests for each comparison mode using a temporary git repo created in tests.
3. `cargo test diff` passes.

## Dependencies

- [git-repo-detection](git-repo-detection.md)
