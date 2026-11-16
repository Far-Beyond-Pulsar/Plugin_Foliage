//! The foliage feature's settings: what a renderer config carries for it
//! (`RendererConfig::features`) and the node builds read, and their host
//! description ([`FOLIAGE`], as `helio_settings` describes the renderer's):
//! engines build their settings UI from it and read the values back
//! through it.

use helio_settings::{choice, integer, number, pick, toggle, Choice, Group, Resolved};

use crate::FoliageQuality;

/// How much foliage to place, and how far.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FoliageSettings {
    /// Build the foliage passes. A scene with no `foliage_types` rows pays
    /// nothing for this being on.
    pub enabled: bool,
    /// Densest ground in tufts per square metre, or `None` for the default.
    pub tufts_per_m2: Option<f32>,
    /// Blades per tuft, or `None` for the default.
    pub blades_per_tuft: Option<f32>,
    /// The quality preset: default range, density multiplier, near LOD
    /// distance scale and cull granularity.
    pub quality: FoliageQuality,
    /// Metres foliage reaches, or `None` for the quality preset's. Bands of
    /// tiles (`plan_bands`) carry it out there.
    pub range: Option<f32>,
    /// Places besides the camera's foliage keeps resident for what portals
    /// show (`helio_portal_api::PORTAL_LIVE_CHAINS`): each holds its own
    /// tile rings, about the arena's memory again. 0: foliage shows only
    /// where the camera's rings reach.
    pub portal_eyes: u32,
}

impl Default for FoliageSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            tufts_per_m2: None,
            blades_per_tuft: None,
            quality: FoliageQuality::Medium,
            range: None,
            portal_eyes: 1,
        }
    }
}

/// The quality tiers [`FoliageQuality`] maps.
const QUALITY_TIERS: &[Choice] = &[
    choice("low", "Low"),
    choice("medium", "Medium"),
    choice("high", "High"),
    choice("ultra", "Ultra"),
];

/// The foliage feature's host settings: a host lists it with its other
/// settings groups.
pub static FOLIAGE: Group = Group {
    key: "foliage",
    title: "Foliage",
    description: "Scattered foliage (grass, flowers)",
    settings: &[
        toggle(
            "enabled",
            "Foliage",
            "Foliage",
            true,
            "Draw scattered foliage (grass, flowers)",
        ),
        pick(
            "quality",
            "Foliage Quality",
            "Foliage",
            QUALITY_TIERS,
            "medium",
            "Foliage range, density and LOD distances",
        ),
        number(
            "range",
            "Foliage Range",
            "Foliage",
            (0.0, 4000.0, 10.0),
            0.0,
            "Metres foliage reaches; 0: the quality's",
        ),
        number(
            "tufts_per_m2",
            "Foliage Max Density",
            "Foliage",
            (0.0, 64.0, 0.5),
            0.0,
            "Densest foliage in tufts a square metre (denser types are thinned); 0: 16",
        ),
        number(
            "blades_per_tuft",
            "Foliage Blades per Tuft",
            "Foliage",
            (0.0, 64.0, 1.0),
            0.0,
            "Blades a tuft averages, sizing the near band; 0: 10",
        ),
        integer(
            "portal_eyes",
            "Foliage Through Portals",
            "Foliage",
            (0, 4),
            1,
            "Places a portal shows that foliage keeps resident besides the camera's, the              nearest routes first; each costs about the foliage's memory again",
        ),
    ],
};

impl FoliageSettings {
    /// The settings `settings` (of [`FOLIAGE`]) hold.
    pub fn from_settings(settings: &Resolved) -> Self {
        debug_assert!(std::ptr::eq(settings.group(), &FOLIAGE));
        Self {
            enabled: settings.bool("enabled"),
            quality: match settings.text("quality") {
                "low" => FoliageQuality::Low,
                "high" => FoliageQuality::High,
                "ultra" => FoliageQuality::Ultra,
                _ => FoliageQuality::Medium,
            },
            range: settings.optional("range").map(|m| m as f32),
            tufts_per_m2: settings.optional("tufts_per_m2").map(|n| n as f32),
            blades_per_tuft: settings.optional("blades_per_tuft").map(|n| n as f32),
            portal_eyes: settings.number("portal_eyes").clamp(0.0, 4.0) as u32,
        }
    }

    /// Metres foliage reaches: the setting, or the quality's.
    pub fn reach(&self) -> f32 {
        self.range.unwrap_or(self.quality.range())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn host_defaults_are_the_default_settings() {
        assert_eq!(
            FoliageSettings::from_settings(&FOLIAGE.defaults()),
            FoliageSettings::default()
        );
    }

    #[test]
    fn every_quality_choice_maps() {
        let setting = FOLIAGE
            .settings
            .iter()
            .find(|s| s.key == "quality")
            .expect("the foliage settings describe its quality");
        let helio_settings::Kind::Choice(options) = setting.kind else {
            panic!("quality is a choice");
        };
        for option in options {
            let settings = FOLIAGE.resolve(|key| {
                (key == setting.key).then(|| helio_settings::Value::Text(option.value.into()))
            });
            let mapped = FoliageSettings::from_settings(&settings);
            let is_default = setting.default == helio_settings::Value::Text(option.value.into());
            assert_eq!(
                mapped == FoliageSettings::default(),
                is_default,
                "{}",
                option.value
            );
        }
    }
}
