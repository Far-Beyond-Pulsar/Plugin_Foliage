//! What the foliage plugin registers for a game (and the editor): every
//! runtime contribution, grouped by the engine seam it plugs into.

use engine_backend::scene::helio_bridge::SceneRowsRegistration;
use engine_backend::scene::level_rules::LevelSingletonRegistration;
use engine_backend::scene::{ClassSpec, ComponentsBuilder};
use engine_backend::subsystems::render::{AnimatedContentRegistration, RenderFeatureRegistration};
use foliage_component::{FoliageComponent, WindComponent, WIND_CLASS_NAME};
use pulsar_settings::project::rendering::SettingsGroupRegistration;

/// Everything the plugin registers for a game (and the editor).
pub(crate) fn build(p: &mut pulsar_plugins::PluginBuilder) {
    // ── Component classes ───────────────────────────────────────────────────

    p.components()
        .class::<FoliageComponent>(ClassSpec::default())
        .class::<WindComponent>(ClassSpec::default());

    // ── Rendering ───────────────────────────────────────────────────────────

    // Placement (before the G-buffer) and the foliage drawer, configured from the
    // project's foliage settings.
    p.add(RenderFeatureRegistration {
        origin: p.origin(),
        features: foliage_component::render_features,
        configure: foliage_component::configure_renderer,
    });

    // The foliage and wind source rows and the row join that places them.
    p.add(SceneRowsRegistration {
        origin: p.origin(),
        name: "foliage.foliage_and_wind",
        register_columns: foliage_component::register_columns,
        derivation: Some(foliage_component::derivation),
    });

    // Foliage in a wind faster than calm keeps a realtime viewport drawing. The
    // answer reads both components: re-checked when either changes.
    p.add(AnimatedContentRegistration {
        origin: p.origin(),
        name: "foliage.foliage_changes",
        watch: |world| world.open_change_cursor::<FoliageComponent>(),
        animated: foliage_component::foliage_in_wind,
    });

    p.add(AnimatedContentRegistration {
        origin: p.origin(),
        name: "foliage.wind_changes",
        watch: |world| world.open_change_cursor::<WindComponent>(),
        animated: foliage_component::foliage_in_wind,
    });

    // ── Settings ────────────────────────────────────────────────────────────

    // The project's foliage settings (`helio_pass_foliage_place::FOLIAGE`).
    p.add(SettingsGroupRegistration {
        origin: p.origin(),
        group: &helio_pass_foliage_place::FOLIAGE,
    });

    // ── Level rules ─────────────────────────────────────────────────────────

    // A level has one global wind, edited in World Settings like the sky.
    p.add(LevelSingletonRegistration {
        origin: p.origin(),
        class_name: WIND_CLASS_NAME,
        refusal: foliage_component::ONE_WIND,
        instances: foliage_component::winds,
        is: |value| value.is::<WindComponent>(),
        held: |world, instance| world.get::<WindComponent>(instance).is_some(),
        default_is: true,
        enabled_only: false,
        edits_checked: false,
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_its_features_settings_and_level_rule() {
        let features =
            engine_backend::subsystems::render::helio_renderer::render_features::plugin_features();
        assert!(
            features.ids().contains(&"helio.foliage"),
            "features: {:?}",
            features.ids()
        );
        assert!(pulsar_settings::project::rendering::groups()
            .iter()
            .any(|group| std::ptr::eq(*group, &helio_pass_foliage_place::FOLIAGE)));
        assert!(pulsar_plugins::registered::<LevelSingletonRegistration>()
            .any(|rule| rule.class_name == WIND_CLASS_NAME));
    }

    #[test]
    fn registers_under_its_class_name() {
        assert_eq!(
            pulsar_world_registry::component_id_for_class("FoliageComponent"),
            Some(pulsar_scenedb::component_id::<FoliageComponent>())
        );
    }

    /// Every registration of the plugin names this root, so it resolves to
    /// the plugin's id.
    #[test]
    fn every_registration_resolves_to_the_plugin() {
        let ours: Vec<_> = pulsar_plugins::contributions()
            .into_iter()
            .filter(|summary| summary.origin == pulsar_plugins::origin!())
            .collect();
        assert!(ours
            .iter()
            .any(|summary| summary.category == "component.class"));
        for summary in &ours {
            assert_eq!(
                pulsar_plugins::plugin_of(summary.origin).map(|plugin| plugin.id.as_str()),
                Some("pulsar.foliage"),
                "{summary:?}"
            );
        }
    }
}
