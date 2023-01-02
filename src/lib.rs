#![feature(stmt_expr_attributes)]

mod actions;
mod audio;
mod camera;
mod dev;
mod dialog;
mod loading;
mod map;
mod math;
mod menu;
mod physics;
mod player;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::camera::CameraPlugin;
use crate::dev::DevPlugin;
use crate::dialog::DialogPlugin;
use crate::loading::LoadingPlugin;
use crate::map::MapPlugin;
use crate::menu::MenuPlugin;
use crate::physics::PhysicsPlugin;
use crate::player::PlayerPlugin;
use bevy::app::App;
use bevy::prelude::*;

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    Loading,
    // During this State the actual game logic is executed
    Playing,
    // Here the menu is drawn and waiting for player interaction
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(PhysicsPlugin)
            .add_plugin(DialogPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(DevPlugin);
    }
}
