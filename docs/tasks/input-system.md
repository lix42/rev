# Input System & Modal Keybindings

## Goal

Implement the vi-inspired modal input system with configurable keybindings.

## Approach

- Define an `Action` enum in `src/input/actions.rs` that decouples actions from key codes.
- Define a `Mode` enum in `src/input/modal.rs`: Normal, Visual, Edit, Search.
- In `src/input/keymap.rs`, map key events to actions based on the current mode.
- Load custom keybindings from config, falling back to defaults.
- Support multi-key sequences (e.g., `]c` for next hunk).
- Status bar shows current mode.

## Design

```rust
pub enum Mode { Normal, Visual, Edit, Search }

pub enum Action {
    ScrollDown, ScrollUp, PageDown, PageUp,
    NextHunk, PrevHunk, NextFile, PrevFile,
    SwitchPanel, ToggleFilePanel, ToggleCommentPanel,
    AddComment, AddGlobalComment, ResolveComment,
    Export, EnterEditMode, EnterSearchMode,
    Quit, Help, // ...
}

pub fn resolve_key(mode: &Mode, key: KeyEvent, keymap: &Keymap) -> Option<Action>;
```

## How to Verify

1. All default keybindings from design spec section 7.1 work.
2. Multi-key sequences (`]c`, `[c`) are recognized.
3. Mode transitions work: Normal → Search (`/`), back to Normal (`Esc`).
4. Custom keybindings from config override defaults.
5. Status bar shows current mode name.

## Dependencies

- [config-loading](config-loading.md)
- [tui-app-skeleton](tui-app-skeleton.md)
