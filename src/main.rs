use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug, Reflect)]
pub enum PlayerAction {
    Close,
}
impl PlayerAction {
    fn mkb_input_map() -> InputMap<Self> {
        InputMap::new([(Self::Close, KeyCode::Escape)])
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<PlayerAction>::default())
        .init_resource::<ActionState<PlayerAction>>()
        .insert_resource(PlayerAction::mkb_input_map())
        .add_systems(Update, close_on_esc)
        .run();
}

fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    action_state: Res<ActionState<PlayerAction>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if action_state.just_pressed(&PlayerAction::Close) {
            commands.entity(window).despawn();
        }
    }
}