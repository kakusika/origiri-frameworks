//! Material Design 3 (M3) Dynamic Color calculation using Material Color Utilities.

use mcu_material_color::prelude::*;

use crate::palette::RgbColor;
use crate::tokens::{ColorTokens, GroundTokens, Rgba};

/// The Material Design 3 scheme style / variant.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaterialVariant {
    #[default]
    TonalSpot,
    Vivid,
    Expressive,
    FruitSalad,
    Rainbow,
    Spritz,
    Monochrome,
    Content,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialSettings {
    pub seed: RgbColor,
    pub variant: MaterialVariant,
    pub is_dark: bool,
    /// Contrast level: -1.0 (reduced), 0.0 (standard default), 0.5 (medium), 1.0 (high).
    pub contrast: f64,
}

impl Default for MaterialSettings {
    fn default() -> Self {
        Self {
            seed: RgbColor::new(0x67, 0x50, 0xa4), // M3 baseline purple
            variant: MaterialVariant::TonalSpot,
            is_dark: true,
            contrast: 0.0,
        }
    }
}

pub fn create_dynamic_scheme(
    hct: Hct,
    variant: MaterialVariant,
    is_dark: bool,
    contrast: f64,
) -> DynamicScheme {
    match variant {
        MaterialVariant::TonalSpot => (*SchemeTonalSpot::new(hct, is_dark, contrast)).clone(),
        MaterialVariant::Vivid => (*SchemeVibrant::new(hct, is_dark, contrast)).clone(),
        MaterialVariant::Expressive => (*SchemeExpressive::new(hct, is_dark, contrast)).clone(),
        MaterialVariant::FruitSalad => (*SchemeFruitSalad::new(hct, is_dark, contrast)).clone(),
        MaterialVariant::Rainbow => (*SchemeRainbow::new(hct, is_dark, contrast)).clone(),
        MaterialVariant::Spritz => (*SchemeNeutral::new(hct, is_dark, contrast)).clone(),
        MaterialVariant::Monochrome => (*SchemeMonochrome::new(hct, is_dark, contrast)).clone(),
        MaterialVariant::Content => (*SchemeContent::new(hct, is_dark, contrast)).clone(),
    }
}

pub fn argb_to_rgba(argb: u32) -> Rgba {
    let a = ((argb >> 24) & 0xff) as u8;
    let r = ((argb >> 16) & 0xff) as u8;
    let g = ((argb >> 8) & 0xff) as u8;
    let b = (argb & 0xff) as u8;
    Rgba { r, g, b, a }
}

pub fn rgba_to_rgb(rgba: Rgba) -> RgbColor {
    RgbColor::new(rgba.r, rgba.g, rgba.b)
}

