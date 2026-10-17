//! Load/save application-wide settings under the OS-level per-user config directory
//! (`$XDG_CONFIG_HOME/<organization>/app.toml`, e.g. `~/.config/mumeum/app.toml`).
//!
//! Kept separate from vault-scoped settings so that UI appearance and behavior (theme, font,
//! status bar, editor appearance, network permission, etc.) persist across all vaults
//! and can be configured even when no vault is currently open.

use std::cell::RefCell;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

thread_local! {
    static APP_PATH_OVERRIDE: RefCell<Option<PathBuf>> = const { RefCell::new(None) };
}

/// Sets an override path for the calling thread, useful for isolated unit testing.
pub fn set_app_path_for_thread(path: Option<PathBuf>) {
    APP_PATH_OVERRIDE.with(|p| *p.borrow_mut() = path);
}

/// Backwards-compatibility alias for tests and older callers.
pub fn set_preferences_path_for_thread(path: Option<PathBuf>) {
    set_app_path_for_thread(path);
}

/// Where `AppSettings` is persisted, or `None` if the platform's config
/// directory couldn't be resolved.
pub fn app_config_path() -> Option<PathBuf> {
    if let Some(p) = APP_PATH_OVERRIDE.with(|p| p.borrow().clone()) {
        return Some(p);
    }
    let id = crate::identity::app_identity();
    let dirs = ProjectDirs::from(&id.qualifier, &id.organization, &id.application)?;
    let app_toml = dirs.config_dir().join("app.toml");
    let legacy_preferences = dirs.config_dir().join("preferences.toml");

    if !app_toml.exists() && legacy_preferences.exists() {
        Some(legacy_preferences)
    } else {
        Some(app_toml)
    }
}

/// Backwards-compatibility alias.
pub fn preferences_path() -> Option<PathBuf> {
    app_config_path()
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accent_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub corner_radius: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_width: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_scale: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animations_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub animation_speed: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_font_family: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_font_point_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_allowed: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_bar_visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_bar_refresh_ms: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_font_point_size: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editor_show_line_numbers: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explorer_alternating_row_colors: Option<bool>,
}

/// Backwards-compatibility alias.
pub type UserPreferences = AppSettings;

pub fn load() -> AppSettings {
    match app_config_path() {
        Some(path) => load_from(&path),
        None => AppSettings::default(),
    }
}

pub fn load_from(path: &Path) -> AppSettings {
    match fs::read_to_string(path) {
        Ok(content) => toml::from_str(&content).unwrap_or_default(),
        Err(_) => AppSettings::default(),
    }
}

pub fn save(settings: &AppSettings) -> io::Result<()> {
    // When saving, prefer writing to app.toml even if preferences.toml was previously loaded
    let path = if let Some(p) = APP_PATH_OVERRIDE.with(|p| p.borrow().clone()) {
        p
    } else {
        let id = crate::identity::app_identity();
        let dirs = ProjectDirs::from(&id.qualifier, &id.organization, &id.application).ok_or_else(
            || {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "could not resolve user config directory",
                )
            },
        )?;
        dirs.config_dir().join("app.toml")
    };
    save_to(&path, settings)
}

pub fn save_to(path: &Path, settings: &AppSettings) -> io::Result<()> {
    let toml = toml::to_string_pretty(settings)
        .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, toml)
}

pub fn update<R>(f: impl FnOnce(&mut AppSettings) -> R) -> io::Result<R> {
    let mut settings = load();
    let res = f(&mut settings);
    save(&settings)?;
    Ok(res)
}

pub fn update_at<R>(path: &Path, f: impl FnOnce(&mut AppSettings) -> R) -> io::Result<R> {
    let mut settings = load_from(path);
    let res = f(&mut settings);
    save_to(path, &settings)?;
    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_dir(label: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "origiri-config-app-test-{label}-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let _ = fs::create_dir_all(&dir);
        dir
    }

    #[test]
    fn app_config_path_ends_with_app_toml() {
        let p = app_config_path().unwrap();
        assert!(p.ends_with("app.toml") || p.ends_with("preferences.toml"));
    }

    #[test]
    fn round_trips_app_settings() {
        let dir = temp_dir("roundtrip");
        let path = dir.join("app.toml");

        let mut settings = AppSettings::default();
        settings.theme_mode = Some("dark".to_string());
        settings.accent_color = Some("#123456".to_string());
        settings.corner_radius = Some("small".to_string());
        settings.border_width = Some("thick".to_string());
        settings.ui_scale = Some(1.2);
        settings.animations_enabled = Some(true);
        settings.animation_speed = Some("fast".to_string());
        settings.ui_font_family = Some("Noto Sans JP".to_string());
        settings.ui_font_point_size = Some(14.0);
        settings.network_allowed = Some(true);
        settings.status_bar_visible = Some(false);
        settings.status_bar_refresh_ms = Some(500);
        settings.editor_font_point_size = Some(12.5);
        settings.editor_show_line_numbers = Some(true);
        settings.explorer_alternating_row_colors = Some(false);

        save_to(&path, &settings).unwrap();
        let loaded = load_from(&path);
        assert_eq!(loaded, settings);

        let _ = fs::remove_dir_all(&dir);
    }
}
