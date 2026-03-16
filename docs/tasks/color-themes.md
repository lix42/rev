# Configurable Color Themes

## Goal

Support user-configurable color themes for the diff view and UI, shipping with sensible defaults.

## Approach

- Ship a default theme that is colorblind-friendly (design spec 6.1, 13.2).
- Allow users to specify a custom theme file path in `config.toml` (`appearance.theme`).
- Theme defines colors for: added lines, removed lines, context lines, syntax highlighting overrides, panel borders, status bar, comment markers.
- Theme file format: TOML (consistent with config).

## How to Verify

1. Default theme renders a readable, accessible diff.
2. Setting `theme = "path/to/custom.toml"` in config applies the custom colors.
3. Invalid theme file produces a clear error with fallback to default.
4. All UI elements respect theme colors.

## Dependencies

- [config-loading](config-loading.md)
- [syntax-highlighting](syntax-highlighting.md)
