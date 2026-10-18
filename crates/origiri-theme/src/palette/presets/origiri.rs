use crate::palette::{DARK_PRESET, DEFAULT_ACCENT, LIGHT_PRESET, RICE_CARD_PRESET};

use super::VariantInfo;

pub const ORIGIRI_VARIANTS: &[VariantInfo] = &[
    VariantInfo {
        id: "light",
        name: "塩むすび",
        preset: LIGHT_PRESET,
        default_accent: DEFAULT_ACCENT,
    },
    VariantInfo {
        id: "dark",
        name: "海苔むすび",
        preset: DARK_PRESET,
        default_accent: DEFAULT_ACCENT,
    },
    VariantInfo {
        id: "origiri-rice-card",
        name: "海苔むすび (白米カード)",
        preset: RICE_CARD_PRESET,
        default_accent: DEFAULT_ACCENT,
    },
];
