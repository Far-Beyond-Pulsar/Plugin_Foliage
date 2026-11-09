//! The level's global wind (#1123) in the level editor: its Create entry and
//! its World Settings section (and place in the environment mixer).
//!
//! A level has at most one: its `WindComponent` (`level_wind`, a level
//! rule of this plugin); every foliage component sways in it (unless one
//! opts out for its own wind). Without one, foliage uses the components'
//! own wind. The section shows the wind's properties when the level has one
//! and a button that creates it when it does not ([`create_wind`]). The
//! plugin's root crate registers both.

use foliage_component::WindComponent;
use ui_level_editor::LevelEditorState;
use ui_level_editor::commands::{CommandResult, TypedComponent};
use ui_level_editor::extend::add_object;
use ui_level_editor::scene_edit::{ObjectType, Transform};

/// Name of the object [`create_wind`] adds.
pub const WIND_OBJECT_NAME: &str = "Wind";

/// Add the level's global wind: an object named [`WIND_OBJECT_NAME`]
/// holding a default `WindComponent` (a light breeze), as one undoable
/// command, and select it. Refused with a message when the level already
/// has one.
pub fn create_wind(state: &mut LevelEditorState) -> CommandResult {
    add_object(
        state,
        WIND_OBJECT_NAME,
        ObjectType::Empty,
        Transform::default(),
        vec![TypedComponent::new(WindComponent::default())],
    )
}

/// The Create menu's label for [`create_wind`], in the editor's language.
pub fn wind_menu_label() -> String {
    rust_i18n::t!("LevelEditor.Create.Wind").to_string()
}

/// The World Settings section's line about the level's wind: which
/// object's `WindComponent` it is, and whether it applies.
pub fn describe_wind(object: &str, enabled: bool) -> String {
    if enabled {
        format!("The global wind is {object}'s WindComponent.")
    } else {
        format!("{object}'s WindComponent is disabled, so foliage sways in its own wind.")
    }
}
