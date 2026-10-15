//! What an app chooses about its theme. Plain data, no persistence: an app
//! that wants to remember the user's choices stores them however it likes
//! and builds a `ThemeSettings` from them at startup.

use crate::palette::material::MaterialSettings;
use crate::palette::{PalettePreset, RgbColor};

/// Corner rounding of controls. `Circle` is fully round (pill shapes).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CornerRadius {
    Disabled,
    #[default]
    Small,
    Medium,
    Large,
    Circle,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BorderWidth {
    Disabled,
    Thin,
    #[default]
    Default,
    Thick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AnimationSpeed {
    Slow,
    #[default]
    Normal,
    Fast,
}

/// The engine that determines how colors are resolved.
#[derive(Debug, Clone, PartialEq, Default)]
pub enum ThemeEngine {
    /// Classic preset scheme (e.g. "dark", "light", "tokyonight", "catppuccin-mocha")
    #[default]
    Preset,
    /// Material Design 3 Dynamic Color Scheme (via Material Color Utilities)
    Material(MaterialSettings),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThemeSettings {
    /// The theme engine to use. Defaults to `ThemeEngine::Preset`.
    pub engine: ThemeEngine,
    /// A palette variant id from [`crate::palette::presets::SCHEMES`], e.g.
    /// `"dark"`, `"light"`, `"catppuccin-mocha"`.
    pub variant: String,
    /// The accent color. `None` uses the variant's own signature accent.
    pub accent: Option<RgbColor>,
    /// A full base palette override.
    pub preset_override: Option<PalettePreset>,
    pub corner_radius: CornerRadius,
    pub border_width: BorderWidth,
    pub animation_speed: AnimationSpeed,
    pub animations_enabled: bool,
    /// Multiplies the spacing tokens only (not shapes or fonts).
    pub ui_scale: f32,
}

impl ThemeSettings {
    pub fn material(settings: MaterialSettings) -> Self {
        Self {
            engine: ThemeEngine::Material(settings),
            ..Default::default()
        }
    }

    pub fn preset(variant: impl Into<String>, accent: Option<RgbColor>) -> Self {
        Self {
            engine: ThemeEngine::Preset,
            variant: variant.into(),
            accent,
            preset_override: None,
            ..Default::default()
        }
    }
}

impl Default for ThemeSettings {
    fn default() -> Self {
        Self {
            engine: ThemeEngine::default(),
            variant: "dark".to_string(),
            accent: None,
            preset_override: None,
            corner_radius: CornerRadius::default(),
            border_width: BorderWidth::default(),
            animation_speed: AnimationSpeed::default(),
            animations_enabled: true,
            ui_scale: 1.0,
        }
    }
}
