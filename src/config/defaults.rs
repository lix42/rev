//! Default values for all config structs, matching design spec section 12.2.

use super::types::*;

impl Default for Config {
    fn default() -> Self {
        Self {
            appearance: AppearanceConfig::default(),
            behavior: BehaviorConfig::default(),
            keybindings: KeybindingConfig::default(),
            git: GitConfig::default(),
            export: ExportConfig::default(),
        }
    }
}

impl Default for AppearanceConfig {
    fn default() -> Self {
        Self {
            theme: "default".into(),
            side_by_side: true,
            line_numbers: true,
            word_diff: true,
            tab_width: 4,
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            auto_reload: true,
            reload_debounce_ms: 200,
            session_stale_days: 30,
        }
    }
}

impl Default for KeybindingConfig {
    fn default() -> Self {
        Self {
            scroll_down: "j".into(),
            scroll_up: "k".into(),
            next_hunk: "]c".into(),
            prev_hunk: "[c".into(),
            toggle_file_panel: "space".into(),
            add_comment: "c".into(),
            resolve_comment: "r".into(),
            export: "e".into(),
            enter_edit: "i".into(),
            quit: "q".into(),
        }
    }
}

impl Default for GitConfig {
    fn default() -> Self {
        Self {
            default_base: "HEAD".into(),
        }
    }
}

impl Default for ExportConfig {
    fn default() -> Self {
        Self {
            default_format: ExportFormat::Text,
            default_status: ExportStatus::Open,
        }
    }
}
