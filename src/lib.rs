//! Foliage (`pulsar.foliage`): the plugin's root crate, the one place the
//! plugin registers everything it contributes.
//!
//! The plugin's other crates hold the code: `helio-foliage-api` (the
//! contracts other kinds read, such as the wind rows water reads), the
//! feature crates (`helio-pass-foliage-place`: placement and its settings;
//! `helio-pass-foliage-gbuffer`: the G-buffer drawer), `foliage_component`
//! (the foliage and wind components, their rows, row join and level rule)
//! and `foliage_editor` (the level editor's Foliage mode and wind UI). None
//! of them registers a contribution; this crate imports them and submits
//! their registrations, so this file and the two modules below are the whole
//! list of what the plugin brings:
//!
//! - [`runtime`]: what a game links. The component classes, the Helio
//!   features and their settings, the scene rows, the one-wind level rule
//!   and the realtime redraw while foliage sways.
//! - `editor` (with the `editor` feature): what the editor adds. The Foliage
//!   tool mode, its palette saved with the level, and the level wind's
//!   Create entry and World Settings section.
//!
//! Every registration names this crate as its origin
//! (`pulsar_plugins::origin!()`), so the plugin gate skips all of them while
//! a project disables the plugin, and the plugin catalog lists them under
//! `pulsar.foliage`. Other plugins never depend on this crate (only on
//! `helio-foliage-api`); hosts reach it through the installed set or a
//! generated game, a game without the `editor` feature.

// The plugin's descriptor (`pulsar-plugin.toml`): linking this crate installs
// the plugin; a project enables it or not (`pulsar_plugins`).
pulsar_plugins::installed!("pulsar-plugin.toml");

pub use foliage_component;
#[cfg(feature = "editor")]
pub use foliage_editor;

pub mod runtime;

#[cfg(feature = "editor")]
pub mod editor;
