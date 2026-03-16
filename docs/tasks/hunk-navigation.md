# Hunk Navigation

## Goal

Allow quick navigation between diff hunks using `]c` / `[c` (vim-compatible) and between files using `]` / `[`.

## Approach

- Track the current scroll position and the list of hunk start positions for the current file.
- `]c` jumps to the next hunk, `[c` to the previous.
- `]` moves to the next file in the file panel and loads its diff, `[` moves to the previous.
- Wrap around or stop at boundaries (configurable, stop by default).

## How to Verify

1. In a file with multiple hunks, `]c` jumps forward to each hunk start.
2. `[c` jumps backward.
3. At the last hunk, `]c` does nothing (no wrap).
4. `]` switches to the next file and shows its diff.
5. `[` switches to the previous file.

## Dependencies

- [side-by-side-diff-view](side-by-side-diff-view.md)
- [file-panel](file-panel.md)
