//! Validates config values are within acceptable ranges.

use anyhow::{Result, bail};

use super::types::Config;

pub fn validate(config: &Config) -> Result<()> {
    if !(1..=32).contains(&config.appearance.tab_width) {
        bail!(
            "appearance.tab_width must be between 1 and 32, got {}",
            config.appearance.tab_width
        );
    }

    if !(10..=5000).contains(&config.behavior.reload_debounce_ms) {
        bail!(
            "behavior.reload_debounce_ms must be between 10 and 5000, got {}",
            config.behavior.reload_debounce_ms
        );
    }

    if config.behavior.session_stale_days == 0 {
        bail!(
            "behavior.session_stale_days must be greater than 0, got {}",
            config.behavior.session_stale_days
        );
    }

    if config.git.default_base.trim().is_empty() {
        bail!("git.default_base must not be empty");
    }

    let keybindings = [
        ("scroll_down", &config.keybindings.scroll_down),
        ("scroll_up", &config.keybindings.scroll_up),
        ("next_hunk", &config.keybindings.next_hunk),
        ("prev_hunk", &config.keybindings.prev_hunk),
        ("toggle_file_panel", &config.keybindings.toggle_file_panel),
        ("add_comment", &config.keybindings.add_comment),
        ("resolve_comment", &config.keybindings.resolve_comment),
        ("export", &config.keybindings.export),
        ("enter_edit", &config.keybindings.enter_edit),
        ("quit", &config.keybindings.quit),
    ];
    for (name, value) in keybindings {
        if value.trim().is_empty() {
            bail!("keybindings.{name} must not be empty");
        }
    }

    Ok(())
}
