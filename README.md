# Plugin_Foliage

Foliage and the level's wind: their components, rows, settings and the renderer feature that places and draws them.

`pulsar.foliage` ("Foliage") is a plugin of
[Pulsar-Native](https://github.com/Far-Beyond-Pulsar/Pulsar-Native), the engine and editor that renders with
[Helio](https://github.com/Far-Beyond-Pulsar/Helio). Pulsar-Native vendors this repository as a git submodule
at `plugins/vendor/foliage`.

## What it owns

Every object kind is a plugin. A plugin owns its SceneDB components (their
data and scriptable methods; the editor calls the same methods scripts do),
the contracts other kinds read (its `api` crate: graph keys, row ABIs, WGSL
snippets), the Helio feature that renders it (passes, scene derivations,
settings), its file formats and its editor UI. Helio knows no object kind,
and other kinds meet this one only through frame contracts, extension points,
draw phases and api crates.

| Crate | Path | What |
|---|---|---|
| `foliage_plugin` | `./` | Foliage (`pulsar.foliage`): the plugin's root crate, the one place it registers everything it contributes (runtime registrations always, the editor's with `editor`) |
| `helio-foliage-api` | `api/` | The foliage feature's public contract: GPU records, packing, placement reference maths, the ground protocol, SceneDB rows and the shared WGSL. No wgpu. |
| `foliage_component` | `component/` | Foliage (pulsar.foliage): the foliage and wind components, their rows, row join, level rule and the plugin's Helio feature set (placement and the G-buffer drawer); the plugin's root crate registers them |
| `foliage_editor` | `editor/` | Foliage (pulsar.foliage): the editor crate -- the Foliage tool mode and its palette, the level's wind entry and World Settings section, over the plugin's component crate |
| `foliage-examples` | `examples/` | The foliage plugin's standalone hosts on Helio: the foliage demo |
| `helio-pass-foliage-gbuffer` | `feature/gbuffer/` | Helio render pass: foliage blade/card rasterisation into the G-buffer |
| `helio-pass-foliage-place` | `feature/place/` | Foliage tile residency, GPU placement, tile/cluster culling and per-LOD compaction for Helio |

- **Root crate**: `foliage_plugin`. It is the one place the plugin registers
  its contributions (component classes, features, scene seams, settings,
  editor tools), each naming it as origin, so the plugin gate and the
  plugin catalog see all of them. The other crates submit none (only type
  attachments, such as a component's GPU mirror, stay with their types). A
  game links the root without its `editor` feature; the editor links it with.
- **Requires**: nothing. A project enables or disables the plugin in its
  `Pulsar.toml` (`[plugins]`); disabled, the kind is absent and the frame
  draws without it.
- **Reads other kinds through**: none.
- **Its standalone hosts also compose**: `helio-pass-gi` (gi); `helio-light-api, helio-light-host` (lights); `helio-mats` (materials); `helio-mesh-api, helio-mesh-host, helio-pass-forward-lit` (meshes); `helio-pass-sky` (sky) (demos and GPU tests
  draw a whole scene; the plugin itself does not depend on these).

## Render graph nodes

Every pass the plugin's feature adds to the default pipeline is a node of `.rendergraph` assets, registered from its feature (`registrar.node`, so only while the plugin is enabled) with generated pins, an `Enabled` parameter and the default placement below. A graph naming one while `pulsar.foliage` is disabled fails validation with the plugin named. Test: `tests/graph_node.rs`; see Pulsar-Native's `.agents/RENDER_GRAPH.md` ("Plugin nodes").

| Node kind | Title | Default placement |
|---|---|---|
| `helio.pass.foliage_place` | Foliage: Place Blades | before the G-buffer phase, order 0 |
| `helio.pass.foliage_boxes` | Foliage: Expand Boxes | before the G-buffer phase, order 5; needs the placement node |
| `helio.pass.foliage_gbuffer` | Foliage: Draw to GBuffer | G-buffer phase drawer, order 20; needs the placement node |

## Building and testing

Like [Plugin_Physics](https://github.com/Far-Beyond-Pulsar/Plugin_Physics), this plugin's crates are members
of Pulsar-Native's workspace: their dependencies are `workspace = true`, and
Helio comes from Pulsar-Native's `crates/renderer/helio` submodule. It builds
only inside a Pulsar-Native checkout:

```bash
git clone --recursive https://github.com/Far-Beyond-Pulsar/Pulsar-Native
cd Pulsar-Native
cargo check -p helio-foliage-api -p foliage_component -p foliage_plugin -p foliage_editor -p foliage-examples -p helio-pass-foliage-gbuffer -p helio-pass-foliage-place --tests
cargo test -p helio-foliage-api -p foliage_plugin -p foliage_component -p helio-pass-foliage-gbuffer -p helio-pass-foliage-place
```

Suites that render real frames need a GPU; CI leaves them out by name
(Pulsar-Native's `.config/nextest.toml`, `ci` profile). This repository's CI
checks the plugin inside a Pulsar-Native checkout: rustfmt, clippy and the
`ci` profile's tests.

A change that spans this plugin and Pulsar-Native (or Helio) merges
dependency-first: merge here, re-pin `plugins/vendor/foliage` in Pulsar-Native to
this repository's `main`, then merge the Pulsar-Native PR (Pulsar-Native's
`.agents/CRATES.md`, "Submodule pins").

## Architecture

- [Every object kind is a plugin](https://github.com/Far-Beyond-Pulsar/Helio/blob/main/docs/architecture/plugin-migration.md): the
  rule, what Helio is, the plugin table and the "As built" notes of each
  phase.
- [The layered rewrite](https://github.com/Far-Beyond-Pulsar/Helio/blob/main/docs/architecture/layered-rewrite.md): layers, buffer
  contracts, extension points and why the renderer has this shape.
- [The `helio-architecture` skill](https://github.com/Far-Beyond-Pulsar/Helio/blob/main/.agents/skills/helio-architecture/SKILL.md): required reading for any change
  here (see `AGENTS.md`).
- Pulsar-Native's `.agents/PLUGIN_SYSTEM.md` and `.agents/CRATES.md`: plugin
  descriptors, the installed set and the submodule pins.

History before 2026-11-02 (when the plugin moved into Pulsar-Native) is in
Helio's repository; this repository's history starts with that move
(`git subtree split` of `plugins/vendor/foliage`).

## License

MIT, as the rest of Pulsar (`LICENSE.md`).
