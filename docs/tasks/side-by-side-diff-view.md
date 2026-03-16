# Side-by-Side Diff View

## Goal

Render a side-by-side diff view in the center panel with line numbers, aligned old/new content, and scrolling.

## Approach

- Split the center panel into two vertical halves (old on left, new on right).
- Show line numbers on each side.
- Align lines: context lines appear on both sides, added lines have blank on the left, removed lines have blank on the right.
- Color-code lines: green background for additions, red for removals, neutral for context.
- Support synchronized scrolling (`j`/`k`, `Ctrl+d`/`Ctrl+u` for page jumps).
- Hunk headers displayed as separator lines (e.g., `@@ -10,5 +12,7 @@`).

## Design

Transform `Vec<DiffHunk>` into a `Vec<DiffRow>` where each row has an optional left line and optional right line:

```rust
pub struct DiffRow {
    pub left: Option<(usize, String)>,   // (line_number, content)
    pub right: Option<(usize, String)>,
    pub kind: RowKind, // Context, Added, Removed, HunkHeader
}
```

## How to Verify

1. Select a file with changes — the diff viewport shows old and new side by side.
2. Line numbers are correct on both sides.
3. Added lines show only on the right; removed lines only on the left.
4. `j`/`k` scrolls both sides in sync.
5. `Ctrl+d`/`Ctrl+u` does page-down/up.
6. Hunk boundaries are visually clear.

## Dependencies

- [three-panel-layout](three-panel-layout.md)
- [git-diff-computation](git-diff-computation.md)
