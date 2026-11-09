//! Foliage (`pulsar.foliage`): the editor crate -- the Foliage tool mode,
//! its palette saved with the level, and the level's global wind (its
//! Create entry and World Settings section) ([`level_editor`]), over the
//! plugin's component crate. It registers nothing itself: the plugin's root
//! crate (`foliage_plugin`, with its `editor` feature) submits these through
//! the level editor's extension points. The inspector edits the components
//! through their reflected properties.

// The level editor UI's text (`locales/`).
rust_i18n::i18n!("locales", fallback = "en");

pub mod level_editor;
