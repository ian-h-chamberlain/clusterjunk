use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[cfg(feature = "dev")]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod actions;
mod audio;
mod doodad;
mod level;
mod loading;
mod menu;
mod physics;
mod player;

use actions::ActionsPlugin;
use audio::InternalAudioPlugin;
use doodad::DoodadPlugin;
use level::LevelPlugin;
use loading::LoadingPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    /// During the loading State the LoadingPlugin will load our assets
    Loading,
    /// During this State the actual game logic is executed
    Playing,
    /// Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(<RapierPhysicsPlugin>::pixels_per_meter(50.0))
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(LevelPlugin)
            .add_plugin(DoodadPlugin);

        #[cfg(feature = "dev")]
        app.add_plugin(RapierDebugRenderPlugin::default())
            .add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_plugin(LogDiagnosticsPlugin::default());
    }
}
