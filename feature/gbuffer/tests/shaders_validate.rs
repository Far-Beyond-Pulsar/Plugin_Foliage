//! The foliage modules validate with every snippet their passes compile them
//! with: placement, the culls (the frame's and the portal chains'), and the
//! G-buffer draw and boxes (both draws).

use helio_pass_foliage_place::{COMMON_SNIPPET, GROUND_SNIPPET, WIND_SNIPPET};
use helio_portal_api::PORTAL_CHAIN_SNIPPET;

#[test]
fn the_foliage_shaders_validate() {
    let place = [COMMON_SNIPPET, GROUND_SNIPPET];
    let cull = [COMMON_SNIPPET, GROUND_SNIPPET, PORTAL_CHAIN_SNIPPET];
    let draw = [
        WIND_SNIPPET,
        COMMON_SNIPPET,
        helio_frame_api::gbuffer::OUTPUT_SNIPPET,
        PORTAL_CHAIN_SNIPPET,
    ];
    for (name, text, snippets) in [
        (
            "foliage_place.wgsl",
            include_str!("../../place/shaders/foliage_place.wgsl"),
            &place[..],
        ),
        (
            "foliage_cull.wgsl",
            include_str!("../../place/shaders/foliage_cull.wgsl"),
            &cull[..],
        ),
        (
            "foliage_gbuffer.wgsl",
            include_str!("../shaders/foliage_gbuffer.wgsl"),
            &draw[..],
        ),
    ] {
        let source = helio_core::shader::resolve_with(text, snippets);
        let module = naga::front::wgsl::parse_str(&source)
            .unwrap_or_else(|error| panic!("{name}: {}", error.emit_to_string(&source)));
        naga::valid::Validator::new(
            naga::valid::ValidationFlags::all(),
            naga::valid::Capabilities::all(),
        )
        .validate(&module)
        .unwrap_or_else(|error| panic!("{name}: {error:?}"));
    }
}
