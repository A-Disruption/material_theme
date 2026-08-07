use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
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

impl Mode {
    fn tag(self) -> &'static str {
        match self {
            Mode::Light => "light",
            Mode::Dark => "dark",
        }
    }

    fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "light" => Some(Mode::Light),
            "dark" => Some(Mode::Dark),
            _ => None,
        }
    }
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

    fn from_tag(tag: &str) -> Option<Self> {
        match tag {
            "mono" => Some(Variant::Monochrome),
            "neut" => Some(Variant::Neutral),
            "tspot" => Some(Variant::TonalSpot),
            "vib" => Some(Variant::Vibrant),
            "expr" => Some(Variant::Expressive),
            "fid" => Some(Variant::Fidelity),
            "cont" => Some(Variant::Content),
            "rbow" => Some(Variant::Rainbow),
            "fruit" => Some(Variant::FruitSalad),
            _ => None,
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
        let name = theme_name(seed_argb, mode, variant);

        // An iced app calls its `theme()` callback every frame, so building the
        // M3 scheme unconditionally would redo the full HCT solve each time.
        let cached = CACHE.read().ok().and_then(|cache| cache.get(&name));
        let scheme = match cached {
            Some(scheme) => scheme,
            None => {
                let scheme = Arc::new(build_scheme(seed_argb, mode, variant));
                if let Ok(mut cache) = CACHE.write() {
                    cache.insert(name.clone(), scheme.clone());
                }
                scheme
            }
        };

        Self {
            name: Cow::Owned(name),
            scheme,
            seed_argb,
            mode,
            variant,
        }
    }
}

fn build_scheme(seed_argb: u32, mode: Mode, variant: Variant) -> MaterialScheme {
    let m3_theme = ThemeBuilder::with_source(Argb::from_u32(seed_argb))
        .variant(variant.to_m3())
        .build();

    let is_dark = matches!(mode, Mode::Dark);
    let m3_scheme = if is_dark { m3_theme.schemes.dark } else { m3_theme.schemes.light };

    MaterialScheme::from_m3(m3_scheme, is_dark)
}

fn theme_name(seed_argb: u32, mode: Mode, variant: Variant) -> String {
    format!("Material-{:08X}-{}-{}", seed_argb, variant.tag(), mode.tag())
}

/// Inverse of [`theme_name`]. The name is the only thing an [`iced::Theme`]
/// carries back to us, so it has to round-trip.
fn parse_theme_name(name: &str) -> Option<(u32, Mode, Variant)> {
    let rest = name.strip_prefix("Material-")?;
    let (seed, rest) = rest.split_once('-')?;
    let (variant, mode) = rest.rsplit_once('-')?;

    Some((
        u32::from_str_radix(seed, 16).ok()?,
        Mode::from_tag(mode)?,
        Variant::from_tag(variant)?,
    ))
}

/// Upper bound on cached schemes. Apps that drive the seed from a live color
/// picker mint a distinct theme per pointer move, so this has to be bounded or
/// it grows for as long as the process runs.
const CACHE_CAPACITY: usize = 64;

/// Insertion-ordered map from theme name to scheme, holding at most
/// [`CACHE_CAPACITY`] entries. Eviction is safe: [`MaterialThemeExt`] rebuilds
/// from the theme name on a miss.
#[derive(Default)]
struct SchemeCache {
    by_name: HashMap<String, Arc<MaterialScheme>>,
    order: VecDeque<String>,
}

impl SchemeCache {
    fn get(&self, name: &str) -> Option<Arc<MaterialScheme>> {
        self.by_name.get(name).cloned()
    }

    fn insert(&mut self, name: String, scheme: Arc<MaterialScheme>) {
        if self.by_name.insert(name.clone(), scheme).is_some() {
            return;
        }

        self.order.push_back(name);
        while self.order.len() > CACHE_CAPACITY {
            if let Some(evicted) = self.order.pop_front() {
                self.by_name.remove(&evicted);
            }
        }
    }
}

static CACHE: LazyLock<RwLock<SchemeCache>> =
    LazyLock::new(|| RwLock::new(SchemeCache::default()));

pub trait MaterialThemeExt {
    fn material_scheme(&self) -> Option<Arc<MaterialScheme>>;
}

impl MaterialThemeExt for iced::Theme {
    fn material_scheme(&self) -> Option<Arc<MaterialScheme>> {
        let iced::Theme::Custom(custom) = self else {
            return None;
        };
        let key = custom.to_string();

        if let Some(scheme) = CACHE.read().ok().and_then(|cache| cache.get(&key)) {
            return Some(scheme);
        }

        // A theme still on screen can be evicted by a burst of new seeds. Its
        // name carries everything needed to rebuild it, so recover (and re-cache
        // via `with_variant`) rather than falling back to iced's own palette.
        let (seed_argb, mode, variant) = parse_theme_name(&key)?;
        Some(MaterialTheme::with_variant(seed_argb, mode, variant).scheme)
    }
}

impl From<MaterialTheme> for iced::Theme {
    fn from(mt: MaterialTheme) -> Self {
        if let Ok(mut cache) = CACHE.write() {
            cache.insert(mt.name.to_string(), mt.scheme.clone());
        }
        iced::Theme::custom(mt.name.clone(), seed_map::to_iced_seed(&mt.scheme))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn theme_name_round_trips() {
        for variant in [
            Variant::Monochrome,
            Variant::Neutral,
            Variant::TonalSpot,
            Variant::Vibrant,
            Variant::Expressive,
            Variant::Fidelity,
            Variant::Content,
            Variant::Rainbow,
            Variant::FruitSalad,
        ] {
            for mode in [Mode::Light, Mode::Dark] {
                let name = theme_name(0xFF6750A4, mode, variant);
                assert_eq!(
                    parse_theme_name(&name),
                    Some((0xFF6750A4, mode, variant)),
                    "round-trip failed for {name}"
                );
            }
        }
    }

    #[test]
    fn foreign_theme_names_are_rejected() {
        assert_eq!(parse_theme_name("Nord"), None);
        assert_eq!(parse_theme_name("Material-XYZ-tspot-light"), None);
        assert_eq!(parse_theme_name("Material-FF6750A4-tspot-dusk"), None);
    }

    #[test]
    fn cache_stays_bounded_and_recovers_evicted_themes() {
        // More distinct seeds than the cache can hold, as a live picker would.
        for seed in 0..(CACHE_CAPACITY as u32 * 3) {
            let _ = MaterialTheme::with_variant(0xFF00_0000 | seed, Mode::Light, Variant::TonalSpot);
        }
        assert!(CACHE.read().unwrap().by_name.len() <= CACHE_CAPACITY);

        // The very first seed has been evicted by now; it must still resolve.
        let theme: iced::Theme = MaterialTheme::with_variant(
            0xFF00_0000,
            Mode::Light,
            Variant::TonalSpot,
        )
        .into();
        assert!(theme.material_scheme().is_some());
    }

    #[test]
    fn built_in_themes_have_no_scheme() {
        assert!(iced::Theme::Dark.material_scheme().is_none());
    }
}