/// Resolves a full set of `ColorTokens` using Material Design 3 dynamic color algorithms.
pub fn resolve_material_colors(
    settings: &MaterialSettings,
    ground_tokens_fn: impl Fn(RgbColor, RgbColor, RgbColor) -> GroundTokens,
) -> ColorTokens {
    let seed_argb = 0xff000000
        | ((settings.seed.r as u32) << 16)
        | ((settings.seed.g as u32) << 8)
        | (settings.seed.b as u32);
    let hct = Hct::from_int(seed_argb);
    let scheme = create_dynamic_scheme(hct, settings.variant, settings.is_dark, settings.contrast);

    let get = |color: DynamicColor| argb_to_rgba(color.get_argb(&scheme));

    let primary = get(MaterialDynamicColors::primary());
    let on_primary = get(MaterialDynamicColors::on_primary());
    let primary_container = get(MaterialDynamicColors::primary_container());
    let on_primary_container = get(MaterialDynamicColors::on_primary_container());

    let secondary = get(MaterialDynamicColors::secondary());
    let on_secondary = get(MaterialDynamicColors::on_secondary());
    let secondary_container = get(MaterialDynamicColors::secondary_container());
    let on_secondary_container = get(MaterialDynamicColors::on_secondary_container());

    let tertiary = get(MaterialDynamicColors::tertiary());
    let on_tertiary = get(MaterialDynamicColors::on_tertiary());
    let tertiary_container = get(MaterialDynamicColors::tertiary_container());
    let on_tertiary_container = get(MaterialDynamicColors::on_tertiary_container());

    let error = get(MaterialDynamicColors::error());
    let on_error = get(MaterialDynamicColors::on_error());
    let error_container = get(MaterialDynamicColors::error_container());
    let on_error_container = get(MaterialDynamicColors::on_error_container());

    let m3_surface = get(MaterialDynamicColors::surface());
    let on_surface = get(MaterialDynamicColors::on_surface());
    let on_surface_variant = get(MaterialDynamicColors::on_surface_variant());

    let surface_container_lowest = get(MaterialDynamicColors::surface_container_lowest());
    let surface_container_low = get(MaterialDynamicColors::surface_container_low());
    let surface_container = get(MaterialDynamicColors::surface_container());
    let surface_container_high = get(MaterialDynamicColors::surface_container_high());
    let surface_container_highest = get(MaterialDynamicColors::surface_container_highest());

    let outline = get(MaterialDynamicColors::outline());
    let outline_variant = get(MaterialDynamicColors::outline_variant());

    // Success color (M3 palette key or harmonized green)
    let success = Rgba::opaque(RgbColor::new(0x2e, 0x7d, 0x32));

    // Ground tokens for traditional surfaces:
    // - `background`: M3 `surface` / `background` (Tone 98 light / Tone 6 dark)
    // - `surface`: M3 `surface-container` (Tone 94 light / Tone 12 dark)
    // - `surface_raised`: M3 `surface-container-high` (Tone 92 light / Tone 17 dark)
    let background_rgb = rgba_to_rgb(m3_surface);
    let surface_rgb = rgba_to_rgb(surface_container);
    let on_surface_rgb = rgba_to_rgb(on_surface);
    let primary_rgb = rgba_to_rgb(primary);
    let high_rgb = rgba_to_rgb(surface_container_high);

    ColorTokens {
        // M3 Semantic Roles
        primary,
        on_primary,
        primary_container,
        on_primary_container,

        secondary,
        on_secondary,
        secondary_container,
        on_secondary_container,

        tertiary,
        on_tertiary,
        tertiary_container,
        on_tertiary_container,

        error,
        on_error,
        error_container,
        on_error_container,

        surface: surface_container,
        on_surface_variant,

        surface_container_lowest,
        surface_container_low,
        surface_container,
        surface_container_high,
        surface_container_highest,

        outline,
        outline_variant,

        // Aliases / Compatibility roles
        background: m3_surface,
        surface_raised: surface_container_high,
        accent: primary,
        text_on_accent: on_primary,
        text_on_destructive: on_error,
        text_primary: on_surface,
        destructive: error,
        accent_hover: primary,
        accent_pressed: primary,
        destructive_hover: error,
        destructive_pressed: error,
        success,
        on_background: ground_tokens_fn(background_rgb, on_surface_rgb, primary_rgb),
        on_surface: ground_tokens_fn(surface_rgb, on_surface_rgb, primary_rgb),
        on_raised: ground_tokens_fn(high_rgb, on_surface_rgb, primary_rgb),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_ground_tokens(_: RgbColor, _: RgbColor, _: RgbColor) -> crate::tokens::GroundTokens {
        let zero = Rgba::opaque(RgbColor::new(0, 0, 0));
        crate::tokens::GroundTokens {
            border: zero,
            divider: zero,
            hover: zero,
            pressed: zero,
            selected: zero,
            text_secondary: zero,
            text_disabled: zero,
            accent_soft: zero,
            accent_strong: zero,
            border_hover: zero,
            destructive_soft: zero,
            destructive_border: zero,
            success_soft: zero,
            success_border: zero,
        }
    }

    #[test]
    fn test_material_generation() {
        let settings = MaterialSettings::default();
        let tokens = resolve_material_colors(&settings, dummy_ground_tokens);
        assert_eq!(tokens.primary.a, 255);
        assert_eq!(tokens.tertiary.a, 255);
        assert_eq!(tokens.surface_container_lowest.a, 255);
    }

    #[test]
    fn light_theme_background_uses_m3_surface_instead_of_pure_white() {
        let white = Rgba::opaque(RgbColor::new(0xff, 0xff, 0xff));
        for seed in [
            RgbColor::new(0x67, 0x50, 0xa4),
            RgbColor::new(0x19, 0x71, 0xc2),
            RgbColor::new(0x0c, 0xa6, 0x78),
            RgbColor::new(0x2f, 0x9e, 0x44),
            RgbColor::new(0xf5, 0x9f, 0x00),
            RgbColor::new(0xe0, 0x31, 0x31),
        ] {
            let settings = MaterialSettings {
                seed,
                variant: MaterialVariant::TonalSpot,
                is_dark: false,
                contrast: 0.0,
            };
            let tokens = resolve_material_colors(&settings, dummy_ground_tokens);
            // `surface_container_lowest` is Tone 100 (#ffffff) in M3 light mode,
            // while `background` uses M3 `surface` (Tone 98, tinted by the seed).
            assert_eq!(tokens.surface_container_lowest, white);
            assert_ne!(tokens.background, white);
            assert_eq!(tokens.surface, tokens.surface_container);
            assert_eq!(tokens.surface_raised, tokens.surface_container_high);
        }
    }
}
