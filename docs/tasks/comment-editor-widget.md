# Comment Editor Widget

## Goal

Implement a floating overlay widget for writing and editing comments, with basic multiline text editing.

## Approach

- Render as a floating box centered over the diff viewport.
- Support multiline text input: `Enter` inserts a newline, `Ctrl+Enter` saves.
- `Esc` cancels (with confirmation if text was modified).
- Basic editing: character insert, backspace, delete, arrow key cursor movement.
- Show the file and line number being commented on in the editor header.
- When editing an existing comment, pre-populate the text.

## How to Verify

1. Pressing `c` on a diff line opens the floating editor.
2. Can type multiline text.
3. `Enter` creates a new line (does not submit).
4. `Ctrl+Enter` saves and closes the editor.
5. `Esc` cancels — if text was entered, a confirmation prompt appears.
6. Editing an existing comment shows the existing text.

## Dependencies

- [three-panel-layout](three-panel-layout.md)
- [input-system](input-system.md)
