# Manual Re-anchor

## Goal

Allow users to manually move a drifted comment's anchor to the correct line using `M` key.

## Approach

- When a comment is marked `⚠ drifted`, user presses `M` to enter move mode.
- Diff view shows a visual indicator on the current anchor position.
- User navigates with `↑`/`↓` to the correct line.
- `Enter` confirms the new anchor, updating the comment's line and context.
- `Esc` cancels move mode.

## How to Verify

1. With a drifted comment, press `M` — visual move mode activates.
2. Arrow keys move the anchor indicator.
3. `Enter` saves the new position, clears the drifted warning.
4. `Esc` cancels without changes.
5. The comment's context is updated to reflect the new position.

## Dependencies

- [line-drift-resolver](line-drift-resolver.md)
