# TUI App Skeleton

## Goal

Set up the Ratatui + Crossterm terminal application loop with basic event handling, so all subsequent UI tasks have a working shell to build on.

## Approach

- Initialize crossterm raw mode and alternate screen in `app::run()`.
- Set up the main event loop:
  - Poll for crossterm events (key, resize).
  - Dispatch events to a handler.
  - Render the UI each tick.
- Implement clean shutdown on `q` keypress or `Ctrl+C`.
- Restore terminal state on exit (including on panic, using a panic hook).
- Define a top-level `AppState` struct that holds all UI state.

## Design

In `src/app.rs`:

```rust
pub struct AppState {
    pub should_quit: bool,
    pub active_panel: Panel,
    // ... extended by later tasks
}

pub enum Panel {
    FilePanel,
    DiffView,
    CommentPanel,
}
```

The event loop structure:

```rust
loop {
    terminal.draw(|frame| ui::render(frame, &app_state))?;
    if crossterm::event::poll(tick_rate)? {
        let event = crossterm::event::read()?;
        handle_event(&mut app_state, event);
    }
    if app_state.should_quit { break; }
}
```

## How to Verify

1. `cargo run` launches a TUI that takes over the terminal.
2. Pressing `q` exits cleanly and restores the terminal.
3. `Ctrl+C` exits cleanly.
4. Terminal is restored even if the app panics (test by adding a temporary `panic!()` in the render loop).

## Dependencies

None — this is a foundational task.
