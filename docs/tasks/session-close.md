# Session Close & Archive

## Goal

Implement `rev close` to close/archive the current review session, with safeguards for unresolved comments.

## Approach

- `rev close` archives the current session (moves to `sessions/archive/` subdirectory).
- If open comments exist, warn the user and require confirmation.
- Archived sessions are not deleted — they can be inspected manually.
- Also accessible from within the TUI via a keybinding or command.

## How to Verify

1. `rev close` with all comments resolved → session is archived.
2. `rev close` with open comments → warning message, requires confirmation.
3. Archived session file exists in `sessions/archive/`.
4. Archived session no longer appears in the resume list.

## Dependencies

- [session-persistence](session-persistence.md)
- [comment-lifecycle](comment-lifecycle.md)
