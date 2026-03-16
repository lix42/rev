# Help Overlay

## Goal

Implement a `?` help overlay showing all available keybindings for the current mode.

## Approach

- Press `?` → floating overlay listing all keybindings grouped by category.
- `Esc` or `?` again dismisses the overlay.
- Keybindings shown should reflect any custom overrides from config.

## How to Verify

1. Press `?` — help overlay appears.
2. All keybindings from design spec section 7.1 are listed.
3. `Esc` dismisses the overlay.
4. Custom keybindings are reflected in the help text.

## Dependencies

- [input-system](input-system.md)
