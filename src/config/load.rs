//! Reads and parses ~/.config/rev/config.toml into a Config struct.

use std::io::ErrorKind;
use std::path::PathBuf;

use anyhow::{Context, Result};

use super::types::Config;
use super::validate::validate;

/// Returns the path to the config file: `<config_dir>/rev/config.toml`.
fn config_path() -> Result<PathBuf> {
    let config_dir = dirs::config_dir().context("could not determine config directory")?;
    Ok(config_dir.join("rev").join("config.toml"))
}

/// Loads configuration from `~/.config/rev/config.toml`.
///
/// Returns defaults if the file does not exist. Returns an error if the file
/// exists but cannot be read (e.g., permission denied) or contains invalid values.
pub fn load_config() -> Result<Config> {
    let path = config_path()?;

    let contents = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) if e.kind() == ErrorKind::NotFound => return Ok(Config::default()),
        Err(e) => {
            return Err(anyhow::anyhow!(e).context(format!("failed to read {}", path.display())));
        }
    };

    load_from_str(&contents).with_context(|| format!("invalid config at {}", path.display()))
}

/// Parses and validates a config from a TOML string.
/// Useful for testing without touching the filesystem.
pub fn load_from_str(toml_str: &str) -> Result<Config> {
    let config: Config = toml::from_str(toml_str).context("failed to parse TOML")?;
    validate(&config)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::types::*;

    #[test]
    fn default_config_is_valid() {
        let config = Config::default();
        validate(&config).expect("default config should be valid");
    }

    #[test]
    fn empty_toml_gives_defaults() {
        let config = load_from_str("").unwrap();
        assert_eq!(config, Config::default());
    }

    #[test]
    fn partial_toml_merges_with_defaults() {
        let toml = r#"
            [appearance]
            tab_width = 2
            theme = "monokai"

            [git]
            default_base = "main"
        "#;
        let config = load_from_str(toml).unwrap();

        assert_eq!(config.appearance.tab_width, 2);
        assert_eq!(config.appearance.theme, "monokai");
        // Rest should be defaults
        assert!(config.appearance.side_by_side);
        assert!(config.appearance.line_numbers);
        assert_eq!(config.behavior.reload_debounce_ms, 200);
        assert_eq!(config.git.default_base, "main");
        assert_eq!(config.export.default_format, ExportFormat::Text);
    }

    #[test]
    fn full_toml_parses() {
        let toml = r#"
            [appearance]
            theme = "dark"
            side_by_side = false
            line_numbers = false
            word_diff = false
            tab_width = 8

            [behavior]
            auto_reload = false
            reload_debounce_ms = 500
            session_stale_days = 7

            [keybindings]
            scroll_down = "n"
            scroll_up = "p"
            next_hunk = "}"
            prev_hunk = "{"
            toggle_file_panel = "t"
            add_comment = "a"
            resolve_comment = "x"
            export = "w"
            enter_edit = "o"
            quit = "Q"

            [git]
            default_base = "develop"

            [export]
            default_format = "json"
            default_status = "all"
        "#;
        let config = load_from_str(toml).unwrap();

        assert_eq!(config.appearance.theme, "dark");
        assert!(!config.appearance.side_by_side);
        assert_eq!(config.appearance.tab_width, 8);
        assert!(!config.behavior.auto_reload);
        assert_eq!(config.behavior.reload_debounce_ms, 500);
        assert_eq!(config.behavior.session_stale_days, 7);
        assert_eq!(config.keybindings.scroll_down, "n");
        assert_eq!(config.git.default_base, "develop");
        assert_eq!(config.export.default_format, ExportFormat::Json);
        assert_eq!(config.export.default_status, ExportStatus::All);
    }

    #[test]
    fn invalid_tab_width_zero() {
        let toml = "[appearance]\ntab_width = 0";
        let err = load_from_str(toml).unwrap_err();
        assert!(err.to_string().contains("tab_width"));
    }

    #[test]
    fn invalid_debounce_too_low() {
        let toml = "[behavior]\nreload_debounce_ms = 1";
        let err = load_from_str(toml).unwrap_err();
        assert!(err.to_string().contains("reload_debounce_ms"));
    }

    #[test]
    fn invalid_debounce_too_high() {
        let toml = "[behavior]\nreload_debounce_ms = 10000";
        let err = load_from_str(toml).unwrap_err();
        assert!(err.to_string().contains("reload_debounce_ms"));
    }

    #[test]
    fn invalid_session_stale_days_zero() {
        let toml = "[behavior]\nsession_stale_days = 0";
        let err = load_from_str(toml).unwrap_err();
        assert!(err.to_string().contains("session_stale_days"));
    }

    #[test]
    fn invalid_export_format() {
        let toml = "[export]\ndefault_format = \"csv\"";
        let err = load_from_str(toml).unwrap_err();
        assert!(err.to_string().contains("TOML"), "got: {err}");
    }

    #[test]
    fn invalid_export_status() {
        let toml = "[export]\ndefault_status = \"archived\"";
        let err = load_from_str(toml).unwrap_err();
        assert!(err.to_string().contains("TOML"), "got: {err}");
    }

    #[test]
    fn all_valid_export_formats_accepted() {
        for fmt in &["text", "markdown", "json"] {
            let toml = format!("[export]\ndefault_format = \"{}\"", fmt);
            load_from_str(&toml).unwrap_or_else(|e| panic!("format '{fmt}' should be valid: {e}"));
        }
    }

    #[test]
    fn all_valid_export_statuses_accepted() {
        for status in &["open", "resolved", "updated", "all"] {
            let toml = format!("[export]\ndefault_status = \"{}\"", status);
            load_from_str(&toml)
                .unwrap_or_else(|e| panic!("status '{status}' should be valid: {e}"));
        }
    }

    #[test]
    fn invalid_tab_width_too_high() {
        let toml = "[appearance]\ntab_width = 33";
        let err = load_from_str(toml).unwrap_err();
        assert!(err.to_string().contains("tab_width"), "got: {err}");
    }

    #[test]
    fn debounce_at_boundaries_is_valid() {
        let lower = "[behavior]\nreload_debounce_ms = 10";
        load_from_str(lower).expect("debounce 10 should be valid");

        let upper = "[behavior]\nreload_debounce_ms = 5000";
        load_from_str(upper).expect("debounce 5000 should be valid");
    }

    #[test]
    fn debounce_just_outside_boundaries_is_invalid() {
        let below = "[behavior]\nreload_debounce_ms = 9";
        assert!(load_from_str(below).is_err());

        let above = "[behavior]\nreload_debounce_ms = 5001";
        assert!(load_from_str(above).is_err());
    }

    #[test]
    fn empty_keybinding_is_invalid() {
        let toml = "[keybindings]\nscroll_down = \"\"";
        let err = load_from_str(toml).unwrap_err();
        assert!(err.to_string().contains("scroll_down"), "got: {err}");
    }

    #[test]
    fn empty_git_default_base_is_invalid() {
        let toml = "[git]\ndefault_base = \"\"";
        let err = load_from_str(toml).unwrap_err();
        assert!(err.to_string().contains("default_base"), "got: {err}");
    }

    #[test]
    fn type_mismatch_gives_parse_error() {
        let toml = "[appearance]\ntab_width = \"not a number\"";
        assert!(load_from_str(toml).is_err());
    }

    #[test]
    fn unknown_keys_are_silently_ignored() {
        let toml = "[appearance]\ntypo_field = true";
        let config = load_from_str(toml).unwrap();
        assert_eq!(config.appearance, AppearanceConfig::default());
    }

    #[test]
    fn malformed_toml_gives_error() {
        let toml = "this is not valid toml [[[";
        let err = load_from_str(toml).unwrap_err();
        assert!(err.to_string().contains("TOML"));
    }
}
