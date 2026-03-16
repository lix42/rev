# Comment CRUD

## Goal

Implement creating, reading, updating, and deleting comments (inline and global) within a review session.

## Approach

- Inline comment: user presses `c` on a diff line → opens comment editor → saves a `Comment` with `CommentKind::Inline` and an `Anchor` capturing the file, line, and 5-line context.
- Global comment: user presses `C` → opens comment editor → saves a `Comment` with `CommentKind::Global`.
- Edit existing comment: navigate to it, press `c` → re-opens editor with existing text.
- Status transitions: `r` on a comment marks as Resolved (see [comment-lifecycle](comment-lifecycle.md) for full lifecycle).
- All changes are persisted to the session JSON file.

> **Note:** The design spec defines `file` and `line` on both `CommentKind::Inline` and `Anchor`. During implementation, reconcile this duplication — prefer having `Inline` hold only an `Anchor` (which contains `file` and `line`), avoiding two sources of truth that can drift apart.

## Design

Functions in `src/review/session.rs`:

```rust
pub fn add_comment(session: &mut ReviewSession, comment: Comment) -> Result<()>;
pub fn update_comment(session: &mut ReviewSession, id: Uuid, body: String) -> Result<()>;
pub fn resolve_comment(session: &mut ReviewSession, id: Uuid) -> Result<()>;
```

## How to Verify

1. Press `c` on a diff line → comment editor appears.
2. Type text, press `Ctrl+Enter` → comment is saved and visible in comment panel.
3. Navigate to comment, press `c` → editor opens with existing text, can edit and save.
4. Press `r` on a comment → status changes from Open to Resolved.
5. Session file on disk reflects all changes.

## Dependencies

- [comment-editor-widget](comment-editor-widget.md)
- [session-persistence](session-persistence.md)
