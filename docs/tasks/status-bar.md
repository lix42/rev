# Status Bar

## Goal

Implement the status bar at the bottom of the screen showing mode, keybinding hints, search state, and transient messages.

## Approach

- Always visible at the bottom of the terminal.
- Left side: current mode (NORMAL, SEARCH, EDIT, etc.).
- Center: contextual keybinding hints for the current mode.
- Right side: transient messages (e.g., "Reloaded", "Comment saved", "Exported 5 comments").
- Transient messages auto-clear after a few seconds.

## How to Verify

1. Status bar shows "NORMAL" mode on startup.
2. Entering search mode shows "SEARCH" and the search input.
3. Saving a comment shows a "Comment saved" message briefly.
4. Keybinding hints update based on current context.

## Dependencies

- [tui-app-skeleton](tui-app-skeleton.md)
