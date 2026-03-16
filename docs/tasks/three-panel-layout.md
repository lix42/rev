# Three-Panel Layout

## Goal

Implement the three-panel layout (file panel, diff viewport, comment panel) with basic panel toggling and focus management.

## Approach

- Use Ratatui's `Layout` with horizontal splits to create three panels.
- Each panel is independently toggleable (design spec 3.1).
- When a panel is hidden, the remaining panels expand to fill the space.
- Track which panel is focused (active) for keyboard navigation.
- Render placeholder content in each panel (real content comes from later tasks).
- Status bar at the bottom shows mode indicator and keybinding hints.

## Design

Layout proportions (all visible): ~20% file panel, ~55% diff viewport, ~25% comment panel.

Panel visibility state in `AppState`:

```rust
pub show_file_panel: bool,    // toggle with Space
pub show_comment_panel: bool, // toggle with Ctrl+p
pub active_panel: Panel,
```

## How to Verify

1. `cargo run` shows three labeled panels side by side.
2. `Space` toggles the file panel — diff viewport expands.
3. `Ctrl+p` toggles the comment panel.
4. A key (TBD — `Tab` is reserved for switching diff pane sides per design spec 7.1) cycles focus between visible panels (highlighted border).
5. Status bar at bottom shows current keybinding hints.

## Dependencies

- [tui-app-skeleton](tui-app-skeleton.md)
