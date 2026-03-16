# Comment Panel

## Goal

Implement the right sidebar that lists all comments for the current file with status indicators and navigation.

## Approach

- Display comments as a scrollable list: line number, truncated body, status icon (`●` open, `○` resolved).
- Navigate with `j`/`k` when the comment panel is focused.
- Selecting a comment scrolls the diff viewport to the anchored line.
- Support a "global" view toggle showing comments across all files.
- Show global comments in a separate section.

## How to Verify

1. After adding comments, they appear in the comment panel.
2. `●` shown for open comments, `○` for resolved.
3. Selecting a comment scrolls the diff view to that line.
4. Global comments appear in a separate section.
5. Panel updates when comments are added/resolved/deleted.

## Dependencies

- [comment-crud](comment-crud.md)
- [three-panel-layout](three-panel-layout.md)
