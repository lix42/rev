# Config Loading

## Goal

Load user configuration from `~/.config/rx/config.toml`, providing sensible defaults when the file doesn't exist or fields are missing.

## Approach

- Define a `Config` struct matching design spec section 12.2.
- Use `dirs` crate for `~/.config/rx/` path.
- Use `toml` crate to parse.
- All fields optional with defaults (the tool should work with zero configuration).
- Validate values (e.g., `tab_width` must be > 0, `reload_debounce_ms` must be reasonable).

## Design

```rust
pub struct Config {
    pub appearance: AppearanceConfig,
    pub behavior: BehaviorConfig,
    pub keybindings: KeybindingConfig,
    pub git: GitConfig,
    pub export: ExportConfig,
}

pub fn load_config() -> Result<Config>;
```

## How to Verify

1. With no config file — app launches with defaults, no errors.
2. With a partial config — specified values are used, rest are defaults.
3. With an invalid config — clear error message pointing to the problem.
4. Unit tests for default values and merging.

## Dependencies

None — can be built independently.
