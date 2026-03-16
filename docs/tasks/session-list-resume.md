# Session List & Resume

## Goal

Allow users to list existing review sessions for the current repo and resume a previous session.

## Approach

- Scan `~/.local/share/rx/sessions/` for session files matching the current repo root.
- Display sessions in the mode selector: branch name, creation date, comment count (open/resolved).
- User presses `r` in mode selector → session list appears.
- Selecting a session loads it and enters the review view with restored comments.
- If no sessions exist, show a message ("No existing sessions").

## How to Verify

1. Create comments, quit `rx`. Relaunch → press `r` → session is listed.
2. Session shows branch, date, and comment count.
3. Selecting it restores all comments in the diff view.
4. With no sessions, a clear message is shown.

## Dependencies

- [session-persistence](session-persistence.md)
- [mode-selector-ui](mode-selector-ui.md)
