use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::{Arc, LazyLock, RwLock};

use material_colors::color::Argb;
use material_colors::dynamic_color::Variant as M3Variant;
use material_colors::theme::ThemeBuilder;

use crate::scheme::MaterialScheme;
use crate::seed_map;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Light,
    Dark,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Variant {
    Monochrome,
    Neutral,
    TonalSpot,
    Vibrant,
    Expressive,
    Fidelity,
    Content,
    Rainbow,
    FruitSalad,
}

impl Variant {
    fn to_m3(self) -> M3Variant {
        match self {
            Variant::Monochrome => M3Variant::Monochrome,
            Variant::Neutral => M3Variant::Neutral,
            Variant::TonalSpot => M3Variant::TonalSpot,
            Variant::Vibrant => M3Variant::Vibrant,
            Variant::Expressive => M3Variant::Expressive,
            Variant::Fidelity => M3Variant::Fidelity,
            Variant::Content => M3Variant::Content,
            Variant::Rainbow => M3Variant::Rainbow,
            Variant::FruitSalad => M3Variant::FruitSalad,
        }
    }

    fn tag(self) -> &'static str {
        match self {
            Variant::Monochrome => "mono",
            Variant::Neutral => "neut",
            Variant::TonalSpot => "tspot",
            Variant::Vibrant => "vib",
            Variant::Expressive => "expr",
            Variant::Fidelity => "fid",
            Variant::Content => "cont",
            Variant::Rainbow => "rbow",
            Variant::FruitSalad => "fruit",
        }
    }
}

#[derive(Debug, Clone)]
pub struct MaterialTheme {
    pub name: Cow<'static, str>,
    pub scheme: Arc<MaterialScheme>,
    pub seed_argb: u32,
    pub mode: Mode,
    pub variant: Variant,
}

impl MaterialTheme {
    pub fn from_seed(seed_argb: u32, mode: Mode) -> Self {
        Self::with_variant(seed_argb, mode, Variant::TonalSpot)
    }

    pub fn with_variant(seed_argb: u32, mode: Mode, variant: Variant) -> Self {
        let m3_theme = ThemeBuilder::with_source(Argb::from_u32(seed_argb))
            .variant(variant.to_m3())
            .build();

        let is_dark = matches!(mode, Mode::Dark);
        let m3_scheme = if is_dark { m3_theme.schemes.dark } else { m3_theme.schemes.light };
        let scheme = Arc::new(MaterialScheme::from_m3(m3_scheme, is_dark));

        let mode_tag = if is_dark { "dark" } else { "light" };
        let name = format!("Material-{:08X}-{}-{}", seed_argb, variant.tag(), mode_tag);

        Self {
            name: Cow::Owned(name),
            scheme,
            seed_argb,
            mode,
            variant,
        }
    }
}

static REGISTRY: LazyLock<RwLock<HashMap<String, Arc<MaterialScheme>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));

pub trait MaterialThemeExt {
    fn material_scheme(&self) -> Option<Arc<MaterialScheme>>;
}

impl MaterialThemeExt for iced::Theme {
    fn material_scheme(&self) -> Option<Arc<MaterialScheme>> {
        if let iced::Theme::Custom(custom) = self {
            let key = custom.to_string();
            REGISTRY.read().ok()?.get(&key).cloned()
        } else {
            None
        }
    }
}

impl From<MaterialTheme> for iced::Theme {
    fn from(mt: MaterialTheme) -> Self {
        let key = mt.name.to_string();
        if let Ok(mut reg) = REGISTRY.write() {
            reg.entry(key).or_insert_with(|| mt.scheme.clone());
        }
        iced::Theme::custom(mt.name.clone(), seed_map::to_iced_seed(&mt.scheme))
    }
}
