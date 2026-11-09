//! What the foliage plugin registers for the editor (the `editor` feature):
//! the Foliage tool mode and its palette, and the level wind's Create entry
//! and World Settings section.

use foliage_editor::level_editor::{self, wind};
use ui_level_editor::extend::{
    CreateEntry, EditorData, LevelComponentSection, ToolModeRegistration,
};

// ── Foliage mode ────────────────────────────────────────────────────────────

// The Foliage tool mode: the palette of foliage sets and its brush.
inventory::submit! {
    ToolModeRegistration {
        origin: pulsar_plugins::origin!(),
        id: "foliage.mode",
        order: 30,
        make: level_editor::foliage_mode,
    }
}

// The palette, saved in the level file's editor section.
inventory::submit! {
    EditorData {
        origin: pulsar_plugins::origin!(),
        key: level_editor::FOLIAGE_SETS_KEY,
        save: level_editor::save_foliage_sets,
        load: level_editor::load_foliage_sets,
    }
}

// ── The level's wind ────────────────────────────────────────────────────────

// Create > Wind: the level's global wind.
inventory::submit! {
    CreateEntry {
        origin: pulsar_plugins::origin!(),
        id: "wind",
        menu_label: Some(wind::wind_menu_label),
        environment_label: None,
        quick_add: None,
        offered: None,
        order: 310,
        create: wind::create_wind,
    }
}

// The wind's World Settings section (and its place in the environment
// mixer): its properties, or a button that creates it.
inventory::submit! {
    LevelComponentSection {
        origin: pulsar_plugins::origin!(),
        title: "Wind",
        sidebar_title: "Global Wind",
        sidebar_missing: "Missing · creates global wind",
        order: 10,
        class_name: foliage_component::WIND_CLASS_NAME,
        find: foliage_component::level_wind,
        missing_note: "This level has no global wind: foliage sways in its own wind.",
        create_label: "Create Wind",
        create: wind::create_wind,
        describe: wind::describe_wind,
    }
}
