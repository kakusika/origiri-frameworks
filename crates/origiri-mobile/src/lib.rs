//! Generic, app-agnostic mobile Slint widgets (`ui/tokens.slint`,
//! `ui/icons.slint`, `ui/components.slint`), kept independent of any one
//! consuming app's own domain-specific components. Mirrors `origiri`'s
//! `links` + `cargo:UI_DIR=...` build-script convention and `ui/` file
//! split.

/// Pushes every color of an `origiri_theme::ColorTokens` onto an app's
/// generated `ThemeColors` global and flips `Theme.use-custom` to `true`
/// in the same call -- unlike `origiri::push_color_tokens!`, which only
/// pushes fields (desktop's `Tokens` global has no system/custom switch
/// to flip), this one also owns the switch so a caller can't push colors
/// and forget to flip it, leaving the UI silently still on OS colors.
/// `warning` has no `origiri-theme` equivalent (its own `destructive`/
/// `success` are fixed constants too) and is deliberately left out --
/// `Colors.warning` always reads from `SystemColors`, never themed.
///
/// Expands at the call site:
///
/// ```ignore
/// origiri_mobile::push_mobile_color_tokens!(
///     app.global::<ThemeColors>(), app.global::<Theme>(), theme.colors, color
/// );
/// ```
///
/// where `color` turns an `origiri_theme::Rgba` into a `slint::Color`.
/// Keep the field list in step with `ui/tokens.slint`'s `ThemeColors`.
#[macro_export]
macro_rules! push_mobile_color_tokens {
    ($theme_colors:expr, $theme_flag:expr, $colors:expr, $color:expr) => {{
        let tokens = $theme_colors;
        let c = &$colors;
        tokens.set_danger($color(c.destructive));
        tokens.set_success($color(c.success));
        tokens.set_separator($color(c.on_background.divider));
        tokens.set_muted($color(c.on_background.text_secondary));
        tokens.set_page_background($color(c.background));
        tokens.set_surface($color(c.surface));
        tokens.set_surface_elevated($color(c.surface_raised));
        tokens.set_card_border($color(c.on_raised.border));

        // M3 Semantic Roles
        tokens.set_primary($color(c.primary));
        tokens.set_on_primary($color(c.on_primary));
        tokens.set_primary_container($color(c.primary_container));
        tokens.set_on_primary_container($color(c.on_primary_container));

        tokens.set_secondary($color(c.secondary));
        tokens.set_on_secondary($color(c.on_secondary));
        tokens.set_secondary_container($color(c.secondary_container));
        tokens.set_on_secondary_container($color(c.on_secondary_container));

        tokens.set_tertiary($color(c.tertiary));
        tokens.set_on_tertiary($color(c.on_tertiary));
        tokens.set_tertiary_container($color(c.tertiary_container));
        tokens.set_on_tertiary_container($color(c.on_tertiary_container));

        tokens.set_error($color(c.error));
        tokens.set_on_error($color(c.on_error));
        tokens.set_error_container($color(c.error_container));
        tokens.set_on_error_container($color(c.on_error_container));

        tokens.set_surface_container_lowest($color(c.surface_container_lowest));
        tokens.set_surface_container_low($color(c.surface_container_low));
        tokens.set_surface_container($color(c.surface_container));
        tokens.set_surface_container_high($color(c.surface_container_high));
        tokens.set_surface_container_highest($color(c.surface_container_highest));

        tokens.set_outline($color(c.outline));
        tokens.set_outline_variant($color(c.outline_variant));

        $theme_flag.set_use_custom(true);
    }};
}
