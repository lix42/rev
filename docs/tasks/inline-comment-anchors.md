# Inline Comment Anchors in Diff View

## Goal

Display comment anchor markers (`●`/`○`) inline in the diff view next to lines that have comments.

## Approach

- When rendering a diff line, check if any comment is anchored to that line.
- Show `●` (open), `○` (resolved), or `◐` (updated) marker in a gutter column.
- Selecting a marker with keyboard navigates to the comment in the comment panel.
- The annotator module (`src/diff/annotator.rs`) overlays comment info onto diff lines.

## How to Verify

1. Add a comment on line 42 — a `●` appears next to line 42 in the diff view.
2. Resolve the comment — marker changes to `○`.
3. Multiple comments on different lines show multiple markers.
4. Markers are visible in both side-by-side and unified views.

## Dependencies

- [comment-crud](comment-crud.md)
- [side-by-side-diff-view](side-by-side-diff-view.md)
