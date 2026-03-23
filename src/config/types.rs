//! Config structs matching design spec section 12.2.

use serde::Deserialize;

/// Top-level application configuration.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct Config {
    pub appearance: AppearanceConfig,
    pub behavior: BehaviorConfig,
    pub keybindings: KeybindingConfig,
    pub git: GitConfig,
    pub export: ExportConfig,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct AppearanceConfig {
    pub theme: String,
    pub side_by_side: bool,
    pub line_numbers: bool,
    pub word_diff: bool,
    pub tab_width: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct BehaviorConfig {
    pub auto_reload: bool,
    pub reload_debounce_ms: u64,
    pub session_stale_days: u32,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct KeybindingConfig {
    pub scroll_down: String,
    pub scroll_up: String,
    pub next_hunk: String,
    pub prev_hunk: String,
    pub toggle_file_panel: String,
    pub add_comment: String,
    pub resolve_comment: String,
    pub export: String,
    pub enter_edit: String,
    pub quit: String,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct GitConfig {
    pub default_base: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportFormat {
    Text,
    Markdown,
    Json,
}

#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportStatus {
    Open,
    Resolved,
    Updated,
    All,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct ExportConfig {
    pub default_format: ExportFormat,
    pub default_status: ExportStatus,
}
