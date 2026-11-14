//! Foliage's frame statistics (`helio_core::stats`). The placement's finalize
//! lane adds each LOD's draw to Helio's shared `draw_calls`, `vertices` and
//! `instances`, and its blades to [`BLADES_DRAWN`].

/// Blades (tuft instances) the foliage draws this frame, over every LOD.
pub const BLADES_DRAWN: &str = "foliage.blades_drawn";
