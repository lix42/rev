# File Panel

## Goal

Implement the file panel (left sidebar) that lists all files with diffs, shows per-file diff stats, and allows navigation/selection.

## Approach

- Render a scrollable list of file paths with diff stats (e.g., `+12 -3`).
- Support `j`/`k` or arrow keys for navigation.
- Highlight the currently selected file.
- Selecting a file (Enter) updates the diff viewport to show that file's diff.
- Support flat list view initially; tree view (directory grouping) can be a follow-up.
- Show `●` marker next to files that have comments (wired up after comment system is built).

## How to Verify

1. Launch `rev` in a repo with changes — file panel lists all changed files.
2. Each file shows `+N -M` stats.
3. `j`/`k` moves selection up/down with visual highlight.
4. `Enter` on a file updates the center panel (even if it just prints the file name for now).
5. Panel scrolls when the list is longer than the viewport.

## Dependencies

- [three-panel-layout](three-panel-layout.md)
- [git-diff-computation](git-diff-computation.md)
