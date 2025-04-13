//! Spawn the main level.

use bevy::prelude::*;

use crate::screens::Screen;

mod props;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Level>();
    app.add_plugins((props::plugin,));
}

/// A [`Command`] to spawn the level.
/// Functions that accept only `&mut World` as their parameter implement [`Command`].
/// We use this style when a command requires no configuration.
pub(crate) fn spawn_level(world: &mut World) {
    // The only thing we have in our level is a player,
    // but add things like walls etc. here.
    let asset_server = world.resource::<AssetServer>();
    world.spawn((
        Name::new("Level"),
        SceneRoot(
            //  Run ./scripts/compile_maps.sh and change .map to .bsp when you're done prototyping and want some extra performance
            asset_server.load("maps/foxtrot/foxtrot.map#Scene"),
        ),
        StateScoped(Screen::Gameplay),
        Level,
    ));
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub(crate) struct Level;
