//! Material Design 3 (M3) Dynamic Color calculation using Material Color Utilities.

use mcu_material_color::{
    ColorSpecDelegate, ColorSpecDelegateImpl2021, ColorSpecDelegateImpl2025, prelude::*,
};

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

/// The Material Design 3 color specification version.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MaterialSpecVersion {
    /// Original Material 3 (2021) tonal specification.
    Spec2021,
    /// Material 3 Expressive (2025) specification with enhanced chroma multipliers
    /// and variant-aware surface tones.
    #[default]
    Spec2025,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaterialSettings {
    pub seed: RgbColor,
    pub variant: MaterialVariant,
    pub is_dark: bool,
    /// Contrast level: -1.0 (reduced), 0.0 (standard default), 0.5 (medium), 1.0 (high).
    pub contrast: f64,
    pub spec_version: MaterialSpecVersion,
}

impl Default for MaterialSettings {
    fn default() -> Self {
        Self {
            seed: RgbColor::new(0x67, 0x50, 0xa4), // M3 baseline purple
            variant: MaterialVariant::TonalSpot,
            is_dark: true,
            contrast: 0.0,
            spec_version: MaterialSpecVersion::default(),
        }
    }
}

pub fn create_dynamic_scheme(
    hct: Hct,
    variant: MaterialVariant,
    is_dark: bool,
    contrast: f64,
    spec_version: MaterialSpecVersion,
) -> DynamicScheme {
    let mcu_variant = match variant {
        MaterialVariant::TonalSpot => Variant::TonalSpot,
        MaterialVariant::Vivid => Variant::Vibrant,
        MaterialVariant::Expressive => Variant::Expressive,
        MaterialVariant::FruitSalad => Variant::FruitSalad,
        MaterialVariant::Rainbow => Variant::Rainbow,
        MaterialVariant::Spritz => Variant::Neutral,
        MaterialVariant::Monochrome => Variant::Monochrome,
        MaterialVariant::Content => Variant::Content,
    };
    let mcu_spec = match spec_version {
        MaterialSpecVersion::Spec2021 => SpecVersion::Spec2021,
        MaterialSpecVersion::Spec2025 => SpecVersion::Spec2025,
    };
    let mut options = DynamicSchemeOptions::new(hct, mcu_variant, contrast, is_dark);
    options.platform = Some(Platform::Phone);
    options.spec_version = Some(mcu_spec);
    DynamicScheme::new(options)
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
    static SPEC_2021: ColorSpecDelegateImpl2021 = ColorSpecDelegateImpl2021;
    static SPEC_2025: ColorSpecDelegateImpl2025 = ColorSpecDelegateImpl2025;

    let seed_argb = 0xff000000
        | ((settings.seed.r as u32) << 16)
        | ((settings.seed.g as u32) << 8)
        | (settings.seed.b as u32);
    let hct = Hct::from_int(seed_argb);
    let scheme = create_dynamic_scheme(
        hct,
        settings.variant,
        settings.is_dark,
        settings.contrast,
        settings.spec_version,
    );
    let spec: &dyn ColorSpecDelegate = match scheme.spec_version {
        SpecVersion::Spec2025 => &SPEC_2025,
        SpecVersion::Spec2021 => &SPEC_2021,
    };

    let get = |color: DynamicColor| argb_to_rgba(color.get_argb(&scheme));

    let primary = get(spec.primary());
    let on_primary = get(spec.on_primary());
    let primary_container = get(spec.primary_container());
    let on_primary_container = get(spec.on_primary_container());

    let secondary = get(spec.secondary());
    let on_secondary = get(spec.on_secondary());
    let secondary_container = get(spec.secondary_container());
    let on_secondary_container = get(spec.on_secondary_container());

    let tertiary = get(spec.tertiary());
    let on_tertiary = get(spec.on_tertiary());
    let tertiary_container = get(spec.tertiary_container());
    let on_tertiary_container = get(spec.on_tertiary_container());

    let error = get(spec.error());
    let on_error = get(spec.on_error());
    let error_container = get(spec.error_container());
    let on_error_container = get(spec.on_error_container());

    let m3_surface = get(spec.surface());
    let on_surface = get(spec.on_surface());
    let on_surface_variant = get(spec.on_surface_variant());

    let surface_container_lowest = get(spec.surface_container_lowest());
    let surface_container_low = get(spec.surface_container_low());
    let surface_container = get(spec.surface_container());
    let surface_container_high = get(spec.surface_container_high());
    let surface_container_highest = get(spec.surface_container_highest());

    let outline = get(spec.outline());
    let outline_variant = get(spec.outline_variant());

    // Success color (M3 palette key or harmonized green)
    let success = Rgba::opaque(RgbColor::new(0x2e, 0x7d, 0x32));

    // Ground tokens for traditional surfaces:
    // - In light mode, `surface_container_low` (Tone 96 / Tone 95 in Vibrant, with
    //   Spec2025 chroma multipliers) serves as `background` so palette styles have
    //   distinct, perceptible tonal backgrounds while staying clearly lighter than
    //   `surface` (`surface_container`) and `surface_raised` (`surface_container_high`).
    // - In dark mode, M3 `surface` (Tone 4 in Spec2025 / Tone 6 in Spec2021) serves
    //   as `background`.
    let background = if settings.is_dark {
        m3_surface
    } else {
        surface_container_low
    };
    let background_rgb = rgba_to_rgb(background);
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
        background,
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
    fn light_theme_background_differs_across_palette_styles_in_spec2025() {
        let white = Rgba::opaque(RgbColor::new(0xff, 0xff, 0xff));
        let seed = RgbColor::new(0x67, 0x50, 0xa4);
        let bg_for = |variant: MaterialVariant| {
            resolve_material_colors(
                &MaterialSettings {
                    seed,
                    variant,
                    is_dark: false,
                    contrast: 0.0,
                    spec_version: MaterialSpecVersion::Spec2025,
                },
                dummy_ground_tokens,
            )
            .background
        };
        let tonal_spot = bg_for(MaterialVariant::TonalSpot);
        let vivid = bg_for(MaterialVariant::Vivid);
        let expressive = bg_for(MaterialVariant::Expressive);
        let spritz = bg_for(MaterialVariant::Spritz);
        let monochrome = bg_for(MaterialVariant::Monochrome);

        assert_ne!(tonal_spot, white);
        assert_ne!(tonal_spot, vivid);
        assert_ne!(tonal_spot, expressive);
        assert_ne!(tonal_spot, spritz);
        assert_ne!(tonal_spot, monochrome);
    }
}
